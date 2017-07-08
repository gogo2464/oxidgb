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

/// Helper for RR operations.
#[inline]
fn rr_helper(cpu : &mut CPU, value : u8) -> u8 {
    let old_flag = if cpu.regs.get_flag_c() {1 >> 7} else {0};

    let result = (value >> 1) | old_flag;

    cpu.regs.f = 0;
    cpu.regs.set_flag_c(value & 0x1 == 1);
    cpu.regs.set_flag_z(result == 0);

    return result;
}

macro_rules! rr {
    ($func:ident, $reg:ident) => (
        fn $func(cpu : &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            cpu.regs.$reg = rr_helper(cpu, current_value);
            return 8 /* Cycles */;
        }
    )
}

rr!(rr_b, b);
rr!(rr_c, c);
rr!(rr_d, d);
rr!(rr_e, e);
rr!(rr_h, h);
rr!(rr_l, l);
rr!(rr_a, a);
