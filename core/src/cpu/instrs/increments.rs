use cpu::regs::Registers;
/**
 * increments.rs
 *
 * Incrementing/decrementing registers.
**/
use cpu::cpu::CPU;

// -- 8 bit increments. --

/// Handles flags for increments
#[inline]
fn inc_flags(x: u8, registers: &mut Registers) {
    registers.set_flag_z(x == 0);
    registers.set_flag_n(false);
    // TODO: Check flag H output.
    registers.set_flag_h(x & 0xf == 0);
}

/// **0x04** - *INC b* - Increment register b
pub fn inc_b(cpu: &mut CPU) -> u8 {
    cpu.regs.b = cpu.regs.b.wrapping_add(1);
    inc_flags(cpu.regs.b, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x0C** - *INC c* - Increment register c
pub fn inc_c(cpu: &mut CPU) -> u8 {
    cpu.regs.c = cpu.regs.c.wrapping_add(1);
    inc_flags(cpu.regs.c, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x14** - *INC d* - Increment register d
pub fn inc_d(cpu: &mut CPU) -> u8 {
    cpu.regs.d = cpu.regs.d.wrapping_add(1);
    inc_flags(cpu.regs.d, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x1C** - *INC e* - Increment register e
pub fn inc_e(cpu: &mut CPU) -> u8 {
    cpu.regs.e = cpu.regs.e.wrapping_add(1);
    inc_flags(cpu.regs.e, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x24** - *INC h* - Increment register h
pub fn inc_h(cpu: &mut CPU) -> u8 {
    cpu.regs.h = cpu.regs.h.wrapping_add(1);
    inc_flags(cpu.regs.h, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x2C** - *INC l* - Increment register l
pub fn inc_l(cpu: &mut CPU) -> u8 {
    cpu.regs.l = cpu.regs.l.wrapping_add(1);
    inc_flags(cpu.regs.l, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x3C** - *INC a* - Increment register a
pub fn inc_a(cpu: &mut CPU) -> u8 {
    cpu.regs.a = cpu.regs.a.wrapping_add(1);
    inc_flags(cpu.regs.a, &mut cpu.regs);
    4 /* Cycles */
}

/// Handles flags for decrements
#[inline]
fn dec_flags(x: u8, registers: &mut Registers) {
    registers.set_flag_z(x == 0);
    registers.set_flag_n(true);
    registers.set_flag_h(x & 0x0f == 0x0f);
}

/// **0x05** - *DEC b* - Decrement register b
pub fn dec_b(cpu: &mut CPU) -> u8 {
    cpu.regs.b = cpu.regs.b.wrapping_sub(1);
    dec_flags(cpu.regs.b, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x0D** - *DEC c* - Decrement register c
pub fn dec_c(cpu: &mut CPU) -> u8 {
    cpu.regs.c = cpu.regs.c.wrapping_sub(1);
    dec_flags(cpu.regs.c, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x15** - *DEC d* - Decrement register d
pub fn dec_d(cpu: &mut CPU) -> u8 {
    cpu.regs.d = cpu.regs.d.wrapping_sub(1);
    dec_flags(cpu.regs.d, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x1D** - *DEC e* - Decrement register e
pub fn dec_e(cpu: &mut CPU) -> u8 {
    cpu.regs.e = cpu.regs.e.wrapping_sub(1);
    dec_flags(cpu.regs.e, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x25** - *DEC h* - Decrement register h
pub fn dec_h(cpu: &mut CPU) -> u8 {
    cpu.regs.h = cpu.regs.h.wrapping_sub(1);
    dec_flags(cpu.regs.h, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x2D** - *DEC l* - Decrement register l
pub fn dec_l(cpu: &mut CPU) -> u8 {
    cpu.regs.l = cpu.regs.l.wrapping_sub(1);
    dec_flags(cpu.regs.l, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x3D** - *DEC a* - Decrement register a
pub fn dec_a(cpu: &mut CPU) -> u8 {
    cpu.regs.a = cpu.regs.a.wrapping_sub(1);
    dec_flags(cpu.regs.a, &mut cpu.regs);
    4 /* Cycles */
}

/// **0x34** - *INC (hl)* - Increment register \*hl
pub fn inc_phl(cpu: &mut CPU) -> u8 {
    let prev_value = cpu.mem.read(cpu.regs.get_hl());
    let new_value = prev_value.wrapping_add(1);
    cpu.mem.write(cpu.regs.get_hl(), new_value);

    cpu.regs.set_flag_z(new_value == 0);
    cpu.regs.set_flag_n(false);
    cpu.regs.set_flag_h(new_value & 0xf == 0);

    12 /* Cycles */
}

/// **0x35** - *DEC (hl)* - Decrement register \*hl
pub fn dec_phl(cpu: &mut CPU) -> u8 {
    let prev_value = cpu.mem.read(cpu.regs.get_hl());
    let new_value = prev_value.wrapping_sub(1);
    cpu.mem.write(cpu.regs.get_hl(), new_value);

    cpu.regs.set_flag_z(new_value == 0);
    cpu.regs.set_flag_n(true);
    cpu.regs.set_flag_h((new_value & 0x0f) == 0x0f);

    12 /* Cycles */
}

// -- 16 bit increments. --

/// **0x03** - *INC bc* - Increment register bc
pub fn inc_bc(cpu: &mut CPU) -> u8 {
    let value = cpu.regs.get_bc().wrapping_add(1);
    cpu.regs.set_bc(value);
    8 /* Cycles */
}

/// **0x0B** - *DEC bc* - Decrement register bc
pub fn dec_bc(cpu: &mut CPU) -> u8 {
    let value = cpu.regs.get_bc().wrapping_sub(1);
    cpu.regs.set_bc(value);
    8 /* Cycles */
}

/// **0x13** - *INC de* - Increment register de
pub fn inc_de(cpu: &mut CPU) -> u8 {
    let value = cpu.regs.get_de().wrapping_add(1);
    cpu.regs.set_de(value);
    8 /* Cycles */
}

/// **0x1B** - *DEC de* - Decrement register bc
pub fn dec_de(cpu: &mut CPU) -> u8 {
    let value = cpu.regs.get_de().wrapping_sub(1);
    cpu.regs.set_de(value);
    8 /* Cycles */
}

/// **0x23** - *INC hl* - Increment register hl
pub fn inc_hl(cpu: &mut CPU) -> u8 {
    let value = cpu.regs.get_hl().wrapping_add(1);
    cpu.regs.set_hl(value);
    8 /* Cycles */
}

/// **0x2B** - *DEC hl* - Decrement register hl
pub fn dec_hl(cpu: &mut CPU) -> u8 {
    let value = cpu.regs.get_hl().wrapping_sub(1);
    cpu.regs.set_hl(value);
    8 /* Cycles */
}

/// **0x33** - *INC sp* - Increment register hl
pub fn inc_sp(cpu: &mut CPU) -> u8 {
    let value = cpu.regs.sp.wrapping_add(1);
    cpu.regs.sp = value;
    8 /* Cycles */
}

/// **0x3B** - *DEC sp* - Decrement register hl
pub fn dec_sp(cpu: &mut CPU) -> u8 {
    let value = cpu.regs.sp.wrapping_sub(1);
    cpu.regs.sp = value;
    8 /* Cycles */
}
