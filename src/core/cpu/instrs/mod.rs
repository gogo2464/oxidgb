/**
 * instrs.rs
 *
 * The primary switchboard for CPU instructions.
**/

mod special;
mod jumps;

use core::cpu::CPU;

use core::cpu::instrs::special::*;
use core::cpu::instrs::jumps::*;

#[inline]
pub fn execute_instruction(cpu : &mut CPU, instr : u16, origin : u16) -> u8 {
    return match instr {
        0x00 => nop(cpu),
        0xC3 => jmp_nn(cpu),
        _ => {
            panic!("Unknown instruction: ${:02X} at ${:04X}", instr, origin);
        }
    }
}
