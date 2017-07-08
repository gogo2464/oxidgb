/**
 * cpu.rs
 *
 * Manages the primary CPU loop, as well as storing information about current state.
**/

pub mod regs;
pub mod interrupts;

mod instrs; // Private to the CPU implementation

use core::mem::GBMemory;

use core::cpu::regs::Registers;
use core::cpu::instrs::execute_instruction;
use core::cpu::interrupts::InterruptType;

pub struct CPU {
    pub regs : Registers,
    pub mem : GBMemory,
    pub interrupts_enabled : bool,
    pub interrupts_countdown : i8,
    pub stopped : bool,
    pub halted : bool
}

impl CPU {
    /// Ticks the CPU + other components one instruction.
    pub fn tick(&mut self) -> bool {
        // Read instruction
        let current_instr = self.regs.pc;

        let mut raw_instruction = self.mem.read(current_instr) as u16;

        //println!("{:02X} = {:02X}", current_instr, raw_instruction);

        self.regs.pc = self.regs.pc.wrapping_add(1);

        if raw_instruction == 0xCB {
            raw_instruction = (raw_instruction << 8) | (self.mem.read(current_instr + 1) as u16);
            self.regs.pc = self.regs.pc.wrapping_add(1);
        }

        /*println!("Read instruction: {:04x}", raw_instruction);
        println!("af = {:04x}", self.regs.get_af());
        println!("bc = {:04x}", self.regs.get_bc());
        println!("de = {:04x}", self.regs.get_de());
        println!("hl = {:04x}", self.regs.get_hl());
        println!("sp = {:04x}", self.regs.sp);
        println!("pc = {:04x}", self.regs.pc);*/

        let cycles = execute_instruction(self, raw_instruction, current_instr);

        // After
        let gpu_result = self.mem.gpu.step(cycles as u32);

        if gpu_result {
            // TODO: Don't actually use this pattern
            self.throw_interrupt(InterruptType::VBLANK);
        }

        return gpu_result;
    }

    /// Runs a iteration of the CPU
    pub fn run(&mut self) {
        while !self.tick() {}
    }

    /// Immediately throws a interrupt.
    pub fn throw_interrupt(&mut self, interrupt : InterruptType) {
        if !self.interrupts_enabled {
            //println!("Unable to throw interrupt when it is not active!");
            return;
        }

        self.interrupts_enabled = false;

        // Push PC to stack
        self.regs.sp -= 2;
        self.mem.write_short(self.regs.sp, self.regs.pc);

        // Jump to interrupt service
        self.regs.pc = match interrupt {
            InterruptType::VBLANK => 0x0040,
            InterruptType::LCDC   => 0x0048,
            InterruptType::TIMER  => 0x0050,
            InterruptType::SERIAL => 0x0058,
            InterruptType::KEYPAD => 0x0060
        }
    }

    /// Builds a CPU from the specified memory module.
    pub fn build(mem : GBMemory) -> CPU {
        return CPU {
            regs : CPU::get_default_registers(),
            mem : mem,
            interrupts_enabled : true,
            interrupts_countdown : -1,
            stopped : false,
            halted : false
        }
    }

    /// Returns the default expected state for the CPU registers.
    pub fn get_default_registers() -> Registers {
        return Registers {
            a: 0x01,
            f: 0xB0,

            b: 0x00,
            c: 0x13,

            d: 0x00,
            e: 0xD8,

            h: 0x01,
            l: 0x4D,

            sp : 0xFFFE,
            pc : 0x0100
        }
    }
}