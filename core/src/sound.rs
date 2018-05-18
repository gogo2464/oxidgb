use io::IORegisters;

/// sound.rs
///
/// I/O register sound emulation.

// TODO: Vary this on different platforms?
const SOUND_CPU_SPEED : u32 = 4194304;

const SOUND_LENGTH_CLOCK_STEP : u32 = (SOUND_CPU_SPEED as f32 / 256f32) as u32;

pub const OUTPUT_FREQUENCY : u32 = 48000;

const FRAME_SIZE : usize = (OUTPUT_FREQUENCY / 59) as usize * 2; // 59 as framerate != 60, but approximating is hard

/// Generates a rectangular wave.
///
/// step: current clock step
/// frequency: target frequency of wave
/// low_percent: percentage of which the wave should be low (e.g. 0.5 for square wave)
/// Returns: -1 .. 1 inclusive output
fn rectangle_wave(step : u64, frequency : u32, low_percent : f32) -> f32 {
    let step = (step as f64) / SOUND_CPU_SPEED as f64;

    //return (step * frequency as f64).sin() as f32;

    let current_interval_float = step * frequency as f64;
    let current_interval = current_interval_float as u64;

    let interval_percentage = current_interval_float - current_interval as f64;

    //println!("x: {} x {} x {} x {} x {} x {}", step, current_interval_float, current_interval, interval_percentage, frequency, low_percent);

    if interval_percentage < low_percent.into() {
        return -1f32;
    } else if interval_percentage == low_percent.into() {
        return 0f32;
    } else {
        return 1f32;
    }
}


pub struct Sound {
    channel_1_running : bool,
    channel_1_step : u64,

    channel_2_running : bool,
    channel_2_step : u64,

    samples : [f32; FRAME_SIZE],
    sample_pointer : usize,

    last_cycle : u64
}

impl Sound {
    /// Iterates the sound engine, bumping the internal buffers.
    pub fn step(&mut self, registers : &mut IORegisters, cycles : u8) {
        // Check for disabled
        if (registers.nr52 >> 7) & 0x1 == 0 {
            return;
        }

        // Check if channel 1 should be reset
        if (registers.nr14 >> 7) & 0x1 == 1 {
            // Clear register
            registers.nr14 &= !(1 << 7);

            self.channel_1_running = true;
            self.channel_1_step = 0;
        }

        let mut register_1_wave = 0f32;

        // Handle channel 1
        if self.channel_1_running {
            // Step sound length if needed
            if (registers.nr14 >> 6) & 0x1 == 1 {
                //let new_step
                let mut current_sound_tick = registers.nr11 & 0b111111;

                if self.channel_1_step / SOUND_LENGTH_CLOCK_STEP as u64 !=
                    (self.channel_1_step + cycles as u64) / SOUND_LENGTH_CLOCK_STEP as u64 {
                    current_sound_tick += 1;

                    registers.nr11 &= !0b111111;
                    registers.nr11 |= current_sound_tick & 0b111111;
                }

                if current_sound_tick > 63 {
                    self.channel_1_running = false;
                }
            }

            // Get pattern duty
            let raw_wave_pattern_duty = (registers.nr11 >> 6) & 0b11;
            let low_percent = (raw_wave_pattern_duty as f32) * 0.125f32;

            // Get frequency
            let mut gb_frequency = (registers.nr13 as u32) | (((registers.nr14 & 0b111) as u32) << 8);

            // Sweep register
            let raw_sweep_time = (registers.nr10 >> 4) & 0b111;
            let sweep_time = (raw_sweep_time as f32) / 128f32 * (SOUND_CPU_SPEED as f32);

            if raw_sweep_time > 0 &&
                self.channel_1_step / sweep_time as u64 !=
                    (self.channel_1_step + cycles as u64) / sweep_time as u64 {
                // Shift frequency
                let shift_iterations = registers.nr10 & 0b111;
                let component = gb_frequency / 2_u32.pow(shift_iterations as u32);

                if (registers.nr10 >> 3) & 0x1 == 0x1 {
                    if gb_frequency as i32 - component as i32 >= 0 {
                        gb_frequency -= component;
                    }
                } else {
                    if gb_frequency + component <= 0b111 {
                        gb_frequency += component;
                    }
                }

                // Update frequency
                registers.nr13 = (gb_frequency & 0xFF) as u8;
                registers.nr14 &= !(0b111);
                registers.nr14 |= ((gb_frequency >> 8) & 0b111) as u8;
            }

            let hz_frequency = 131072 / (2048 - gb_frequency);

            // Get pattern
            let wave_pattern = rectangle_wave(self.channel_1_step, hz_frequency, low_percent);

            // Get volume
            let mut volume = (registers.nr12 >> 4) & 0b1111;

            // Volume envelope
            let raw_envelope = registers.nr12 & 0b111;
            let envelope_time = (raw_envelope as f32) / 64f32 * (SOUND_CPU_SPEED as f32);

            if raw_envelope > 0 &&
                self.channel_1_step / envelope_time as u64 !=
                    (self.channel_1_step + cycles as u64) / envelope_time as u64 {

                if (registers.nr12 >> 3) & 0x1 == 0x1 {
                    if volume < 0xF {
                        volume += 1;
                    }
                } else {
                    if volume > 0 {
                        volume -= 1;
                    }
                }

                // Update presented volume
                registers.nr12 &= !(0b1111 << 4);
                registers.nr12 |= volume << 4;
            }

            // Generate the final wave for this channel
            register_1_wave = wave_pattern * ((volume as f32) / (0x0F as f32));

            self.channel_1_step += cycles as u64;

        }

        // Check if channel 2 should be reset
        if (registers.nr24 >> 7) & 0x1 == 1 {
            // Clear register
            registers.nr24 &= !(1 << 7);

            self.channel_2_running = true;
            self.channel_2_step = 0;
        }

        let mut register_2_wave = 0f32;

        // Handle channel 1
        if self.channel_2_running {
            // Step sound length if needed
            if (registers.nr24 >> 6) & 0x1 == 1 {
                //let new_step
                let mut current_sound_tick = registers.nr21 & 0b111111;

                if self.channel_2_step / SOUND_LENGTH_CLOCK_STEP as u64 !=
                    (self.channel_2_step + cycles as u64) / SOUND_LENGTH_CLOCK_STEP as u64 {
                    current_sound_tick += 1;

                    registers.nr21 &= !0b111111;
                    registers.nr21 |= current_sound_tick & 0b111111;
                }

                if current_sound_tick > 63 {
                    self.channel_2_running = false;
                }
            }

            // Get pattern duty
            let raw_wave_pattern_duty = (registers.nr21 >> 6) & 0b11;
            let low_percent = (raw_wave_pattern_duty as f32) * 0.125f32;

            // Get frequency
            let gb_frequency = (registers.nr23 as u32) | (((registers.nr24 & 0b111) as u32) << 8);
            let hz_frequency = 131072 / (2048 - gb_frequency);

            // Get pattern
            let wave_pattern = rectangle_wave(self.channel_2_step, hz_frequency, low_percent);

            // Get volume
            let mut volume = (registers.nr22 >> 4) & 0b1111;

            // Volume envelope
            let raw_envelope = registers.nr22 & 0b111;
            let envelope_time = (raw_envelope as f32) / 64f32 * (SOUND_CPU_SPEED as f32);

            if raw_envelope > 0 &&
                self.channel_2_step / envelope_time as u64 !=
                    (self.channel_2_step + cycles as u64) / envelope_time as u64 {

                if (registers.nr22 >> 3) & 0x1 == 0x1 {
                    if volume < 0xF {
                        volume += 1;
                    }
                } else {
                    if volume > 0 {
                        volume -= 1;
                    }
                }

                // Update presented volume
                registers.nr22 &= !(0b1111 << 4);
                registers.nr22 |= volume << 4;
            }

            // Generate the final wave for this channel
            register_2_wave = wave_pattern * ((volume as f32) / (0x0F as f32));

            self.channel_2_step += cycles as u64;

        }

        // Mix channels, check enable status
        let mut left_wave = 0f32;
        let mut right_wave = 0f32;

        if self.channel_1_running {
            if registers.nr51 & 0x1 == 0x1 {
                left_wave += register_1_wave / 100f32 / 2f32;
            }
            if (registers.nr51 >> 4) & 0x1 == 0x1 {
                right_wave += register_1_wave / 100f32 / 2f32;
            }
        }

        if self.channel_2_running {
            if (registers.nr51 >> 1) & 0x1 == 0x1 {
                left_wave += register_2_wave / 100f32 / 2f32;
            }
            if (registers.nr51 >> 5) & 0x1 == 0x1 {
                right_wave += register_2_wave / 100f32 / 2f32;
            }
        }

        // Final master volume
        left_wave *= (((registers.nr50 >> 4) & 0b111) as f32) / (0x0F as f32);
        right_wave *= ((registers.nr50 & 0b111) as f32) / (0x0F as f32);

        if ((self.last_cycle as f64 / SOUND_CPU_SPEED as f64) * OUTPUT_FREQUENCY as f64) as u64 !=
            (((self.last_cycle + cycles as u64) as f64 / SOUND_CPU_SPEED as f64) * OUTPUT_FREQUENCY as f64) as u64 {
            if self.sample_pointer + 2 <= self.samples.len() {
                self.samples[self.sample_pointer] = left_wave;
                self.samples[self.sample_pointer + 1] = right_wave;

                self.sample_pointer += 2;
            }
        }

        self.last_cycle += cycles as u64;
    }

    /// Drains all samples from this device.
    pub fn take_samples(&mut self) -> ([f32; FRAME_SIZE], usize) {
        let samples = self.samples;
        self.samples = [0f32; FRAME_SIZE];

        let old_pointer = self.sample_pointer;
        self.sample_pointer = 0;

        (samples, old_pointer)
    }

    pub fn build() -> Sound {
        Sound {
            channel_1_running : false,
            channel_1_step : 0,

            channel_2_running : false,
            channel_2_step : 0,

            samples : [0f32; FRAME_SIZE],
            sample_pointer: 0,
            last_cycle : 0,
        }
    }
}
