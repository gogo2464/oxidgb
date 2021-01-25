/**
 * loads.rs
 *
 * Instructions to store information into registers/memory.
**/
use cpu::CPU;
use mem::GBMemory;

// 8 bit loads

/// **0x02** - *LD (xx),y* - Put y in \*xx
pub fn ld_pxx_x(xx: u16, y: u8, cpu: &mut CPU) -> u8 {
    cpu.mem.write(xx, y);
    8 /* Cycles */
}

/// **0x06** - *LD b,#* - Put # in b
pub fn ld_b_n(cpu: &mut CPU) -> u8 {
    cpu.regs.b = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;

    8 /* Cycles */
}

/// **0x0A** - *LD n,(xx)* - Put (xx) in n
pub fn ld_n_pxx(mem: &GBMemory, xx: u16, n: &mut u8) -> u8 {
    *n = mem.read(xx);
    8 /* Cycles */
}

/// **0x0E** - *LD c,#* - Put # in c
pub fn ld_c_n(cpu: &mut CPU) -> u8 {
    cpu.regs.c = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;

    8 /* Cycles */
}

/// **0x16** - *LD d,#* - Put # in d
pub fn ld_d_n(cpu: &mut CPU) -> u8 {
    cpu.regs.d = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;

    8 /* Cycles */
}

/// **0x1E** - *LD e,#* - Put # in e
pub fn ld_e_n(cpu: &mut CPU) -> u8 {
    cpu.regs.e = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;

    8 /* Cycles */
}

/// **0x22** - *LDI (hl),a* - Put a into \*hl. Increment hl.
pub fn ldi_phl_a(cpu: &mut CPU) -> u8 {
    cpu.mem.write(cpu.regs.get_hl(), cpu.regs.a);
    let new_value = cpu.regs.get_hl().wrapping_add(1);
    cpu.regs.set_hl(new_value);

    8 /* Cycles */
}

/// **0x26** - *LD h,#* - Put # in h
pub fn ld_h_n(cpu: &mut CPU) -> u8 {
    cpu.regs.h = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;

    8 /* Cycles */
}

/// **0x2A** - *LDI a,(hl)* - Put \*hl into a. Increment hl.
pub fn ldi_a_phl(cpu: &mut CPU) -> u8 {
    cpu.regs.a = cpu.mem.read(cpu.regs.get_hl());
    let new_value = cpu.regs.get_hl() + 1;
    cpu.regs.set_hl(new_value);

    8 /* Cycles */
}

/// **0x2E** - *LD l,#* - Put # in l
pub fn ld_l_n(cpu: &mut CPU) -> u8 {
    cpu.regs.l = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;

    8 /* Cycles */
}

/// **0x32** - *LDD (hl),a* - Put a into \*hl. Decrement hl.
pub fn ldd_phl_a(cpu: &mut CPU) -> u8 {
    cpu.mem.write(cpu.regs.get_hl(), cpu.regs.a);
    let new_value = cpu.regs.get_hl() - 1;
    cpu.regs.set_hl(new_value);

    8 /* Cycles */
}

/// **0x36** - *LD (hl),n* - Put n in \*hl
pub fn ld_phl_n(cpu: &mut CPU) -> u8 {
    let new_value = cpu.mem.read(cpu.regs.pc);
    cpu.mem.write(cpu.regs.get_hl(), new_value);
    cpu.regs.pc += 1;

    12 /* Cycles */
}

/// **0x3A** - *LDD a,(hl)* - Put \*hl into a. Decrement hl.
pub fn ldd_a_phl(cpu: &mut CPU) -> u8 {
    cpu.regs.a = cpu.mem.read(cpu.regs.get_hl());
    let new_value = cpu.regs.get_hl() - 1;
    cpu.regs.set_hl(new_value);

    8 /* Cycles */
}

/// **0x3E** - *LD l,#* - Put # in a
pub fn ld_a_n(cpu: &mut CPU) -> u8 {
    cpu.regs.a = cpu.mem.read(cpu.regs.pc);
    cpu.regs.pc += 1;

    8 /* Cycles */
}

/// **0x40** - *LD x,y* - Put y in x
pub fn ld_x_y(y: u8, x: &mut u8) -> u8 {
    *x = y;
    4 /* Cycles */
}

/// **0x46** - *LD x,(hl)* - Put (hl) in x
pub fn ld_x_phl(hl: u16, mem: &GBMemory, x: &mut u8) -> u8 {
    *x = mem.read(hl);
    4 /* Cycles */
}

/// **0x70** - *LD (hl),x* - Put x in \*hl
pub fn ld_phl_x(x: u8, cpu: &mut CPU) -> u8 {
    cpu.mem.write(cpu.regs.get_hl(), x);
    8 /* Cycles */
}
