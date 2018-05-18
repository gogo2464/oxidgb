/**
 * main.rs
 *
 * The main entry-point for the Glutin frontend
**/

extern crate gl;
extern crate glutin;
extern crate libc;

extern crate clap;
extern crate nfd;
extern crate rustyline;

#[macro_use]
extern crate log;

extern crate oxidgb_core;

extern crate rodio;

mod logging;
mod debugger;

use std::ffi::CStr;
use std::ptr;
use std::mem;

use clap::App;
use clap::Arg;

/*use sdl2::event::Event;
use sdl2::pixels;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::PixelMasks;
use sdl2::keyboard::Keycode;*/

use glutin::GlContext;

use nfd::Response;

use std::{thread, time};
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::time::Duration;
use std::path::Path;
use std::process::exit;

use oxidgb_core::input::GameboyButton;
use oxidgb_core::rom::GameROM;
use oxidgb_core::rom::get_rom_size;
use oxidgb_core::mem::GBMemory;
use oxidgb_core::cpu::CPU;

use debugger::CommandLineDebugger;

use rodio::Sink;
use rodio::queue::queue;
use rodio::buffer::SamplesBuffer;

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

    let file_size = match fs::metadata(rom_path) {
        Err(why) => panic!("couldn't read metadata of {}: {}", rom_path.display(),
                           why.description()),
        Ok(meta) => meta.len()
    } as usize;

    let mut data = Vec::with_capacity(file_size);

    let mut file = match File::open(&rom_path) {
        Err(why) => panic!("couldn't open {}: {}", rom_path.display(),
                           why.description()),
        Ok(file) => file,
    };

    let read = file.read_to_end(&mut data).unwrap();

    assert_eq!(read, file_size);
    let rom_size = get_rom_size(data[0x148]);

    if rom_size != file_size {
        warn!("File size is not equal to what ROM declares!");
    }

    let rom = GameROM::build(data);

    // Build memory
    let memory = GBMemory::build(rom);

    let mut debugger = CommandLineDebugger::build();

    // Build CPU
    let mut cpu = CPU::build(memory);

    info!("Opening ROM: {}", cpu.mem.rom.name);
    debug!("Mapper type: {:?}", cpu.mem.rom.cart_type);

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Oxidgb")
        .with_dimensions(160 * 2, 144 * 2);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window,
                                          context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // Prepare OpenGL
    let version = unsafe {
        let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
        String::from_utf8(data).unwrap()
    };

    info!("OpenGL version: {}", version);

    let mut tex = unsafe { mem::uninitialized() };
    let mut ebo = unsafe { mem::uninitialized() };

    unsafe {
        // Generate shaders
        // Stolen from https://github.com/tomaka/glutin/blob/master/examples/support/mod.rs &
        //             https://open.gl/content/code/c3_multitexture.txt
        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::CompileShader(vs);

        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::CompileShader(fs);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::BindFragDataLocation(program, 0, b"outColor\0".as_ptr() as *const _);
        gl::LinkProgram(program);
        gl::UseProgram(program);

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (ELEMENTS.len() * mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                       ELEMENTS.as_ptr() as *const _, gl::STATIC_DRAW);

        let mut vb = mem::uninitialized();
        gl::GenBuffers(1, &mut vb);
        gl::BindBuffer(gl::ARRAY_BUFFER, vb);
        gl::BufferData(gl::ARRAY_BUFFER,
                      (VERTEX_DATA.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                      VERTEX_DATA.as_ptr() as *const _, gl::STATIC_DRAW);

        let mut vao = mem::uninitialized();
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let pos_attrib = gl::GetAttribLocation(program, b"position\0".as_ptr() as *const _);
        let color_attrib = gl::GetAttribLocation(program, b"color\0".as_ptr() as *const _);
        let tex_attrib = gl::GetAttribLocation(program, b"texcoord\0".as_ptr() as *const _);
        gl::VertexAttribPointer(pos_attrib as gl::types::GLuint, 2, gl::FLOAT, 0,
                               7 * mem::size_of::<f32>() as gl::types::GLsizei,
                               ptr::null());
        gl::VertexAttribPointer(color_attrib as gl::types::GLuint, 3, gl::FLOAT, 0,
                               7 * mem::size_of::<f32>() as gl::types::GLsizei,
                               (2 * mem::size_of::<f32>()) as *const () as *const _);
        gl::VertexAttribPointer(tex_attrib as gl::types::GLuint, 2, gl::FLOAT, 0,
                                7 * mem::size_of::<f32>() as gl::types::GLsizei,
                                (5 * mem::size_of::<f32>()) as *const () as *const _);
        gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
        gl::EnableVertexAttribArray(color_attrib as gl::types::GLuint);
        gl::EnableVertexAttribArray(tex_attrib as gl::types::GLuint);

        // Generate texture (for us to dump into)
        gl::GenTextures(1, &mut tex);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex);

        gl::Uniform1i(gl::GetUniformLocation(program, b"tex\0".as_ptr() as *const _), 0);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::types::GLint);
    }

    // Init audio
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    let (audio_input, audio_output) = queue(true);
    sink.append(audio_output);
    sink.play();

    let mut running = true;

    // Update input
    let mut gb_buttons = Vec::new();
    let mut fast_forward = false;

    let mut last_synced = time::Instant::now();

    while running {
        let start_loop = time::Instant::now();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        match input.virtual_keycode {
                            Some(key) => {
                                let key = match key {
                                    glutin::VirtualKeyCode::Up => GameboyButton::UP,
                                    glutin::VirtualKeyCode::Down => GameboyButton::DOWN,
                                    glutin::VirtualKeyCode::Left => GameboyButton::LEFT,
                                    glutin::VirtualKeyCode::Right => GameboyButton::RIGHT,
                                    glutin::VirtualKeyCode::X => GameboyButton::A,
                                    glutin::VirtualKeyCode::Z => GameboyButton::B,
                                    glutin::VirtualKeyCode::A => GameboyButton::SELECT,
                                    glutin::VirtualKeyCode::S => GameboyButton::START,
                                    glutin::VirtualKeyCode::Tab => {
                                        match input.state {
                                            glutin::ElementState::Pressed => {
                                                fast_forward = true;
                                            },
                                            _ => {
                                                fast_forward = false;
                                            }
                                        }
                                        return;
                                    },
                                    _ => {
                                        return;
                                    }
                                };

                                match input.state {
                                    glutin::ElementState::Pressed => {
                                        if !gb_buttons.contains(&key) {
                                            gb_buttons.push(key);
                                        }
                                    },
                                    glutin::ElementState::Released => {
                                        match gb_buttons.iter().position(|x| *x == key) {
                                            Some(pos) => {
                                                gb_buttons.remove(pos);
                                            },
                                            None => {}
                                        }
                                    }
                                }
                            },
                            None => {}
                        }
                    },
                    _ => ()
                },

                _ => ()
            }
        });

        cpu.mem.set_input(&gb_buttons);

        if enable_debugging {
            cpu.run(&mut Some(&mut debugger));
        } else {
            cpu.run(&mut None);
        }

        let max_frame = Duration::from_millis(16);

        if !fast_forward || last_synced.elapsed() > max_frame {
            if cpu.mem.gpu.is_enabled() {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, tex);
                    gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as gl::types::GLint, 160, 144, 0,
                                   gl::RGB, gl::UNSIGNED_BYTE,
                                   cpu.mem.gpu.pixel_data.as_ptr() as *const _);

                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT,
                                     (0 * mem::size_of::<f32>()) as *const () as *const _);
                }
            }

            gl_window.swap_buffers().unwrap();
            last_synced = time::Instant::now();
        }

        let elapsed = start_loop.elapsed();
        if elapsed < max_frame && !fast_forward {
            let sleep_time = max_frame - elapsed;

            thread::sleep(sleep_time);
        }

        // Handle audio
        let (samples, sample_count) = cpu.mem.sound.take_samples();
        let sample_buffer = SamplesBuffer::new(2,
                                               oxidgb_core::sound::OUTPUT_FREQUENCY,
                                               &samples[0 .. sample_count]);
        audio_input.append(sample_buffer);
    }
}

// OpenGL resources
static VERTEX_DATA: [f32; 28] = [
    // X    Y    R    G    B    U    V
    -1.0,  1.0, 1.0, 1.0, 1.0, 0.0, 0.0, // Top-left
     1.0,  1.0, 1.0, 1.0, 1.0, 1.0, 0.0, // Top-right
     1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, // Bottom-right
    -1.0, -1.0, 1.0, 1.0, 1.0, 0.0, 1.0  // Bottom-left
];

static ELEMENTS: [u32; 6] = [
    0, 1, 2,
    2, 3, 0
];

const VS_SRC: &'static [u8] = b"
    #version 150 core

    in vec2 position;
    in vec3 color;
    in vec2 texcoord;

    out vec3 Color;
    out vec2 Texcoord;

    void main()
    {
        Color = color;
        Texcoord = texcoord;
        gl_Position = vec4(position, 0.0, 1.0);
    }
\0";

const FS_SRC: &'static [u8] = b"
    #version 150 core

    in vec3 Color;
    in vec2 Texcoord;

    out vec4 outColor;

    uniform sampler2D tex;

    void main()
    {
        outColor = texture(tex, Texcoord) * vec4(Color, 1.0);
    }
\0";