pub mod interrupts;
/**
 * cpu.rs
 *
 * Manages the primary CPU loop, as well as storing information about current state.
**/
pub mod regs;

mod instrs; // Private to the CPU implementation

use mem::GBMemory;

use cpu::instrs::execute_instruction;
use cpu::interrupts::InterruptType;
use cpu::regs::Registers;

#[cfg_attr(feature = "serialisation", derive(Serialize, Deserialize))]
pub struct CPU<'a> {
    pub regs: Registers,
    pub mem: GBMemory<'a>,
    pub interrupts_enabled: bool,
    pub interrupts_countdown: i8,
    pub stopped: bool,
    pub halted: bool,

    /// If the timer was high
    pub timer_armed: bool,
    pub timer_counter: i32,
    pub timer_enabled: bool,

    pub cycle_counter: u32,
    pub timer_invoke_counter: u32,
}

impl CPU<'_> {
    /// Ticks the CPU + other components one instruction.
    pub fn tick<#[cfg(feature = "debugger")] Debugger: GameboyDebugger>(
        &mut self,
        #[cfg(feature = "debugger")] debugger: &mut Debugger,
    ) -> bool {
        // Before tick
        if self.mem.dirty_interrupts {
            //self.mem.dirty_interrupts = false;
            let available_interrupts = self.mem.ioregs.iflag & self.mem.interrupt_reg;

            for bit in 0..5 {
                if (available_interrupts >> bit) & 0x1 == 1 {
                    let interrupt = InterruptType::get_by_bit(bit);
                    match interrupt {
                        Some(value) => {
                            if self.try_interrupt(value) {
                                break;
                            }
                        }
                        None => {
                            panic!("WARN: Unable to handle unknown interrupt");
                        }
                    }
                }
            }
        }

        let timer_active = (((self.mem.ioregs.tac >> 2) & 0x1) as u16) == 1;
        if timer_active {
            let tac = self.mem.ioregs.tac;
            let speed = tac & 0b11;
            let freq = match speed {
                0b00 => 4096,
                0b01 => 262144,
                0b10 => 65536,
                0b11 => 16384,
                _ => panic!("Bad clock speed: {}", speed),
            };

            if self.timer_counter > 4194304 / freq {
                self.timer_counter -= 4194304 / freq;

                if self.mem.ioregs.tima == 0xFF {
                    self.mem.ioregs.tima = self.mem.ioregs.tma;
                    self.throw_interrupt(InterruptType::TIMER);
                    self.timer_invoke_counter += 1;
                } else {
                    self.mem.ioregs.tima += 1;
                }
            }
        }

        // Main tick
        #[cfg(feature = "debugger")]
        debugger.debug(self);

        let cycles = if !self.stopped && !self.halted {
            // Read instruction
            let current_instr = self.regs.pc;

            let mut raw_instruction = self.mem.read(current_instr) as u16;

            //println!("{:02X} = {:02X}", current_instr, raw_instruction);

            self.regs.pc = self.regs.pc.wrapping_add(1);

            if raw_instruction == 0xCB {
                raw_instruction |= (self.mem.read(current_instr + 1) as u16) << 8;
                self.regs.pc = self.regs.pc.wrapping_add(1);
            }

            execute_instruction(self, raw_instruction, current_instr)
        } else {
            64 // TODO: Is this really the best?
        };

        // After
        // Handle interrupt toggle
        if self.interrupts_countdown > -1 {
            self.interrupts_countdown -= 1;

            if self.interrupts_countdown == -1 {
                self.interrupts_enabled = true;
            }
        }

        // Handle timers
        let cur_value = self.mem.ioregs.div;
        self.mem.ioregs.div = cur_value.wrapping_add(cycles as u16);
        self.cycle_counter += cycles as u32;

        if timer_active {
            self.timer_counter += cycles as i32;
        }

        // Handle audio
        self.mem.sound.step(&mut self.mem.ioregs, cycles);

        // Handle GPU
        let gpu_result = self.mem.gpu.step(cycles as u32);

        if let Some(value) = gpu_result {
            //println!("GPU throwing interrupt: {:?}", value);
            self.throw_interrupt(value);
            if value == InterruptType::VBLANK {
                return true;
            }
        }

        false
    }

    /// Runs a iteration of the CPU
    pub fn run<#[cfg(feature = "debugger")] Debugger: GameboyDebugger>(
        &mut self,
        #[cfg(feature = "debugger")] debugger: &mut Debugger,
    ) {
        self.cycle_counter = 0;
        self.timer_invoke_counter = 0;

        #[cfg(feature = "debugger")]
        while !self.tick(debugger) {}
        #[cfg(not(feature = "debugger"))]
        while !self.tick() {}
    }

    /// Registers that a interrupt should be thrown.
    pub fn throw_interrupt(&mut self, interrupt: InterruptType) -> bool {
        // Check to see if we are in a STOP event
        if self.stopped && interrupt != InterruptType::KEYPAD {
            return false;
        }

        //panic!("Throwing: {:?}", interrupt);

        // Set the IF flag
        self.mem.ioregs.iflag |= 1 << (interrupt as u8);
        self.mem.dirty_interrupts = true;

        true
    }

    /// Callback from memory to try to throw a memory interrupt.
    pub fn try_interrupt(&mut self, interrupt: InterruptType) -> bool {
        if !self.interrupts_enabled && !self.halted {
            return false;
        }

        // TODO: 20 cycle event
        self.halted = false; // TODO: extra 4 cycles if true
        self.stopped = false;

        if !self.interrupts_enabled {
            return true;
        }

        self.mem.ioregs.iflag &= !(1 << interrupt as u8);

        self.interrupts_enabled = false;

        //println!("Throwing interrupt: {:?}", interrupt);

        // Push PC to stack
        self.regs.sp -= 2;
        self.mem.write_short(self.regs.sp, self.regs.pc);

        // Jump to interrupt service
        self.regs.pc = match interrupt {
            InterruptType::VBLANK => 0x0040,
            InterruptType::LCDC => 0x0048,
            InterruptType::TIMER => 0x0050,
            InterruptType::SERIAL => 0x0058,
            InterruptType::KEYPAD => 0x0060,
        };

        true
    }

    /// Builds a CPU from the specified memory module.
    pub fn build(mem: GBMemory) -> CPU {
        CPU {
            regs: CPU::get_default_registers(),
            mem,
            interrupts_enabled: true,
            interrupts_countdown: -1,
            stopped: false,
            halted: false,
            timer_counter: 0,
            timer_enabled: false,
            timer_armed: false,
            cycle_counter: 0,
            timer_invoke_counter: 0,
        }
    }

    /// Returns the default expected state for the CPU registers.
    pub fn get_default_registers() -> Registers {
        Registers {
            a: 0x01,
            f: 0xB0,

            b: 0x00,
            c: 0x13,

            d: 0x00,
            e: 0xD8,

            h: 0x01,
            l: 0x4D,

            sp: 0xFFFE,
            pc: 0x0100,
        }
    }
}

pub trait GameboyDebugger {
    fn debug(&mut self, cpu: &mut CPU);
}
