/**
 * bitrotation.rs
 *
 * Operations to rotate bits.
**/

use core::cpu::CPU;

/// **0x1F** - *RRA* - Rotate a right through Carry.
pub fn rra(cpu : &mut CPU) -> u8 {
    let value = cpu.regs.a;

    let old_flag = if cpu.regs.get_flag_c() {1 >> 7} else {0};

    cpu.regs.a = (value >> 1) | old_flag;

    cpu.regs.f = 0;
    cpu.regs.set_flag_c(value & 0x1 == 1);
    cpu.regs.set_flag_z(false);

    return 4 /* Cycles */;
}
