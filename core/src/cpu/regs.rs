/**
 * regs.rs
 *
 * Contains definitions for GB's CPU registers + functions for 16-bit operations.
**/

/// The available CPU registers on a Gameboy.
#[cfg_attr(feature = "serialisation", derive(Serialize, Deserialize))]
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
        ((self.a as u16) << 8) | (self.f as u16 & 0xF0)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_af(&mut self, val : u16) {
        self.a = ((val >> 8) & 0xFF) as u8;
        self.f = ((val) & 0xF0) as u8;
    }

    pub fn set_bc(&mut self, val : u16) {
        self.b = ((val >> 8) & 0xFF) as u8;
        self.c = ((val) & 0xFF) as u8;
    }

    pub fn set_de(&mut self, val : u16) {
        self.d = ((val >> 8) & 0xFF) as u8;
        self.e = ((val) & 0xFF) as u8;
    }

    pub fn set_hl(&mut self, val : u16) {
        self.h = ((val >> 8) & 0xFF) as u8;
        self.l = ((val) & 0xFF) as u8;
    }

    pub fn get_flag_z(&self) -> bool {
        ((self.f >> 7) & 0x1) == 1
    }

    pub fn get_flag_n(&self) -> bool {
        ((self.f >> 6) & 0x1) == 1
    }

    pub fn get_flag_h(&self) -> bool {
        ((self.f >> 5) & 0x1) == 1
    }

    pub fn get_flag_c(&self) -> bool {
        ((self.f >> 4) & 0x1) == 1
    }

    pub fn set_flag_z(&mut self, val : bool) {
        if val {
            self.f |= 1 << 7;
        } else {
            self.f &= !(1 << 7);
        }
    }

    pub fn set_flag_n(&mut self, val : bool) {
        if val {
            self.f |= 1 << 6;
        } else {
            self.f &= !(1 << 6);
        }
    }

    pub fn set_flag_h(&mut self, val : bool) {
        if val {
            self.f |= 1 << 5;
        } else {
            self.f &= !(1 << 5);
        }
    }

    pub fn set_flag_c(&mut self, val : bool) {
        if val {
            self.f |= 1 << 4;
        } else {
            self.f &= !(1 << 4);
        }
    }
}
