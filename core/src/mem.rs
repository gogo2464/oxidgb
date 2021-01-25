use input::build_input;
use input::GameboyButton;
/**
 * mem.rs
 *
 * Handles the Gameboy's memory bus.
**/
use input::GameboyInput;

use rom::GameROM;

use gpu::GPUMode;
use gpu::GPU;

use io;
use io::IORegisters;

use sound::Sound;

#[cfg(feature = "heap_alloc")]
use alloc::vec::Vec;

#[cfg_attr(feature = "serialisation", derive(Serialize, Deserialize))]
pub struct GBMemory<'a> {
    pub rom: GameROM<'a>,

    #[cfg(feature = "heap_alloc")]
    pub ram: Vec<u8>, // Fixed size of 8192
    #[cfg(not(feature = "heap_alloc"))]
    pub ram: [u8; 8192],
    #[cfg(feature = "heap_alloc")]
    pub high_ram: Vec<u8>, // Fixed size of 127 (not 128, as - interrupt enable reg)
    #[cfg(not(feature = "heap_alloc"))]
    pub high_ram: [u8; 127],

    pub gpu: GPU,
    pub sound: Sound,

    pub dirty_interrupts: bool, // If the CPU should handle interrupts
    pub interrupt_reg: u8,
    pub ioregs: IORegisters,

    pub buttons: GameboyInput,
}

impl GBMemory<'_> {
    /// Reads a value from memory. 0xFF if invalid.
    pub fn read(&self, ptr: u16) -> u8 {
        match ptr {
            0xFFFF => {
                // Interrupt enable reg
                self.interrupt_reg
            }
            0xFF80..=0xFFFE => {
                // High internal RAM+
                self.high_ram[(ptr - 0xFF80) as usize]
            }
            0xFF00..=0xFF7F => {
                // I/O Registers
                io::read(self, (ptr & 0xFF) as u8)
            }
            0xFEA0..=0xFEFF => {
                // Unusable
                //println!("WARN: Reading from unreadable memory: {:04x}", ptr);
                0x00
            }
            0xFE00..=0xFE9F => {
                // OAM
                // Check if read is valid
                match self.gpu.mode {
                    GPUMode::Vblank | GPUMode::Hblank => self.gpu.oam[(ptr - 0xFE00) as usize],
                    _ => {
                        //println!("Inaccessible OAM: {:04x}", ptr);
                        0xFF
                    }
                }
            }
            0xE000..=0xFDFF => {
                // RAM Echo
                self.ram[(ptr - 0xE000) as usize]
            }
            0xC000..=0xDFFF => {
                // Internal RAM
                self.ram[(ptr - 0xC000) as usize]
            }
            0xA000..=0xBFFF => {
                // Switchable RAM
                self.rom.read_ram(ptr - 0xA000)
            }
            0x8000..=0x9FFF => {
                // GPU
                // Check if read is valid
                match self.gpu.mode {
                    GPUMode::Vblank | GPUMode::Hblank | GPUMode::OamScanline => {
                        self.gpu.vram[(ptr - 0x8000) as usize]
                    }
                    _ => {
                        //println!("Inaccessible VRAM: {:04x}", ptr);
                        0xFF
                    }
                }
            }
            0x0000..=0x7FFF => {
                // Cartridge / Switchable ROM
                self.rom.read(ptr)
            }
        }
    }

    /// Writes a value to a memory location if possible.
    pub fn write(&mut self, ptr: u16, val: u8) {
        //println!("${:04X}: Write ${:02X}", ptr, val);

        match ptr {
            0xFFFF => {
                // Interrupt enable reg
                self.interrupt_reg = val;
            }
            0xFF80..=0xFFFE => {
                // High internal RAM
                self.high_ram[(ptr - 0xFF80) as usize] = val;
            }
            0xFF00..=0xFF7F => {
                // I/O Registers
                io::write(self, (ptr & 0xFF) as u8, val);
            }
            0xFEA0..=0xFEFF => { // Unusable
                 //println!("WARN: Writing to unreadable memory: {:04x} = {:02x}", ptr, val);
            }
            0xFE00..=0xFE9F => {
                // OAM
                // Check if write is valid
                match self.gpu.mode {
                    GPUMode::Vblank | GPUMode::Hblank => {
                        self.gpu.oam[(ptr - 0xFE00) as usize] = val
                    }
                    _ => {
                        //println!("Inaccessible OAM: {:04x} = {:02x}", ptr, val);
                    }
                };
            }
            0xE000..=0xFDFF => {
                // RAM Echo
                self.ram[(ptr - 0xE000) as usize] = val;
            }
            0xC000..=0xDFFF => {
                // Internal RAM
                self.ram[(ptr - 0xC000) as usize] = val;
            }
            0xA000..=0xBFFF => {
                // Switchable RAM
                self.rom.write_ram(ptr - 0xA000, val);
            }
            0x8000..=0x9FFF => {
                // GPU
                // Check if write is valid
                match self.gpu.mode {
                    GPUMode::Vblank | GPUMode::Hblank | GPUMode::OamScanline => {
                        self.gpu.vram[(ptr - 0x8000) as usize] = val
                    }
                    _ => {
                        //println!("Inaccessible VRAM: {:04x} = {:02x}", ptr, val);
                    }
                };
            }
            0x0000..=0x7FFF => {
                // Cartridge / Switchable ROM
                self.rom.write(ptr, val);
            }
        }
    }

    /// Reads a short. 0xFFFF if invalid.
    pub fn read_short(&self, ptr: u16) -> u16 {
        (self.read(ptr) as u16) | ((self.read(ptr + 1) as u16) << 8)
    }

    /// Writes a short value to a memory location if possible.
    pub fn write_short(&mut self, ptr: u16, val: u16) {
        self.write(ptr, (val & 0xFF) as u8);
        self.write(ptr + 1, ((val >> 8) & 0xFF) as u8);
    }

    /// Sets the input registers.
    pub fn set_input(&mut self, input: &[GameboyButton]) {
        self.buttons = build_input(input);
    }

    /// Builds a new memory manager.
    pub fn build(rom: GameROM) -> GBMemory {
        GBMemory {
            rom,
            #[cfg(feature = "heap_alloc")]
            ram: vec![0; 8192],
            #[cfg(not(feature = "heap_alloc"))]
            ram: [0; 8192],
            #[cfg(feature = "heap_alloc")]
            high_ram: vec![0; 127],
            #[cfg(not(feature = "heap_alloc"))]
            high_ram: [0; 127],

            gpu: GPU::build(),
            sound: Sound::build(),

            dirty_interrupts: false,
            interrupt_reg: 0,
            ioregs: IORegisters::build(),

            buttons: GameboyInput { p14: 0, p15: 0 },
        }
    }
}
