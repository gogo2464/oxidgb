/**
 * bitrotation.rs
 *
 * Operations to rotate bits.
**/

use core::cpu::CPU;

/// Helper for RL operations.
#[inline]
fn rl_helper(cpu : &mut CPU, value : u8) -> u8 {
    let old_flag = if cpu.regs.get_flag_c() {1 << 7} else {0};

    let result = ((value << 1) & !1) | old_flag;

    cpu.regs.f = 0;
    cpu.regs.set_flag_c((value >> 7) & 0x1 == 1);
    cpu.regs.set_flag_z(result == 0);

    return result;
}

/// **0x1F** - *RLA* - Rotate a left through Carry.
pub fn rla(cpu : &mut CPU) -> u8 {
    let current_value = cpu.regs.a;
    cpu.regs.a = rl_helper(cpu, current_value);
    cpu.regs.set_flag_z(false);

    return 4 /* Cycles */;
}

macro_rules! rl {
    ($func:ident, $reg:ident) => (
        pub fn $func(cpu : &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            cpu.regs.$reg = rl_helper(cpu, current_value);
            return 8 /* Cycles */;
        }
    )
}

rl!(rl_b, b);
rl!(rl_c, c);
rl!(rl_d, d);
rl!(rl_e, e);
rl!(rl_h, h);
rl!(rl_l, l);
rl!(rl_a, a);

/// Helper for RR operations.
#[inline]
fn rr_helper(cpu : &mut CPU, value : u8) -> u8 {
    let old_flag = if cpu.regs.get_flag_c() {1 << 7} else {0};

    let result = (value >> 1) | old_flag;

    cpu.regs.f = 0;
    cpu.regs.set_flag_c(value & 0x1 == 1);
    cpu.regs.set_flag_z(result == 0);

    return result;
}

/// **0x1F** - *RRA* - Rotate a right through Carry.
pub fn rra(cpu : &mut CPU) -> u8 {
    let current_value = cpu.regs.a;
    cpu.regs.a = rr_helper(cpu, current_value);
    cpu.regs.set_flag_z(false);

    return 4 /* Cycles */;
}

macro_rules! rr {
    ($func:ident, $reg:ident) => (
        pub fn $func(cpu : &mut CPU) -> u8 {
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
