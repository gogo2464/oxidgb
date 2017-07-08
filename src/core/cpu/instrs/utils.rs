/**
 * utils.rs
 *
 * Helper utilities for instructions.
**/

use core::cpu::CPU;

#[inline]
pub fn get_n(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc = cpu.regs.pc.wrapping_add(1);
    return value;
}
