/**
 * cpu.rs
 *
 * Manages the primary CPU loop, as well as storing information about current state.
**/

pub mod regs;

mod instrs; // Private to the CPU implementation

use core::mem::GBMemory;

use core::cpu::regs::Registers;
use core::cpu::instrs::execute_instruction;

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
    pub fn tick(&mut self) {
        // Read instruction
        let current_instr = self.regs.pc;

        let mut raw_instruction = self.mem.read(current_instr) as u16;

        self.regs.pc += 1;

        if raw_instruction == 0xCB {
            raw_instruction = (raw_instruction << 8) | (self.mem.read(current_instr + 1) as u16);
            self.regs.pc += 1;
        }

        println!("Read instruction: {:04x}", raw_instruction);

        execute_instruction(self, raw_instruction, current_instr);
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