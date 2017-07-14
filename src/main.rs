extern crate clap;
extern crate sdl2;
extern crate nfd;
extern crate rustyline;

mod core;

use clap::App;
use clap::Arg;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::PixelMasks;
use sdl2::keyboard::Keycode;

use nfd::Response;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::{thread, time};
use std::time::Duration;
use std::path::Path;
use std::process::exit;

use core::input::GameboyButton;
use core::rom::GameROM;
use core::mem::GBMemory;
use core::cpu::CPU;
use core::cpu::GameboyDebugger;
use core::gpu::PITCH;

fn main() {
    let app = App::new("Oxidgb")
        .about("A experimental Gameboy emulator")
        .version("v0.1")
        .arg(Arg::with_name("load")
            .short("l")
            .long("load")
            .value_name("FILE")
            .help("Loads the specified ROM")
            .takes_value(true))
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Enables debugging"));

    let args = app.get_matches();

    let enable_debugging = args.is_present("debug");

    println!("Oxidgb v0.1");

    let file = match args.value_of("load") {
        Some(data) => data.to_string(),
        None       => {
            // Open a file dialog
            match nfd::open_file_dialog(Some("gb"), None).unwrap() {
                Response::Okay(file_path) => file_path,
                _ => {
                    println!("No file selected.");
                    exit(2);
                },
            }
        }
    };

    let rom_path = Path::new(&file);
    if !rom_path.exists() {
        println!("Specified file does not exist.");
        exit(2);
    }

    // Load game ROM
    let rom = GameROM::build(rom_path);

    // Build memory
    let memory = GBMemory::build(rom);

    let mut debugger = CommandLineDebugger {
        enabled : true,
        shutdown : false,
        breakpoints : Vec::new(),
        editor : Editor::<()>::new()
    };

    // Build CPU
    let mut cpu = CPU::build(memory);

    println!("Opening ROM: {}", cpu.mem.rom.name);
    println!("Mapper type: {:?}", cpu.mem.rom.cart_type);

    // Build a window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Oxidgb", 160 * 2, 144 * 2)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mask = PixelMasks {
        bpp : 8 * 3,
        rmask : 0x0000FF,
        gmask : 0x00FF00,
        bmask : 0xFF0000,
        amask : 0
    };

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::from_masks(mask), 160, 144).unwrap();

    canvas.set_draw_color(pixels::Color::RGB(cpu.mem.gpu.palette[0],
                                             cpu.mem.gpu.palette[1],
                                             cpu.mem.gpu.palette[2]));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        let start_loop = time::Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        if debugger.shutdown {
            break 'running;
        }

        // Update input
        let keys : Vec<sdl2::keyboard::Keycode> = event_pump.keyboard_state().pressed_scancodes()
                                                    .filter_map(Keycode::from_scancode).collect();
        let mut gb_buttons = Vec::new();

        for x in &keys {
            let result = match x {
                &Keycode::Up    => GameboyButton::UP,
                &Keycode::Down  => GameboyButton::DOWN,
                &Keycode::Left  => GameboyButton::LEFT,
                &Keycode::Right => GameboyButton::RIGHT,
                &Keycode::X     => GameboyButton::A,
                &Keycode::Z     => GameboyButton::B,
                &Keycode::A     => GameboyButton::SELECT,
                &Keycode::S     => GameboyButton::START,
                _               => continue
            };
            gb_buttons.push(result);
        }

        cpu.mem.set_input(&gb_buttons);

        if enable_debugging {
            cpu.run(&mut Some(&mut debugger));
        } else {
            cpu.run(&mut None);
        }

        if cpu.mem.gpu.is_enabled() {
            texture.update(None, &cpu.mem.gpu.pixel_data, 160 * PITCH).unwrap();

            canvas.clear();
            canvas.copy(&texture, None, None).unwrap();
        }

        canvas.present();

        let max_frame = Duration::from_millis(16);
        let elapsed = start_loop.elapsed();
        if elapsed < max_frame {
            let sleep_time = max_frame - elapsed;

            thread::sleep(sleep_time);
        }
    }
}

struct CommandLineDebugger {
    enabled : bool,
    shutdown : bool,
    breakpoints : Vec<u16>,
    editor : Editor<()>
}

impl GameboyDebugger for CommandLineDebugger {
    fn debug(&mut self, cpu : &mut CPU) {
        // Check to see if we have any breakpoints
        if self.breakpoints.contains(&cpu.regs.pc) {
            println!("Hit breakpoint: {:04X}", cpu.regs.pc);
            self.enabled = true;
        }

        if !self.enabled {
            return
        }

        let mut instruction = cpu.mem.read(cpu.regs.pc) as u16;

        if instruction == 0xCB {
            instruction = (instruction << 8) | cpu.mem.read(cpu.regs.pc + 1) as u16;
        }

        println!("{:04X} @ {:04X}", instruction, cpu.regs.pc);

        loop {
            println!("Debugger:");
            match self.editor.readline("") {
                Ok(line) => {
                    let mut args = line.trim().split(" ");

                    match args.nth(0) {
                        Some("") => {
                            println!("Stepping...");
                            break;
                        }
                        Some("r") |
                        Some("run") => {
                            println!("Running...");
                            self.enabled = false;
                            break;
                        },
                        Some("i") |
                        Some("regs") => {
                            println!("af = {:04X}", cpu.regs.get_af());
                            println!("bc = {:04X}", cpu.regs.get_bc());
                            println!("de = {:04X}", cpu.regs.get_de());
                            println!("hl = {:04X}", cpu.regs.get_hl());
                            println!("sp = {:04X}", cpu.regs.sp);
                            println!("pc = {:04X}", cpu.regs.pc);
                        },
                        Some("mem") => {
                            match args.nth(0) {
                                Some(value) => match u16::from_str_radix(&value, 16) {
                                    Ok(number) => {
                                        println!("{:04X} = {:02X}", number, cpu.mem.read(number));
                                    },
                                    _ => {
                                        println!("Invalid number.");
                                    }
                                },
                                _ => {
                                    println!("Requires an argument.");
                                }
                            }
                        },
                        Some("mems") => {
                            match args.nth(0) {
                                Some(value) => match u16::from_str_radix(&value, 16) {
                                    Ok(number) => {
                                        for i in 0 .. 0xF {
                                            print!("{:04X} = {:02X} ", number + i, cpu.mem.read(number + i));
                                        }
                                        println!();
                                    },
                                    _ => {
                                        println!("Invalid number.");
                                    }
                                },
                                _ => {
                                    println!("Requires an argument.");
                                }
                            }
                        },
                        Some("break") => {
                            match args.nth(0) {
                                Some(value) => match u16::from_str_radix(&value, 16) {
                                    Ok(number) => {
                                        match self.breakpoints.iter().position(|&r| r == number) {
                                            Some(position) => {
                                                println!("Removing breakpoint: {:04X}", number);
                                                self.breakpoints.remove(position);
                                            },
                                            _ => {
                                                println!("Adding breakpoint: {:04X}", number);
                                                self.breakpoints.push(number);
                                            }
                                        }
                                    },
                                    _ => {
                                        println!("Invalid number.");
                                    }
                                },
                                _ => {
                                    println!("Requires an argument.");
                                }
                            }
                        },
                        None => {}
                        _ => {
                            println!("Unknown command.");
                        }
                    }
                },
                Err(ReadlineError::Interrupted) |
                Err(ReadlineError::Eof) => {
                    println!("Closing...");
                    self.shutdown = true;
                    self.enabled = false;
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    self.shutdown = true;
                    self.enabled = false;
                    break
                }
            }
        }
    }
}
