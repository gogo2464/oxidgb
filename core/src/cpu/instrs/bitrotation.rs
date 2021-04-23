/**
 * bitrotation.rs
 *
 * Operations to rotate bits.
**/
use cpu::cpu::CPU;

/// Helper for RL operations.
#[inline]
fn rl_helper(cpu: &mut CPU, value: u8) -> u8 {
    let old_flag = if cpu.regs.get_flag_c() { 1 } else { 0 };

    let result = ((value << 1) & !1) | old_flag;

    cpu.regs.f = 0;
    cpu.regs.set_flag_c((value >> 7) & 0x1 == 1);
    cpu.regs.set_flag_z(result == 0);

    result
}

/// **0x17** - *RLA* - Rotate a left through Carry.
pub fn rla(cpu: &mut CPU) -> u8 {
    let current_value = cpu.regs.a;
    cpu.regs.a = rl_helper(cpu, current_value);
    cpu.regs.set_flag_z(false);

    4 /* Cycles */
}

/// **0xCB 0x16** - *RL (hl)* - Rotate (hl) left through Carry.
pub fn rl_phl(cpu: &mut CPU) -> u8 {
    let current_value = cpu.mem.read(cpu.regs.get_hl());
    let result = rl_helper(cpu, current_value);
    cpu.mem.write(cpu.regs.get_hl(), result);

    16 /* Cycles */
}

macro_rules! rl {
    ($func:ident, $reg:ident) => {
        pub fn $func(cpu: &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            cpu.regs.$reg = rl_helper(cpu, current_value);
            8 /* Cycles */
        }
    };
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
fn rr_helper(cpu: &mut CPU, value: u8) -> u8 {
    let old_flag = if cpu.regs.get_flag_c() { 1 << 7 } else { 0 };

    let result = (value >> 1) | old_flag;

    cpu.regs.f = 0;
    cpu.regs.set_flag_c(value & 0x1 == 1);
    cpu.regs.set_flag_z(result == 0);

    result
}

/// **0x1F** - *RRA* - Rotate a right through Carry.
pub fn rra(cpu: &mut CPU) -> u8 {
    let current_value = cpu.regs.a;
    cpu.regs.a = rr_helper(cpu, current_value);
    cpu.regs.set_flag_z(false);

    4 /* Cycles */
}

/// **0xCB 0x1E** - *RR (hl)* - Rotate (hl) right through Carry.
pub fn rr_phl(cpu: &mut CPU) -> u8 {
    let current_value = cpu.mem.read(cpu.regs.get_hl());
    let result = rr_helper(cpu, current_value);
    cpu.mem.write(cpu.regs.get_hl(), result);

    16 /* Cycles */
}

macro_rules! rr {
    ($func:ident, $reg:ident) => {
        pub fn $func(cpu: &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            cpu.regs.$reg = rr_helper(cpu, current_value);
            8 /* Cycles */
        }
    };
}

rr!(rr_b, b);
rr!(rr_c, c);
rr!(rr_d, d);
rr!(rr_e, e);
rr!(rr_h, h);
rr!(rr_l, l);
rr!(rr_a, a);

/// Helper for RLC operations.
#[inline]
fn rlc_helper(cpu: &mut CPU, value: u8) -> u8 {
    let result = ((value << 1) & !1) | ((value >> 7) & 0x1);

    cpu.regs.f = 0;
    cpu.regs.set_flag_c((value >> 7) & 0x1 == 1);
    cpu.regs.set_flag_z(result == 0);

    result
}

/// **0x07** - *RLCA* - Rotate a left. Bit 7 into Carry.
pub fn rlca(cpu: &mut CPU) -> u8 {
    let current_value = cpu.regs.a;
    cpu.regs.a = rlc_helper(cpu, current_value);
    cpu.regs.set_flag_z(false);

    4 /* Cycles */
}

/// **0xCB 0x06** - *RLC (hl)* - Rotate a left. Bit 7 into Carry.
pub fn rlc_phl(cpu: &mut CPU) -> u8 {
    let current_value = cpu.mem.read(cpu.regs.get_hl());
    let result = rlc_helper(cpu, current_value);
    cpu.mem.write(cpu.regs.get_hl(), result);

    16 /* Cycles */
}

macro_rules! rlc {
    ($func:ident, $reg:ident) => {
        pub fn $func(cpu: &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            cpu.regs.$reg = rlc_helper(cpu, current_value);
            8 /* Cycles */
        }
    };
}

rlc!(rlc_b, b);
rlc!(rlc_c, c);
rlc!(rlc_d, d);
rlc!(rlc_e, e);
rlc!(rlc_h, h);
rlc!(rlc_l, l);
rlc!(rlc_a, a);

/// Helper for RRC operations.
#[inline]
fn rrc_helper(cpu: &mut CPU, value: u8) -> u8 {
    let result = ((value >> 1) & !(1 << 7)) | ((value & 1) << 7);

    cpu.regs.f = 0;
    cpu.regs.set_flag_c(value & 0x1 == 1);
    cpu.regs.set_flag_z(result == 0);

    result
}

/// **0x07** - *RRCA* - Rotate a left. Bit 7 into Carry.
pub fn rrca(cpu: &mut CPU) -> u8 {
    let current_value = cpu.regs.a;
    cpu.regs.a = rrc_helper(cpu, current_value);
    cpu.regs.set_flag_z(false);

    4 /* Cycles */
}

/// **0xCB 0x0E** - *RRC (hl)* - Rotate (hl) right. Bit 0 into Carry.
pub fn rrc_phl(cpu: &mut CPU) -> u8 {
    let current_value = cpu.mem.read(cpu.regs.get_hl());
    let result = rrc_helper(cpu, current_value);
    cpu.mem.write(cpu.regs.get_hl(), result);

    16 /* Cycles */
}

macro_rules! rrc {
    ($func:ident, $reg:ident) => {
        pub fn $func(cpu: &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            cpu.regs.$reg = rrc_helper(cpu, current_value);
            8 /* Cycles */
        }
    };
}

rrc!(rrc_b, b);
rrc!(rrc_c, c);
rrc!(rrc_d, d);
rrc!(rrc_e, e);
rrc!(rrc_h, h);
rrc!(rrc_l, l);
rrc!(rrc_a, a);
