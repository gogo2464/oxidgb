extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

mod core;

use std::path::Path;

use core::rom::GameROM;
use core::mem::GBMemory;
use core::cpu::CPU;
use core::gpu::PITCH;

fn main() {
    println!("Oxidgb v0.1");

    // Load game ROM
    let rom = GameROM::build(Path::new("jp.gb"));

    // Build memory
    let memory = GBMemory::build(rom);

    // Build CPU
    let mut cpu = CPU::build(memory);

    println!("Opening ROM: {}", cpu.mem.rom.get_name());
    println!("Mapper type: {:?}", cpu.mem.rom.get_cart_type());

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

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB888, 160, 144).unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // The rest of the game loop goes here...
        cpu.run();

        texture.update(None, &cpu.mem.gpu.pixel_data, 160 * PITCH);

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }
}
