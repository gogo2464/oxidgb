/**
 * io.rs
 *
 * Handles the I/O registers.
**/

use core::cpu::interrupts::InterruptType;
use core::mem::GBMemory;
use core::gpu::GPUMode;

/// Storage for various I/O registers.
pub struct IORegisters {
    pub p1 : u8,    // 0x00 - Joypad info and controller (R/W)
    pub sb : u8,    // 0x01 - Serial transfer data (R/W)
    pub div : u16,  // 0x04 - Divider register (R/W)
    pub tima : u8,  // 0x05 - Timer Counter (R/W)
    pub tma : u8,   // 0x06 - Timer Modulo (R/W)
    pub tac : u8,   // 0x07 - Timer Control (R/W)
    pub iflag : u8, // 0x0F - (if) Interrupt Flag (R/W)
    pub nr10 : u8,  // 0x10 - Channel 1 Sweep register (R/W)
    pub nr11 : u8,  // 0x11 - Channel 1 Sound length/Wave pattern duty (R/W)
    pub nr12 : u8,  // 0x12 - Channel 1 Volume Envelope (R/W)
    pub nr13 : u8,  // 0x13 - Channel 1 Frequency lo (Write Only)
    pub nr14 : u8,  // 0x14 - Channel 1 Frequency hi (R/W)
    pub nr21 : u8,  // 0x16 - Channel 2 Sound Length/Wave Pattern Duty (R/W)
    pub nr22 : u8,  // 0x17 - Channel 2 Volume Envelope (R/W)
    pub nr23 : u8,  // 0x18 - Channel 2 Frequency lo data (W)
    pub nr24 : u8,  // 0x19 - Channel 2 Frequency hi data (R/W)
    pub nr30 : u8,  // 0x1A - Channel 3 Sound on/off (R/W)
    pub nr31 : u8,  // 0x1B - Channel 3 Sound Length
    pub nr32 : u8,  // 0x1C - Channel 3 Select output level (R/W)
    pub nr33 : u8,  // 0x1D - Channel 3 Frequency's lower data (W)
    pub nr34 : u8,  // 0x1E - Channel 3 Frequency's higher data (R/W)
    pub nr41 : u8,  // 0x20 - Channel 4 Sound Length (R/W)
    pub nr42 : u8,  // 0x21 - Channel 4 Volume Envelope (R/W)
    pub nr43 : u8,  // 0x22 - Channel 4 Polynomial Counter (R/W)
    pub nr44 : u8,  // 0x23 - Channel 4 Counter/consecutive; Initial (R/W)
    pub nr50 : u8,  // 0x24 - Channel control / ON-OFF / Volume (R/W)
    pub nr51 : u8,  // 0x25 - Selection of Sound output terminal (R/W)
    pub nr52 : u8,  // 0x26 - Sound on/off (R/W)
    pub wave : [u8; 0x10], // Wave Pattern RAM
    pub dma : u8,   // 0x46 - DMA Transfer and Start Address (W)
}

impl IORegisters {
    pub fn build() -> IORegisters {
        // TODO: Validate these
        return IORegisters {
            p1 : 0,
            sb : 0,
            div : 0xABCC,
            tima : 0,
            tma : 0,
            tac : 0xF8,
            nr10 : 0x80,
            nr11 : 0xBF,
            nr12 : 0xF3,
            nr13 : 0,
            nr14 : 0xBF,
            nr21 : 0x3F,
            nr22 : 0x00,
            nr23 : 0,
            nr24 : 0xBF,
            nr30 : 0x7F,
            nr31 : 0xFF,
            nr32 : 0x9F,
            nr33 : 0,
            nr34 : 0,
            nr41 : 0xFF,
            nr42 : 0x00,
            nr43 : 0x00,
            nr44 : 0,
            nr50 : 0x77,
            nr51 : 0xF3,
            nr52 : 0xF1,
            wave : [0; 0x10],
            iflag : 0,
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
            output |= ((if p14 {1} else {0}) << 5) | ((if p15 {1} else {0}) << 4);

            output
        }
        0x02 => mem.ioregs.sb,
        0x04 => (mem.ioregs.div >> 8) as u8,
        0x05 => mem.ioregs.tima,
        0x06 => mem.ioregs.tma,
        0x07 => mem.ioregs.tac,
        0x0F => mem.ioregs.iflag | !(0b11111),
        0x10 => mem.ioregs.nr10,
        0x11 => mem.ioregs.nr11,
        0x12 => mem.ioregs.nr12,
        0x13 => 0xFF, // Write only
        0x14 => mem.ioregs.nr14,
        0x16 => mem.ioregs.nr21,
        0x17 => mem.ioregs.nr22,
        0x18 => 0xFF, // Write only
        0x19 => mem.ioregs.nr24,
        0x1A => mem.ioregs.nr30,
        0x1B => mem.ioregs.nr31,
        0x1C => mem.ioregs.nr32,
        0x1D => 0xFF, // Write only
        0x1E => mem.ioregs.nr34,
        0x20 => mem.ioregs.nr41,
        0x21 => mem.ioregs.nr42,
        0x22 => mem.ioregs.nr43,
        0x23 => mem.ioregs.nr44,
        0x24 => mem.ioregs.nr50,
        0x25 => mem.ioregs.nr51,
        0x26 => mem.ioregs.nr52,
        0x30 ... 0x3F => mem.ioregs.wave[(ptr - 0x30) as usize],
        0x40 => mem.gpu.lcdc,
        0x41 => {
            if mem.gpu.lcdc >> 7 & 0x1 == 0 {
                // Screen disabled
                (1 << 7)
            } else {
                let stat = mem.gpu.stat & 0b1111000;
                let mode = (mem.gpu.mode as u8) & 0b11;
                let mut result = stat | mode;

                // Handle coin
                if mem.gpu.lyc == mem.gpu.current_line {
                    result |= 1 << 2;
                }

                result | (1 << 7)
            }
        }
        0x42 => mem.gpu.scy,
        0x43 => mem.gpu.scx,
        0x44 => mem.gpu.current_line,
        0x45 => mem.gpu.lyc,
        0x47 => mem.gpu.bgp,
        0x48 => mem.gpu.obp0,
        0x49 => mem.gpu.obp1,
        0x4A => mem.gpu.wy,
        0x4B => mem.gpu.wx,
        0x4C ... 0xFF => {
            warn!("Out of range I/O register: {:02x}", ptr);
            0xFF
        },
        _ => {
            warn!("Unknown I/O register: {:02x}", ptr);
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
        0x02 => mem.ioregs.sb = val,
        0x04 => mem.ioregs.div = 0,
        0x05 => mem.ioregs.tima = val,
        0x06 => mem.ioregs.tma = val,
        0x07 => mem.ioregs.tac = val & 0b111,
        0x0F => {
            mem.ioregs.iflag = val;
            mem.dirty_interrupts = true;
        },
        0x10 => mem.ioregs.nr10 = val,
        0x11 => mem.ioregs.nr11 = val,
        0x12 => mem.ioregs.nr12 = val,
        0x13 => mem.ioregs.nr13 = val,
        0x14 => mem.ioregs.nr14 = val,
        0x16 => mem.ioregs.nr21 = val,
        0x17 => mem.ioregs.nr22 = val,
        0x18 => mem.ioregs.nr23 = val,
        0x19 => mem.ioregs.nr24 = val,
        0x1A => mem.ioregs.nr30 = val,
        0x1B => mem.ioregs.nr31 = val,
        0x1C => mem.ioregs.nr32 = val,
        0x1D => mem.ioregs.nr33 = val,
        0x1E => mem.ioregs.nr34 = val,
        0x20 => mem.ioregs.nr41 = val,
        0x21 => mem.ioregs.nr42 = val,
        0x22 => mem.ioregs.nr43 = val,
        0x23 => mem.ioregs.nr44 = val,
        0x24 => mem.ioregs.nr50 = val,
        0x25 => mem.ioregs.nr51 = val,
        0x26 => mem.ioregs.nr52 = val,
        0x30 ... 0x3F => mem.ioregs.wave[(ptr - 0x30) as usize] = val,
        0x40 => {
            let old_bit = mem.gpu.lcdc >> 7;
            let changed_bit = val >> 7;
            if old_bit != changed_bit {
                if changed_bit == 0 {
                    if mem.gpu.mode != GPUMode::Vblank {
                        panic!("Disabling/enabling LCD during non-vblank! (actual mode: {:?})",
                               mem.gpu.mode);
                    }

                    mem.gpu.current_line = 0;
                } else {
                    mem.gpu.mode = GPUMode::Hblank;
                    mem.gpu.internal_clock = 0;

                    if (mem.gpu.stat >> 6) & 0x1 == 1 && mem.gpu.lyc == mem.gpu.current_line {
                        mem.ioregs.iflag |= 1 << (InterruptType::LCDC as u8);
                        mem.dirty_interrupts = true;
                    }
                    // TODO: Check STAT
                }
            }

            mem.gpu.lcdc = val;
        },
        0x41 => mem.gpu.stat = val,
        0x42 => mem.gpu.scy = val,
        0x43 => mem.gpu.scx = val,
        0x45 => mem.gpu.lyc = val,
        0x46 => {
            mem.ioregs.dma = val;
            execute_dma(mem);
        }
        0x47 => mem.gpu.bgp = val,
        0x48 => mem.gpu.obp0 = val,
        0x49 => mem.gpu.obp1 = val,
        0x4A => mem.gpu.wy = val,
        0x4B => mem.gpu.wx = val,
        0x4C ... 0xFF => {
            warn!("Out of range I/O register: {:02x} = {:02x}", ptr, val);
        },
        _ => {
            warn!("Unknown I/O register: {:02x} = {:02x}", ptr, val);
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
