/**
 * rom.rs
 *
 * Loads and parses .gb cartridges, and provides a interface for mappers.
**/

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::string::String;

/// The different kinds of cartridges that can be handled. Each has a
///  specific way of managing memory/providing additional capabilities.
#[derive(Debug)]
#[allow(dead_code)] // For debug messages
pub enum CartridgeType {
    RomOnly           = 0x00,
    RomMbc1           = 0x01,
    RomMbc1Ram        = 0x02,
    RomMbc1RamBatt    = 0x03,
    RomMbc2           = 0x05,
    RomMbc2Batt       = 0x06,
    RomRam            = 0x08,
    RomRamBatt        = 0x09,
    RomMMMD1          = 0x0B,
    RomMMMD1Sram      = 0x0C,
    RomMMMD1SramBatt  = 0x0D,
    RomMbc3TimerBatt  = 0x0F,
    RomMbc3TimerRamBatt = 0x10,
    RomMbc3           = 0x11,
    RomMbc3Ram        = 0x12,
    RomMbc3RamBatt    = 0x13,
    RomMbc5           = 0x19,
    RomMbc5Ram        = 0x1A,
    RomMbc5RamBatt    = 0x1B,
    RomMbc5Rumble     = 0x1C,
    RomMbc5RumbleSram = 0x1D,
    RomMbc5RumbleSramBatt = 0x1E,
    PocketCamera      = 0x1F,
    BandaiTAMA5       = 0xFD,
    HudsonHuC3        = 0xFE,
    HudsonHuC1        = 0xFF
}

/// Holds a game's ROM, and exposes interfaces to read information from
///  it intelligently.
pub struct GameROM {
    backing_data : Vec<u8>
}

impl GameROM {
    /// Gets the ROM name from the ROM header.
    pub fn get_name(&self) -> String {
        return String::from_utf8(self.backing_data[0x134 .. 0x142].to_vec()).unwrap();
    }

    /// Returns the cartridge type of the ROM.
    /// May return undefined enum entry if ROM type is not supported.
    pub fn get_cart_type(&self) -> CartridgeType {
        return unsafe { ::std::mem::transmute(self.backing_data[0x0147]) };
    }

    pub fn read(&self, ptr : u16) -> u8 {
        return self.backing_data[ptr as usize];
    }

    pub fn read_ram(&self, ptr : u16) -> u8 {
        println!("WARN: Cart RAM not implemented: {:04x}", ptr);
        return 0xFF;
    }

    pub fn write(&self, ptr : u16, val : u8) {
        println!("WARN: Writing to ROM: {:04x} = {:02x}", ptr, val);
    }

    pub fn write_ram(&self, ptr : u16, val : u8) {
        println!("WARN: Cart RAM not implemented: {:04x} = {:02x}", ptr, val);
    }

    /// Builds a new ROM from the specified file. Expects
    ///  a correctly formatted file.
    ///
    /// * `path` - The path to load from. Must be readable.
    pub fn build(path : &Path) -> GameROM {
        let file_size = match fs::metadata(path) {
            Err(why) => panic!("couldn't read metadata of {}: {}", path.display(),
                                why.description()),
            Ok(meta) => meta.len()
        };

        let mut data = Vec::with_capacity(file_size as usize);

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(),
                               why.description()),
            Ok(file) => file,
        };

        let read = file.read_to_end(&mut data).unwrap();

        println!("Read: {}, expected: {}", read, file_size);

        return GameROM {
            backing_data : data
        };
    }
}