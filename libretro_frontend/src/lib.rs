#![feature(vec_remove_item)]
/**
 * lib.rs
 *
 * The main entry-point for the LibRetro frontend
**/

#[macro_use]
extern crate libretro_backend;

#[macro_use]
extern crate log;

extern crate oxidgb_core;

mod logging;

use libretro_backend::*;

use oxidgb_core::input::GameboyButton;
use oxidgb_core::rom::GameROM;
use oxidgb_core::mem::GBMemory;
use oxidgb_core::cpu::CPU;

use std::path::Path;
use std::fs;
use std::fs::File;
use oxidgb_core::rom::get_rom_size;

struct OxidgbEmulator {
    game_data: Option<GameData>,
    cpu: Option<CPU>
}

impl OxidgbEmulator {
    fn new() -> Self {
        OxidgbEmulator {
            game_data: None,
            cpu: None
        }
    }
}

impl Default for OxidgbEmulator {
    fn default() -> Self {
        Self::new()
    }
}

impl libretro_backend::Core for OxidgbEmulator {
    fn on_serialize( &mut self, _data: *mut libc::c_void, _size: libc::size_t ) -> bool {
        false
    }

    fn info() -> CoreInfo {
        CoreInfo::new("oxidgb", env!("CARGO_PKG_VERSION"))
            .supports_roms_with_extension("gb")
    }

    fn on_load_game(&mut self, game_data: GameData) -> LoadGameResult {
        // Set up logger
        // TODO: gate is_verbose
        logging::setup_logging(true).unwrap();

        let rom =  if let Some(data) = game_data.data() {
            GameROM::build(data.to_owned())
        } else if let Some(path) = game_data.path() {
            let rom_path = Path::new(path);
            if !rom_path.exists() {
                panic!("Specified file does not exist.");
            }

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

            GameROM::build(data)
        } else {
            unreachable!();
        };

        let memory = GBMemory::build(rom);

        let cpu = CPU::build(memory);

        self.game_data = Some(game_data);
        self.cpu = Some(cpu);

        let info = AudioVideoInfo::new()
            .video(160, 144,
                   60.0, PixelFormat::ARGB8888)
            .audio(48000f64);

        LoadGameResult::Success(info)
    }

    fn on_unload_game(&mut self) -> GameData {
        self.game_data.take().unwrap()
    }

    fn on_run(&mut self, handle: &mut RuntimeHandle) {
        let mut cpu = self.cpu.take().unwrap();

        cpu.run(&mut None);

        let mut pixel_data = [0 as u8; 160 * 144 * 4];

        {
            let src_data = &cpu.mem.gpu.pixel_data;
            for i in 0 .. 160 * 144 {
                pixel_data[i * 4] = src_data[i * 3];
                pixel_data[i * 4 + 1] = src_data[i * 3 + 1];
                pixel_data[i * 4 + 2] = src_data[i * 3 + 2];
                pixel_data[i * 4 + 3] = 0;
            }
        }

        handle.upload_audio_frame()
        handle.upload_video_frame(&pixel_data);

        self.cpu = Some(cpu);
    }

    fn on_reset(&mut self) {
        // Deconstruct current state
        let cpu = self.cpu.take().unwrap();
        let memory = cpu.mem;

        // Take ROM and run
        let rom = memory.rom;
        let memory = GBMemory::build(rom);
        let cpu = CPU::build(memory);
        self.cpu = Some(cpu);
    }
}

libretro_core!(OxidgbEmulator);

