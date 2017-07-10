extern crate sdl2;
extern crate nfd;

mod core;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::PixelMasks;
use sdl2::keyboard::Keycode;

use nfd::Response;

use std::{thread, time};
use std::time::Duration;
use std::path::Path;

use core::input::GameboyButton;
use core::rom::GameROM;
use core::mem::GBMemory;
use core::cpu::CPU;
use core::gpu::PITCH;

fn main() {
    println!("Oxidgb v0.1");

    // TODO: Commandline arguments parser
    let result = match nfd::open_file_dialog(None, None).unwrap() {
        Response::Okay(file_path) => file_path,
        _ => {
            println!("No file selected.");
            return;
        },
    };

    // Load game ROM
    let rom = GameROM::build(Path::new(&result));

    // Build memory
    let memory = GBMemory::build(rom);

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
                _              => continue
            };
            gb_buttons.push(result);
        }

        cpu.mem.set_input(&gb_buttons);

        // The rest of the game loop goes here...
        cpu.run();

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
