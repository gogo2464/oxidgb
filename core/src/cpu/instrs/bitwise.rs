/**
 * bitwise.rs
 *
 * Bitwise operations.
**/

use cpu::CPU;
use cpu::regs::Registers;

/// Helper to XOR something into A.
#[inline]
fn reg_xor(registers : &mut Registers, y_val: u8) {
    let x_val = registers.a;
    let result = y_val ^ x_val;
    registers.a = result;

    registers.f = 0;
    registers.set_flag_z(result == 0);
}

/// **0xA8 ~ 0xAF** - *XOR X* - Xor X with a into a
pub fn xor(x : u8, cpu : &mut CPU) -> u8 {
    reg_xor(&mut cpu.regs, x);

    return 4 /* Cycles */;
}

/**
 * **0xAE** - *XOR (hl)* - Xor \*hl with a into a
 */
pub fn xor_hl(cpu : &mut CPU) -> u8 {
    // TODO: Check that this is correct
    let value = cpu.mem.read(cpu.regs.get_hl());
    reg_xor(&mut cpu.regs, value);

    return 8 /* Cycles */;
}

/**
 * **0xEE** - *XOR #* - Xor # with a into a
 */
pub fn xor_n(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;
    reg_xor(&mut cpu.regs, value);

    return 8 /* Cycles */;
}

/**
 * -- ORs. --
 */

/**
 * Helper to OR something into A.
 */
#[inline]
fn reg_or(registers : &mut Registers, y_val: u8) {
    let x_val = registers.a;
    let result = y_val | x_val;
    registers.a = result;

    registers.f = 0;
    registers.set_flag_z(result == 0);
}

/**
 * **0xB0 ~ 0xB7** - *OR X* - Or X with a into a
 */
pub fn or(x : u8, cpu : &mut CPU) -> u8 {
    reg_or(&mut cpu.regs, x);

    return 4 /* Cycles */;
}

/**
 * **0xB6** - *OR (hl)* - Or *hl with a into a
 */
pub fn or_phl(cpu : &mut CPU) -> u8 {
    // TODO: Check these *hl's
    let value = cpu.mem.read(cpu.regs.get_hl());
    reg_or(&mut cpu.regs, value);

    return 8 /* Cycles */;
}

/**
 * **0xF6** - *OR #* - or # with a into a
 */
pub fn or_n(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;
    reg_or(&mut cpu.regs, value);

    return 8 /* Cycles */;
}

/**
 * -- ANDs. --
 */

/**
 * Helper to AND something into A.
 */
#[inline]
fn reg_and(registers : &mut Registers, y_val: u8) {
    let x_val = registers.a;
    let result = x_val & y_val;
    registers.a = result;

    registers.f = 0;
    registers.set_flag_z(result == 0);
    registers.set_flag_h(true);
}

/**
 * **0xA0 ~ 0xA7** - *AND X* - AND X with a into a
 */
pub fn and(x : u8, cpu : &mut CPU) -> u8 {
    reg_and(&mut cpu.regs, x);

    return 4 /* Cycles */;
}

/**
 * **0xA6** - *AND (hl)* - And (hl) with a into a
 */
pub fn and_phl(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read(cpu.regs.get_hl());
    reg_and(&mut cpu.regs, value);

    return 8 /* Cycles */;
}

/**
 * **0xE6** - *AND #* - And # with a into a
 */
pub fn and_n(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;
    reg_and(&mut cpu.regs, value);

    return 8 /* Cycles */;
}
