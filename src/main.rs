extern crate gl;
extern crate glutin;
extern crate libc;

mod core;

use std::path::Path;

use core::rom::GameROM;
use core::mem::GBMemory;
use core::cpu::CPU;

fn main() {
    //println!("Oxidgb v0.1");

    // Load game ROM
    let rom = GameROM::build(Path::new("jp.gb"));

    // Build memory
    let memory = GBMemory::build(rom);

    // Build CPU
    let mut cpu = CPU::build(memory);


    //println!("Opening ROM: {}", cpu.mem.rom.get_name());
    //println!("Mapper type: {:?}", cpu.mem.rom.get_cart_type());

    loop {
        cpu.tick()
    }

    // Create window for rendering
    /*let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!".to_string())
        .with_dimensions(160, 144)
        .with_vsync()
        .build(&events_loop)
        .unwrap();

    unsafe {
        window.make_current()
    }.unwrap();

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event: glutin::WindowEvent::Closed, .. } => {
                    running = false;
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers().unwrap();
    }*/
}
