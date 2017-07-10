/**
 * io.rs
 *
 * Handles the I/O registers.
**/

use core::mem::GBMemory;

/// Storage for various I/O registers.
pub struct IORegisters {
    pub p1 : u8,    // 0x00 - Joypad info and controller (R/W)
    pub div : u16,  // 0x04 - Divider register (R/W)
    pub tima : u8,  // 0x05 - Timer Counter (R/W)
    pub tma : u8,   // 0x06 - Timer Modulo (R/W)
    pub tac : u8,   // 0x07 - Timer Control (R/W)
    pub iflag : u8, // 0x0F - (if) Interrupt Flag (R/W)
    pub stat : u8,  // 0x41 - LCDC Status (R/W)
    pub lyc : u8,   // 0x45 - LY Compare (R/W)
    pub dma : u8,   // 0x46 - DMA Transfer and Start Address (W)
}

impl IORegisters {
    pub fn build() -> IORegisters {
        // TODO: Validate these
        return IORegisters {
            p1 : 0,
            div : 0xABCC,
            tima : 0,
            tma : 0,
            tac : 0,
            iflag : 0,
            stat : 0,
            lyc : 0,
            dma : 0
        }
    }
}

// These are separate as they need to access the entirety of memory

/// Reads a I/O register.
pub fn read(mem : &GBMemory, ptr : u8) -> u8 {
    return match ptr {
        0x00 => {
            let p14 = (mem.ioregs.p1 >> 5) & 0x1 == 1;
            let p15 = (mem.ioregs.p1 >> 4) & 0x1 == 1;

            let mut output = 0;

            if p14 {
                output |= mem.buttons.p14;
            }

            if p15 {
                output |= mem.buttons.p15;
            }

            output = (!output) & 0b1111;
            output |= ((if p14 {1} else {0}) >> 5) | ((if p15 {1} else {0}) >> 4);

            output
        }
        0x04 => ((mem.ioregs.div >> 8) & 0xFF) as u8,
        0x05 => mem.ioregs.tima,
        0x06 => mem.ioregs.tma,
        0x07 => mem.ioregs.tac,
        0x0F => mem.ioregs.iflag,
        0x40 => mem.gpu.lcdc,
        0x41 => {
            let stat = mem.ioregs.stat & 0b1111000;
            let mode = (mem.gpu.mode as u8) & 0b11;
            let mut result = stat | mode;

            // Handle coin
            if mem.ioregs.lyc == mem.gpu.current_line {
                result |= 1 << 2;
            }

            result
        }
        0x42 => mem.gpu.scy,
        0x43 => mem.gpu.scx,
        0x44 => mem.gpu.current_line,
        0x47 => mem.gpu.bgp,
        0x48 => mem.gpu.obp0,
        0x49 => mem.gpu.obp1,
        0x4A => mem.gpu.wy,
        0x4B => mem.gpu.wx,
        _ => {
            println!("Unknown I/O register: {:04x}", ptr);
            0xFF
        }
    }
}

/// Writes to a I/O register.
pub fn write(mem : &mut GBMemory, ptr : u8, val : u8) {
    match ptr {
        0x00 => mem.ioregs.p1 = val,
        0x01 => {
            // Serial
            //print!("{}", char::from_u32(val as u32).unwrap());
        },
        0x04 => mem.ioregs.div = 0,
        0x05 => mem.ioregs.tima = val,
        0x06 => mem.ioregs.tma = val,
        0x07 => mem.ioregs.tac = val & 0b111,
        0x0F => {
            mem.ioregs.iflag = val;
            mem.dirty_interrupts = true;
        },
        0x40 => mem.gpu.lcdc = val,
        0x42 => mem.gpu.scy = val,
        0x43 => mem.gpu.scx = val,
        0x45 => mem.ioregs.lyc = val,
        0x46 => {
            mem.ioregs.dma = val;
            execute_dma(mem);
        }
        0x47 => mem.gpu.bgp = val,
        0x48 => mem.gpu.obp0 = val,
        0x49 => mem.gpu.obp1 = val,
        0x4A => mem.gpu.wy = val,
        0x4B => mem.gpu.wx = val,
        _ => {
            println!("Unknown I/O register: {:02x} = {:02x}", ptr, val);
        }
    }
}

/// Executes a DMA.
fn execute_dma(mem : &mut GBMemory) {
    // TODO: Locking
    let address = (mem.ioregs.dma as u16) * 0x100;

    for i in 0 .. 0xA0 {
        let byte = mem.read(address + i);
        mem.write(0xFE00 + i, byte);
    }
}
