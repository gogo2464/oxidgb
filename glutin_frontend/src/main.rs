/**
 * main.rs
 *
 * The main entry-point for the Glutin frontend
**/
extern crate gl;
extern crate glutin;
extern crate libc;

extern crate clap;
extern crate native_dialog;
#[cfg(feature = "debugger")]
extern crate rustyline;

#[macro_use]
extern crate log;

extern crate oxidgb_core;

#[cfg(feature = "enable_sound")]
extern crate rodio;

#[cfg(feature = "debugger")]
mod debugger;
mod logging;

use std::ffi::CStr;
use std::mem;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::ptr;
use std::time::Duration;
use std::time::Instant;

use clap::App;
use clap::Arg;

use oxidgb_core::cpu::CPU;
use oxidgb_core::input::GameboyButton;
use oxidgb_core::mem::GBMemory;
use oxidgb_core::rom::get_rom_size;
use oxidgb_core::rom::GameROM;

#[cfg(feature = "debugger")]
use debugger::CommandLineDebugger;

#[cfg(feature = "enable_sound")]
use rodio::buffer::SamplesBuffer;
#[cfg(feature = "enable_sound")]
use rodio::queue::queue;
#[cfg(feature = "enable_sound")]
use rodio::Sink;

use glutin::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
#[cfg(target_family = "windows")]
use glutin::platform::windows::WindowBuilderExtWindows;

fn main() {
    // Parse arguments
    let app = App::new("Oxidgb")
        .about("A experimental Gameboy emulator")
        .version("v0.1")
        .arg(
            Arg::with_name("load")
                .short("l")
                .long("load")
                .value_name("FILE")
                .help("Loads the specified ROM")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enables verbose logging"),
        );

    let args = app.get_matches();

    let enable_verbose = args.is_present("verbose");

    // Set up logger
    logging::setup_logging(enable_verbose).expect("Failed to setup logger");

    info!("Oxidgb v0.1");

    let file = match args.value_of("load") {
        Some(data) => PathBuf::from(data),
        None => {
            // Open a file dialog
            let file_result = native_dialog::FileDialog::new()
                .add_filter("Gameboy ROM", &["gb"])
                .show_open_single_file()
                .expect("Failed to select file");

            match file_result {
                Some(file) => file,
                None => {
                    error!("No file selected.");
                    exit(2);
                }
            }
        }
    };

    let rom_path = Path::new(&file);
    if !rom_path.exists() {
        error!("Specified file does not exist.");
        exit(2);
    }

    // Load game ROM
    let data = std::fs::read(&rom_path).expect("Failed to read ROM");

    let rom_size = get_rom_size(data[0x148]);

    if rom_size != data.len() {
        warn!("File size is not equal to what ROM declares!");
    }

    let rom = GameROM::build(data);

    // Build memory
    let memory = GBMemory::build(rom);

    #[cfg(feature = "debugger")]
    let mut debugger = CommandLineDebugger::build();

    // Build CPU
    let mut cpu = CPU::build(memory);

    info!("Opening ROM: {}", cpu.mem.rom.get_cart_name());
    debug!("Mapper type: {:?}", cpu.mem.rom.cart_type);
    info!(
        "Emulation state memory usage: {}",
        std::mem::size_of_val(&cpu)
    );

    // Init audio
    #[cfg(feature = "enable_sound")]
    let device = rodio::default_output_device().unwrap();
    #[cfg(feature = "enable_sound")]
    let sink = Sink::new(&device);

    #[cfg(feature = "enable_sound")]
    let audio_input = {
        let (audio_input, audio_output) = queue(true);
        sink.append(audio_output);
        sink.play();
        audio_input
    };

    let events_loop = glutin::event_loop::EventLoop::new();
    let mut window = glutin::window::WindowBuilder::new()
        .with_title("Oxidgb")
        .with_inner_size(glutin::dpi::LogicalSize::new(160 * 2, 144 * 2));

    // Work around COM support issue with audio
    #[cfg(target_family = "windows")]
    {
        window = window.with_drag_and_drop(false);
    }

    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window, &events_loop)
        .expect("Failed to build context");

    let context = unsafe {
        context
            .make_current()
            .expect("Failed to set context as current")
    };

    unsafe {
        gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // Prepare OpenGL
    let version = unsafe {
        CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_str()
            .expect("Failed to convert to string")
    };

    info!("OpenGL version: {}", version);

    let mut tex = MaybeUninit::uninit();
    let mut ebo = MaybeUninit::uninit();

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

        gl::GenBuffers(1, ebo.as_mut_ptr());
        gl::BindBuffer(gl::ARRAY_BUFFER, ebo.assume_init());
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (ELEMENTS.len() * mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            ELEMENTS.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let mut vb = MaybeUninit::uninit();
        gl::GenBuffers(1, vb.as_mut_ptr());
        gl::BindBuffer(gl::ARRAY_BUFFER, vb.assume_init());
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let mut vao = MaybeUninit::uninit();
        gl::GenVertexArrays(1, vao.as_mut_ptr());
        gl::BindVertexArray(vao.assume_init());

        let pos_attrib = gl::GetAttribLocation(program, b"position\0".as_ptr() as *const _);
        let color_attrib = gl::GetAttribLocation(program, b"color\0".as_ptr() as *const _);
        let tex_attrib = gl::GetAttribLocation(program, b"texcoord\0".as_ptr() as *const _);
        gl::VertexAttribPointer(
            pos_attrib as gl::types::GLuint,
            2,
            gl::FLOAT,
            0,
            7 * mem::size_of::<f32>() as gl::types::GLsizei,
            ptr::null(),
        );
        gl::VertexAttribPointer(
            color_attrib as gl::types::GLuint,
            3,
            gl::FLOAT,
            0,
            7 * mem::size_of::<f32>() as gl::types::GLsizei,
            (2 * mem::size_of::<f32>()) as *const () as *const _,
        );
        gl::VertexAttribPointer(
            tex_attrib as gl::types::GLuint,
            2,
            gl::FLOAT,
            0,
            7 * mem::size_of::<f32>() as gl::types::GLsizei,
            (5 * mem::size_of::<f32>()) as *const () as *const _,
        );
        gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
        gl::EnableVertexAttribArray(color_attrib as gl::types::GLuint);
        gl::EnableVertexAttribArray(tex_attrib as gl::types::GLuint);

        // Generate texture (for us to dump into)
        gl::GenTextures(1, tex.as_mut_ptr());
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex.assume_init());

        gl::Uniform1i(
            gl::GetUniformLocation(program, b"tex\0".as_ptr() as *const _),
            0,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_EDGE as gl::types::GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_EDGE as gl::types::GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST as gl::types::GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as gl::types::GLint,
        );
    }

    // Update input
    let mut gb_buttons = Vec::new();
    let mut fast_forward = false;

    let mut loop_time_remaining = Instant::now();
    let mut update_submitted = false;

    events_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let max_frame = Duration::from_nanos(16742706);

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(key) => {
                        let key = match key {
                            VirtualKeyCode::Up => GameboyButton::UP,
                            VirtualKeyCode::Down => GameboyButton::DOWN,
                            VirtualKeyCode::Left => GameboyButton::LEFT,
                            VirtualKeyCode::Right => GameboyButton::RIGHT,
                            VirtualKeyCode::X => GameboyButton::A,
                            VirtualKeyCode::Z => GameboyButton::B,
                            VirtualKeyCode::A => GameboyButton::SELECT,
                            VirtualKeyCode::S => GameboyButton::START,
                            VirtualKeyCode::Tab => {
                                match input.state {
                                    ElementState::Pressed => {
                                        fast_forward = true;
                                    }
                                    _ => {
                                        fast_forward = false;
                                        // Reset the loop time
                                        loop_time_remaining = Instant::now();
                                    }
                                }
                                return;
                            }
                            _ => {
                                return;
                            }
                        };

                        match input.state {
                            ElementState::Pressed => {
                                if !gb_buttons.contains(&key) {
                                    gb_buttons.push(key);
                                }
                            }
                            ElementState::Released => {
                                match gb_buttons.iter().position(|x| *x == key) {
                                    Some(pos) => {
                                        gb_buttons.remove(pos);
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                    None => {}
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                if cpu.mem.gpu.is_enabled() && update_submitted {
                    unsafe {
                        gl::Clear(gl::COLOR_BUFFER_BIT);

                        gl::ActiveTexture(gl::TEXTURE0);
                        gl::BindTexture(gl::TEXTURE_2D, tex.assume_init());
                        gl::TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGB as gl::types::GLint,
                            160,
                            144,
                            0,
                            gl::RGB,
                            gl::UNSIGNED_BYTE,
                            cpu.mem.gpu.pixel_data.as_ptr() as *const _,
                        );

                        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo.assume_init());
                        gl::DrawElements(
                            gl::TRIANGLES,
                            6,
                            gl::UNSIGNED_INT,
                            (0 * mem::size_of::<f32>()) as *const () as *const _,
                        );
                    }
                }

                context.swap_buffers().unwrap();
                update_submitted = false;
            }
            Event::MainEventsCleared => {
                let mut requested_redraw = false;
                let mut fast_forward_loops = 0;

                while (!fast_forward && loop_time_remaining + max_frame <= Instant::now())
                    || (fast_forward && fast_forward_loops < 4)
                {
                    cpu.mem.set_input(&gb_buttons);

                    #[cfg(feature = "debugger")]
                    cpu.run(&mut debugger);
                    #[cfg(not(feature = "debugger"))]
                    cpu.run();

                    // Hard sync sleep
                    if !fast_forward {
                        loop_time_remaining += max_frame;
                    }

                    // Handle audio
                    #[cfg(feature = "enable_sound")]
                    {
                        let (mut samples, sample_count) = cpu.mem.sound.take_samples();

                        if !fast_forward {
                            for i in 0..samples.len() {
                                samples[i] /= 100f32;
                            }
                            let sample_buffer = SamplesBuffer::new(
                                2,
                                oxidgb_core::sound::OUTPUT_FREQUENCY,
                                &samples[0..sample_count],
                            );
                            audio_input.append(sample_buffer);
                        }
                    }

                    if !requested_redraw {
                        context.window().request_redraw();
                        requested_redraw = true;
                    }

                    update_submitted = true;

                    if fast_forward {
                        fast_forward_loops += 1;
                    }
                }
            }
            _ => (),
        }

        if loop_time_remaining + max_frame > Instant::now() {
            *control_flow = ControlFlow::WaitUntil(loop_time_remaining + max_frame);
        }
    });
}

// OpenGL resources
static VERTEX_DATA: [f32; 28] = [
    // X    Y    R    G    B    U    V
    -1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, // Top-left
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, // Top-right
    1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, // Bottom-right
    -1.0, -1.0, 1.0, 1.0, 1.0, 0.0, 1.0, // Bottom-left
];

static ELEMENTS: [u32; 6] = [0, 1, 2, 2, 3, 0];

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
