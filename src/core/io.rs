/**
 * io.rs
 *
 * Handles the I/O registers.
**/

use core::mem::GBMemory;

/// Reads a I/O register.
pub fn read(mem : &GBMemory, ptr : u8) -> u8 {
    return match ptr {
        0x40 => mem.gpu.lcdc,
        0x42 => mem.gpu.scy,
        0x43 => mem.gpu.scx,
        0x44 => mem.gpu.current_line,
        0x47 => mem.gpu.bgp,
        0x48 => mem.gpu.obp0,
        0x49 => mem.gpu.obp1,
        0x4A => mem.gpu.wy,
        0x4B => mem.gpu.wx,
        _ => {
            //println!("Unknown I/O register: {:04x}", ptr);
            0xFF
        }
    }
}

/// Writes to a I/O register.
pub fn write(mem : &mut GBMemory, ptr : u8, val : u8) {
    match ptr {
        0x01 => {
            //print!("{}", char::from_u32(val as u32).unwrap());
        }
        0x40 => mem.gpu.lcdc = val,
        0x42 => mem.gpu.scy = val,
        0x43 => mem.gpu.scx = val,
        0x47 => mem.gpu.bgp = val,
        0x48 => mem.gpu.obp0 = val,
        0x49 => mem.gpu.obp1 = val,
        0x4A => mem.gpu.wy = val,
        0x4B => mem.gpu.wx = val,
        _ => {
            //println!("Unknown I/O register: {:02x} = {:02x}", ptr, val);
        }
    }
}
