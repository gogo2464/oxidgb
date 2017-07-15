extern crate clap;
extern crate sdl2;
extern crate nfd;
extern crate rustyline;

#[macro_use]
extern crate log;

mod core;
mod logging;
mod debugger;

use clap::App;
use clap::Arg;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::PixelMasks;
use sdl2::keyboard::Keycode;

use nfd::Response;

use std::{thread, time};
use std::time::Duration;
use std::path::Path;
use std::process::exit;

use core::input::GameboyButton;
use core::rom::GameROM;
use core::mem::GBMemory;
use core::cpu::CPU;
use core::gpu::PITCH;

use debugger::CommandLineDebugger;

fn main() {
    // Parse arguments
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
            .help("Enables debugging"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Enables verbose logging"));

    let args = app.get_matches();

    let enable_debugging = args.is_present("debug");
    let enable_verbose = args.is_present("verbose");

    // Set up logger
    logging::setup_logging(enable_verbose).unwrap();

    info!("Oxidgb v0.1");

    let file = match args.value_of("load") {
        Some(data) => data.to_string(),
        None       => {
            // Open a file dialog
            match nfd::open_file_dialog(Some("gb"), None).unwrap() {
                Response::Okay(file_path) => file_path,
                _ => {
                    error!("No file selected.");
                    exit(2);
                },
            }
        }
    };

    let rom_path = Path::new(&file);
    if !rom_path.exists() {
        error!("Specified file does not exist.");
        exit(2);
    }

    // Load game ROM
    let rom = GameROM::build(rom_path);

    // Build memory
    let memory = GBMemory::build(rom);

    let mut debugger = CommandLineDebugger::build();

    // Build CPU
    let mut cpu = CPU::build(memory);

    info!("Opening ROM: {}", cpu.mem.rom.name);
    debug!("Mapper type: {:?}", cpu.mem.rom.cart_type);

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

    // Build a texture with a proper RGB888 implementation
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
        let mut gb_buttons = Vec::new();
        let mut fast_forward = false;

        for x in event_pump.keyboard_state().pressed_scancodes()
            .filter_map(Keycode::from_scancode) {
            let result = match x {
                Keycode::Up    => GameboyButton::UP,
                Keycode::Down  => GameboyButton::DOWN,
                Keycode::Left  => GameboyButton::LEFT,
                Keycode::Right => GameboyButton::RIGHT,
                Keycode::X     => GameboyButton::A,
                Keycode::Z     => GameboyButton::B,
                Keycode::A     => GameboyButton::SELECT,
                Keycode::S     => GameboyButton::START,
                Keycode::Tab   => {
                    fast_forward = true;
                    continue
                },
                _              => continue
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
        if elapsed < max_frame && !fast_forward {
            let sleep_time = max_frame - elapsed;

            thread::sleep(sleep_time);
        }
    }
}
