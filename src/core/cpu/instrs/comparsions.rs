/**
 * bitrotation.rs
 *
 * Operations to compare registers/memory against each other.
**/

use core::cpu::CPU;
use core::cpu::regs::Registers;

/// Helper to compare registers
#[inline]
fn compare_registers(registers: &mut Registers, x : u8, y : u8) {
    let value = x.wrapping_sub(y);

    registers.set_flag_z(y == x);
    registers.set_flag_n(true);
    registers.set_flag_h(x & 0x0f < y & 0x0f);
    registers.set_flag_c(x < value);
}

/// **0xB8 ~ 0xBE** - *CP X* - Compares a with field X
pub fn cp(x : u8, cpu : &mut CPU) -> u8 {
    let y = cpu.regs.a;
    compare_registers(&mut cpu.regs, y, x);

    return 4 /* Cycles */;
}

/// **0xBE** - *CP (hl)* - Compare a with (hl)
pub fn cp_phl(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read(cpu.regs.get_hl());
    let y = cpu.regs.a;
    compare_registers(&mut cpu.regs, y, value);

    return 8 /* Cycles */;
}

/// **0xFE** - *CP #* - Compare a with #
pub fn cp_n(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;
    let y = cpu.regs.a;
    compare_registers(&mut cpu.regs, y, value);

    return 8 /* Cycles */;
}