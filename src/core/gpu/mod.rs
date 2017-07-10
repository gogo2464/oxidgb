/**
 * gpu.rs
 *
 * Renders graphics into a framebuffer
**/

pub const PITCH : usize = 3;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)] // For debug messages
pub enum GPUMode {
    Vblank = 0,
    Hblank = 1,
    OamScanline = 2,
    VramScanline = 3
}

pub struct GPU {
    pub pixel_data : [u8; 160 * 144 * PITCH],
    pub mode : GPUMode,
    pub palette : [u8; 4 * 3],

    pub vram : [u8; 8192],
    pub oam : [u8; 160],

    pub lcdc : u8,
    pub scx : u8,
    pub scy : u8,
    pub wx : u8,
    pub wy : u8,
    pub bgp : u8,
    pub obp0 : u8,
    pub obp1 : u8,

    pub internal_clock : u32,
    pub current_line : u8
}

impl GPU {
    /// Steps the GPU. Returns true if a Vblank interrupt should be thrown.
    pub fn step(&mut self, cycles : u32) -> bool {
        self.internal_clock += cycles;

        match self.mode {
            GPUMode::Vblank => {
                if self.internal_clock >= 4560 {
                    self.internal_clock -= 4560;

                    self.current_line += 1;

                    if self.current_line > 153 {
                        // VBlank is done, empty our framebuffer
                        for i in 0 .. 160 * 144 * PITCH {
                            // TODO: Fill with palette blanks
                            self.pixel_data[i] = 0xFF;
                        }

                        self.current_line = 0;
                        self.mode = GPUMode::OamScanline;
                    }
                }
            }
            GPUMode::Hblank => {
                if self.internal_clock >= 204 {
                    self.internal_clock -= 204;

                    self.current_line += 1;

                        if self.current_line > 143 {
                            self.mode = GPUMode::Vblank;

                            return true;
                        } else {
                            self.mode = GPUMode::OamScanline;
                        }
                }
            }
            GPUMode::OamScanline => {
                if self.internal_clock >= 80 {
                    self.internal_clock -= 80;
                    self.mode = GPUMode::VramScanline;

                    self.draw_vram();
                    self.draw_sprites();
                }
            }
            GPUMode::VramScanline => {
                if self.internal_clock >= 172 {
                    self.internal_clock -= 172;
                    self.mode = GPUMode::Hblank;
                }
            }
        }

        return false;
    }

    /// Returns if the screen is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.lcdc >> 7 & 0x1 == 1
    }

    /// Draws a pixel to the backing framebuffer, based upon the overall
    ///  RGB framebuffer.
    #[inline]
    fn draw_pixel(&mut self, pos : usize, shade : u8) {
        for i in 0 .. PITCH {
            self.pixel_data[pos * PITCH + i] = self.palette[shade as usize * PITCH + i];
        }
    }

    fn draw_vram(&mut self) {
        let display_screen    = self.lcdc >> 7 & 0x1 == 1;
        let window_tile_map   = self.lcdc >> 6 & 0x1 == 1;
        let window_display    = self.lcdc >> 5 & 0x1 == 1;
        let tile_data         = self.lcdc >> 4 & 0x1 == 1;
        let bg_tile_map       = self.lcdc >> 3 & 0x1 == 1;
        let bg_window_display = self.lcdc      & 0x1 == 1;

        if !display_screen {
            return
        }

        //lineState.fill(PixelState.EMPTY)

        // -- Tiles
        if bg_window_display {
            let mut x = self.scx as i16;
            if x < 0 {
                x += 32 * 8
            }
            let mut y = self.current_line as i16 + self.scy as i16;
            if y < 0 {
                y += 32 * 8
            }
            if y > 32 * 8 {
                y -= 32 * 8
            }

            for col in 0 .. 160 {
                // Work out which tile we are drawing
                let x_tile = x / 8;
                let y_tile = y / 8;

                let tile_pointer : usize;

                if bg_tile_map {
                    tile_pointer = (0x1C00 + y_tile * 32 + x_tile) as usize;
                } else {
                    tile_pointer = (0x1800 + y_tile * 32 + x_tile) as usize;
                }

                let data = self.vram[tile_pointer];

                let tex_pos: usize;
                if tile_data {
                    tex_pos = ((((data as i8) as i16 & 0xFF) * 16) + (y % 8) * 2) as usize;
                } else {
                    tex_pos = (0x1000 + ((data as i8) as i16 * 16) + (y % 8) * 2) as usize;
                }

                // Row is two bytes (16bits)
                let first_byte = self.vram[tex_pos];
                let second_byte = self.vram[tex_pos + 1];

                let bit = x % 8;

                // Combine our bits from first and second byte
                let first_bit = (first_byte >> (7 - bit)) & 0x1;
                let second_bit = (second_byte >> (7 - bit)) & 0x1;
                let combined_bit = first_bit | (second_bit * 2);
                let combined = (self.bgp >> (combined_bit * 2)) & 0b11;

                // If this pixel is filled in, render it (scaled) to the screen
                let pos = self.current_line as usize * 160 + col as usize;

                x += 1;

                while x >= 32 * 8 {
                    x -= 32 * 8
                }

                if pos >= self.pixel_data.len() {
                    continue
                }

                //lineState[col] = PixelState.BACKGROUND
                self.draw_pixel(pos, combined);
            }
        }

        // -- Window
        let wx = (self.wx & 0xFF) as u16;
        let wy = (self.wy & 0xFF) as u16;

        if window_display {
            let mut x = -(wx as i16) + 7;
            let y = (self.current_line as i16) - (wy as i16);

            for col in 0..160 {
                if y < 0 || y >= 144
                    || self.current_line as u16 >= wy + 144
                    || wx >= 160 || wy >= 144 {
                    break
                }

                // Work out which tile we are drawing
                let x_tile = x / 8;
                let y_tile = y / 8;

                let tile_pointer : usize;

                if window_tile_map {
                    tile_pointer = (0x1C00 + y_tile * 32 + x_tile) as usize;
                } else {
                    tile_pointer = (0x1800 + y_tile * 32 + x_tile) as usize;
                }

                let data = self.vram[tile_pointer as usize];

                let tex_pos: usize;

                if tile_data {
                    tex_pos = ((((data as i8) as i16 & 0xFF) * 16) + (y % 8) * 2) as usize;
                } else {
                    tex_pos = (0x1000 + ((data as i8) as i16 * 16) + (y % 8) * 2) as usize;
                }

                // Row is two bytes (16bits)
                let first_byte = self.vram[tex_pos];
                let second_byte = self.vram[tex_pos + 1];

                let bit = x % 8;

                // Combine our bits from first and second byte
                let first_bit = (first_byte >> (7 - bit)) & 0x1;
                let second_bit = (second_byte >> (7 - bit)) & 0x1;
                let combined_bit = first_bit | (second_bit * 2);
                let combined = (self.bgp >> (combined_bit * 2)) & 0b11;

                // If this pixel is filled in, render it (scaled) to the screen
                let pos = self.current_line as usize * 160 + col as usize;

                x += 1;

                if self.current_line < self.wy {
                    continue
                }

                if pos >= self.pixel_data.len() {
                    continue
                }

                //lineState[col] = PixelState.WINDOW
                self.draw_pixel(pos, combined);
            }
        }
    }


    fn draw_sprites(&mut self) {
        let sprite_size    = self.lcdc >> 2 & 0x1 == 1;
        let sprite_display = self.lcdc >> 1 & 0x1 == 1;

        let sprite_height : i16 = if sprite_size {16} else {8};

        // -- Sprites
        let mut sprite_row_count = 0;

        if sprite_display {
            for sprite_index in 0 .. 40 {
                let info_ptr = sprite_index * 4;

                let x_pos = ((self.oam[info_ptr + 1] as u16 & 0xFF) as i16) - 8;
                let y_pos = ((self.oam[info_ptr] as u16 & 0xFF) as i16) - 16;

                let y_tile = self.current_line as i16 - y_pos;

                if y_pos <= self.current_line as i16 - sprite_height
                    || y_pos > self.current_line as i16 {
                    continue
                }

                sprite_row_count += 1;
                if sprite_row_count > 10 {
                    break
                }

                let info = self.oam[info_ptr + 3] as i8 as i16 & 0xFF;
                let has_priority = info >> 7 & 0x1 == 0;
                let y_flip = info >> 6 & 0x1 == 1;
                let x_flip = info >> 5 & 0x1 == 1;
                let palette = info >> 4 & 0x1 == 1;

                let tile_pos = (self.oam[info_ptr + 2] as i8 as i16 &
                                    if sprite_size {0b11111110} else {0xFF}) * 16;
                let tex_pos = (tile_pos +
                             if y_flip { sprite_height - 1 - y_tile } else { y_tile } * 2) as usize;

                let first_byte = self.vram[tex_pos] & 0xFF;
                let second_byte = self.vram[tex_pos + 1] & 0xFF;

                for bit in 0 .. 8 {
                    if y_pos + y_tile >= 144
                        || x_pos + if x_flip {7 - bit} else {bit} < 0
                        || x_pos + if x_flip {7 - bit} else {bit} >= 160 {
                        //println!("Bad pos: (x={}, y={}, y_tile={}", x_pos, y_pos, y_tile);
                        continue
                    }

                    // Combine our bits from first and second byte
                    let first_bit = (first_byte >> 7 - bit) & 0x1;
                    let second_bit = (second_byte >> 7 - bit) & 0x1;
                    let combined_bit = first_bit + second_bit * 2;

                    let combined : u8;
                    if palette {
                        combined = (self.obp1 >> (combined_bit * 2)) & 0b11;
                    } else {
                        combined = (self.obp0 >> (combined_bit * 2)) & 0b11;
                    }

                    let array_pos = ((y_pos + y_tile)
                        * 160 + x_pos + if x_flip {7 - bit} else {bit}) as usize;

                    if combined_bit == 0x00 {
                        continue
                    }

                    // If this pixel is filled in, render it (scaled) to the screen
                    if has_priority || !has_priority
                        && (self.pixel_data[array_pos * PITCH] & 0xFF) as u8 == self.palette[0] {
                        self.draw_pixel(array_pos, combined);
                    }
                }
            }
        }
    }

    /// Builds a new instance of the GPU
    pub fn build() -> GPU {
        return GPU {
            pixel_data : [0xFF; 160 * 144 * PITCH],
            mode : GPUMode::Hblank,
            palette : [224,248,208, 136,192,112, 52,104,86, 8,24,32], // BGB palette

            vram : [0; 8192],
            oam : [0; 160],

            lcdc: 0x91,
            scx: 0,
            scy: 0,
            wx: 0,
            wy: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,

            internal_clock: 0,
            current_line: 0,
        };
    }
}
