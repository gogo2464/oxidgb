/**
 * loads16.rs
 *
 * Instructions to store information into registers/memory.
**/

use core::cpu::CPU;

/// **0x01** - *LD bc,nnnn* - Put nnnn in bc
pub fn ld_bc_nnnn(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read_short(cpu.regs.pc);
    cpu.regs.set_bc(value);
    cpu.regs.pc += 2;

    return 12 /* Cycles */;
}

/// **0x08** - *LD (nn),sp* - Put sp at n
pub fn ld_pnn_sp(cpu : &mut CPU) -> u8 {
    let pointer = cpu.mem.read_short(cpu.regs.pc);
    cpu.mem.write_short(pointer, cpu.regs.sp);
    cpu.regs.pc += 2;

    return 20 /* Cycles */;
}

/// **0x11** - *LD de,nn* - Put nn in de
pub fn ld_de_nn(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read_short(cpu.regs.pc);
    cpu.regs.set_de(value);
    cpu.regs.pc += 2;

    return 12 /* Cycles */;
}

/// **0x21** - *LD hl,nnnn* - Put nnnn in hl
pub fn ld_hl_nnnn(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read_short(cpu.regs.pc);
    cpu.regs.set_hl(value);
    cpu.regs.pc += 2;

    return 12 /* Cycles */;
}

/// **0x31** - *LD sp,nn* - Put nn in sp
pub fn ld_sp_nn(cpu : &mut CPU) -> u8 {
    cpu.regs.sp = cpu.mem.read_short(cpu.regs.pc);
    cpu.regs.pc += 2;

    return 12 /* Cycles */;
}

/// **0xC1** - *POP bc* - Pop stack element into bc
pub fn pop_bc(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read_short(cpu.regs.sp);
    cpu.regs.set_bc(value);
    cpu.regs.sp += 2;

    return 12 /* Cycles */;
}

/// **0xC5** - *PUSH bc* - Push bc onto the stack
pub fn push_bc(cpu : &mut CPU) -> u8 {
    cpu.regs.sp -= 2;
    cpu.mem.write_short(cpu.regs.sp, cpu.regs.get_bc());

    return 16 /* Cycles */;
}

/// **0xD1** - *POP de* - Pop stack element into de
pub fn pop_de(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read_short(cpu.regs.sp);
    cpu.regs.set_de(value);
    cpu.regs.sp += 2;

    return 12 /* Cycles */;
}

/// **0xD5** - *PUSH de* - Push de onto the stack
pub fn push_de(cpu : &mut CPU) -> u8 {
    cpu.regs.sp -= 2;
    cpu.mem.write_short(cpu.regs.sp, cpu.regs.get_de());

    return 16 /* Cycles */;
}

/// **0xE0** - *LDH (n),a* - Put a in memory address *($FF00+n)
pub fn ldh_pn_a(cpu : &mut CPU) -> u8 {
    let value = 0xFF00 + (cpu.mem.read(cpu.regs.pc) as u16);
    cpu.regs.pc += 1;
    cpu.mem.write(value, cpu.regs.a);

    return 12 /* Cycles */;
}

/// **0xE1** - *POP hl* - Pop stack element into hl
pub fn pop_hl(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read_short(cpu.regs.sp);
    cpu.regs.set_hl(value);
    cpu.regs.sp += 2;

    return 12 /* Cycles */;
}

/// **0xE2** - *LD (c),a* - Put a in memory address *($FF00+c)
pub fn ld_pc(cpu : &mut CPU) -> u8 {
    let value = 0xFF00 + ((cpu.regs.c as u16) & 0xFF);
    cpu.mem.write(value, cpu.regs.a);

    return 8 /* Cycles */;
}

/// **0xE5** - *PUSH hl* - Push hl onto the stack
pub fn push_hl(cpu : &mut CPU) -> u8 {
    cpu.regs.sp -= 2;
    cpu.mem.write_short(cpu.regs.sp, cpu.regs.get_hl());

    return 16 /* Cycles */;
}

/// **0xEA** - *LD (nn),a* - Put a in \*nn
pub fn ld_pnn_a(cpu : &mut CPU) -> u8 {
    // Read PC short
    let value = cpu.mem.read_short(cpu.regs.pc);
    cpu.regs.pc += 2;
    // Write it
    cpu.mem.write(value, cpu.regs.a);

    return 16 /* Cycles */;
}

/// **0xF0** - *LDH a,(n)* - Put memory address *($FF00+n) in A
pub fn ldh_a_pn(cpu : &mut CPU) -> u8 {
    let value = 0xFF00 + ((cpu.mem.read(cpu.regs.pc) as u16) & 0xFF);
    //println("New value: $value = ${cpu.mem.read(value)}")
    cpu.regs.a = cpu.mem.read(value);
    cpu.regs.pc += 1;

    return 12 /* Cycles */;
}

/// **0xF1** - *POP af* - Pop stack element into af
pub fn pop_af(cpu : &mut CPU) -> u8 {
    let value = cpu.mem.read_short(cpu.regs.sp);
    cpu.regs.set_af(value);
    cpu.regs.sp += 2;

    return 12 /* Cycles */;
}

/// **0xF2** - *LD a,(c)* - Put *($FF00+c) into a
pub fn ld_a_ptrc(cpu : &mut CPU) -> u8 {
    let value = 0xFF00 + ((cpu.regs.c as u16) & 0xFF);
    cpu.regs.a = cpu.mem.read(value);

    return 8 /* Cycles */;
}

/// **0xF5** - *PUSH af* - Push af onto the stack
pub fn push_af(cpu : &mut CPU) -> u8 {
    cpu.regs.sp -= 2;
    cpu.mem.write_short(cpu.regs.sp, cpu.regs.get_af());

    return 16 /* Cycles */;
}

/// **0xF8** - *LDHL SP,n* - Put sp + n effective address into hl
pub fn ldhl_sp_n(cpu : &mut CPU) -> u8 {
    let prev_value = cpu.regs.sp;
    let cur_value = cpu.mem.read(cpu.regs.pc & 0xFFFF) as u16;
    cpu.regs.pc += 1;

    let result = prev_value + cur_value;

    cpu.regs.set_hl(result);

    cpu.regs.set_flag_z(false);
    cpu.regs.set_flag_n(false);

    cpu.regs.set_flag_h((prev_value ^ cur_value ^ (result & 0xFFFF) & 0x10) == 0x10);
    cpu.regs.set_flag_c((prev_value ^ cur_value ^ (result & 0xFFFF) & 0x100) == 0x100);

    return 12 /* Cycles */;
}

/// **0xF9** - *LD sp,hl* - Put hl in sp
pub fn ld_sp_hl(cpu : &mut CPU) -> u8 {
    cpu.regs.sp = cpu.regs.get_hl();

    return 8 /* Cycles */;
}

/// **0xFA** - *LD a,(nn)* - Read \*nn into a
pub fn ld_a_pnn(cpu : &mut CPU) -> u8 {
    // TODO: WTF?
    cpu.regs.a = cpu.mem.read((cpu.mem.read_short(cpu.regs.pc) & 0xFFFF));
    cpu.regs.pc += 2;

    return 16 /* Cycles */;
}
