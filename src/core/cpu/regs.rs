/**
 * regs.rs
 *
 * Contains definitions for GB's CPU registers + functions for 16-bit operations.
**/

/// The available CPU registers on a Gameboy.
pub struct Registers {
    pub a : u8,
    pub b : u8,
    pub c : u8,
    pub d : u8,
    pub e : u8,
    pub f : u8,

    pub h : u8,
    pub l : u8,

    pub sp : u16,
    pub pc : u16
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        return ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn get_bc(&self) -> u16 {
        return ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        return ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_af(&mut self, val : u16) {
        self.a = ((val >> 8) & 0xFF) as u8;
        self.f = ((val) & 0xFF) as u8;
    }
}