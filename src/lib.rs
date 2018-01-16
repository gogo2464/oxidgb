#![no_std]
#![feature(start)]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(alloc)]
#![feature(global_allocator)]
#![feature(allocator_api)]

#[macro_use]
extern crate alloc;

use alloc::allocator::Alloc;
use alloc::allocator::Layout;
use alloc::allocator::AllocErr;
use alloc::String;

use core::intrinsics;

pub mod rom;
pub mod mem;
pub mod cpu;
pub mod gpu;
pub mod input;

mod io;

use rom::GameROM;
use mem::GBMemory;
use cpu::CPU;

extern "C" {
    pub fn printf(str : *const u8) -> usize;
}

fn print(content : &str) {
    // We don't have any kind of proper CString API. /shrug
    let mut buf = String::new();
    buf += content;
    buf += "\0";
    unsafe {
        printf(buf.as_ptr());
    }
}

// Stolen from rust source
macro_rules! print {
    ($($arg:tt)*) => (print(&format!($($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32,
                               _column: u32) -> ! {
    unsafe {
        println!("PANIC: {} at {}:{}: {}", _file, _line, _column, _msg);
        intrinsics::abort()
    }
}

extern "C" {
    pub fn malloc(size : usize) -> *const u8;
    pub fn free(ptr : *mut u8);

    pub fn submit_frame(ptr : *const u8, surf : *mut surface_t, vsync : *mut u32, out_buffer : *mut u32);

    // LibTransistor
    pub fn sm_init() -> u32;
    pub fn gpu_initialize() -> u32;
    pub fn vi_init() -> u32;
    pub fn display_init() -> u32;

    pub fn display_open_layer(surf : *mut surface_t) -> u32;
    pub fn display_get_vsync_event(surf : *mut u32) -> u32;

    pub fn surface_dequeue_buffer(surf : *mut surface_t, buffer : *mut *mut u32) -> u32;
    pub fn surface_queue_buffer(surf : *mut surface_t) -> u32;

    pub fn gfx_slow_swizzling_blit(buf : *mut u32, logo : *const u32, width : u32,
                                   height : u32, tx : u32, ty : u32);

    pub fn svcWaitSynchronization(handle_idx : *mut u32, vsync : *mut u32,
                                  num_handles : u32, timeout : u64) -> u32;
    pub fn svcResetSignal(vsync : u32) -> u32;
}

pub struct LibcAllocator {}

unsafe impl<'a> Alloc for &'a LibcAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let ptr = malloc(layout.size()) as *mut u8;
        Ok(ptr)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, _: Layout) {
        free(ptr);
    }
}

#[global_allocator]
static HEAP_ALLOCATOR: LibcAllocator = LibcAllocator {};

// Very basic wrapper around libtransistors surfaces
#[repr(C)]
#[allow(non_snake_case)]
pub struct binder_t {
    handle : i32
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct igbp_t {
    igbp_binder : binder_t
}

#[repr(C)]
#[allow(non_snake_case)]
pub enum surface_state_t {
    #[allow(non_camel_case_types)]
    SURFACE_STATE_INVALID,
    #[allow(non_camel_case_types)]
    SURFACE_STATE_DEQUEUED,
    #[allow(non_camel_case_types)]
    SURFACE_STATE_QUEUED,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct gpu_buffer_t {
    nvmap_handle : u32,
    size : usize,
    alignment : u32,
    kind : u8
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct graphic_buffer_t {
    width : u32,
    height : u32,
    stride : u32,
    format : u32,
    usage : u32,
    gpu_buffer : *mut gpu_buffer_t,
    index : i32,
    unknown : i32
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct surface_t {
    layer_id : u64,
    igbp : igbp_t,
    state : surface_state_t,
    has_requested : [bool; 2],
    current_slot : u32,
    gpu_buffer : gpu_buffer_t,
    gpu_buffer_memory : *mut u32,
    gpu_buffer_memory_alloc : *mut u32,
    graphic_buffers : [graphic_buffer_t; 2]
}

static ROM_DATA : &'static [u8] = include_bytes!("rom.gb");

#[no_mangle]
#[start]
pub fn main() {
    let mut surface_t = surface_t {
        layer_id: 0,
        igbp: igbp_t { igbp_binder: binder_t { handle: 0 } },
        state: surface_state_t::SURFACE_STATE_INVALID,
        has_requested: [false, false],
        current_slot: 0,
        gpu_buffer: gpu_buffer_t {
            nvmap_handle: 0,
            size: 0,
            alignment: 0,
            kind: 0,
        },
        gpu_buffer_memory: 0 as *mut u32,
        gpu_buffer_memory_alloc: 0 as *mut u32,
        graphic_buffers: [
            graphic_buffer_t {
                width: 0,
                height: 0,
                stride: 0,
                format: 0,
                usage: 0,
                gpu_buffer: 0 as *mut gpu_buffer_t,
                index: 0,
                unknown: 0,
            },
            graphic_buffer_t {
                width: 0,
                height: 0,
                stride: 0,
                format: 0,
                usage: 0,
                gpu_buffer: 0 as *mut gpu_buffer_t,
                index: 0,
                unknown: 0,
            }
        ],
    };

    let mut vsync : u32 = 0;

    unsafe {
        assert_eq!(sm_init(), 0);
        assert_eq!(gpu_initialize(), 0);
        assert_eq!(vi_init(), 0);
        assert_eq!(display_init(), 0);

        assert_eq!(display_open_layer(&mut surface_t), 0);
        assert_eq!(display_get_vsync_event(&mut vsync), 0);
    }

    println!("Display ready");

    let rom = GameROM::build(ROM_DATA);
    let memory = GBMemory::build(rom);
    let mut cpu = CPU::build(memory);

    println!("Emulator ready");


    loop {
        println!("Starting emulation for 1 frame...");

        cpu.run(&mut None);
        let mut buf = vec!(0 as u8; 160 * 144 * 4);
        println!("Converting buffer...");

        // libtransistor expects rgba, we render using a rgb buffer. convert.
        for y in 0 .. 144 {
            for x in 0 .. 160 {
                buf[(y * 160 + x) * 4] = cpu.mem.gpu.pixel_data[(y * 160 + x) * 3];
                buf[(y * 160 + x) * 4 + 1] = cpu.mem.gpu.pixel_data[(y * 160 + x) * 3 + 1];
                buf[(y * 160 + x) * 4 + 2] = cpu.mem.gpu.pixel_data[(y * 160 + x) * 3 + 2];
                buf[(y * 160 + x) * 4 + 3] = 0xFF;
            }
        }

        println!("Submitting frame...");

        unsafe {
            let mut output_buffer: *mut u32 = 0 as *mut u32;
            assert_eq!(surface_dequeue_buffer(&mut surface_t, &mut output_buffer), 0);

            let output_buffer_raw = core::slice::from_raw_parts_mut(output_buffer,
                                                                    0x3c0000/4);
            for p in 0 .. 0x3c0000/4 {
                output_buffer_raw[p] = 0xFF0000FF;
            }

            gfx_slow_swizzling_blit(output_buffer, buf.as_ptr() as *const _,
                                    160, 144, 1, 1);

            assert_eq!(surface_queue_buffer(&mut  surface_t), 0);

            let mut handle : u32 = 0;
            assert_eq!(svcWaitSynchronization(&mut handle, &mut vsync,
                                              1, 33333333), 0);
            assert_eq!(svcResetSignal(vsync), 0);

        }

        println!("Done!");
    }
}
