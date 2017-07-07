/**
 * special.rs
 *
 * Operations to branch code.
**/

use core::cpu::CPU;

/// **0xC3** - *JMP nn* - Jump to address nn (two byte)
pub fn jmp_nn(cpu : &mut CPU) -> u8 {
    let new_ptr = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc = new_ptr as u16;
    return 16 /* Cycles */;
}
