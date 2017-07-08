/**
 * special.rs
 *
 * Operations to branch code.
**/

use core::cpu::CPU;

/// **0x18** - *JR n* - Jump to pc+n
pub fn jr_n(cpu : &mut CPU) -> u8 {
    let n = cpu.mem.read(cpu.regs.pc) as i8 as i16;
    cpu.regs.pc = ((cpu.regs.pc as i16).wrapping_add(n.wrapping_add(1))) as u16 /* +1 for n size */;

    // TODO
    /*
        if (cpu.regs.pc + 1 + pointer == cpu.regs.pc - 1 && !cpu.interruptsEnabled) {
            cpu.infiniteLoop = true
        }
    */

    return 12 /* Cycles */;
}

/// **0x20** - *JR NZ,n* - Jump if Z flag is reset
pub fn jr_nz_n(cpu : &mut CPU) -> u8 {
    if !cpu.regs.get_flag_z() {
        let n = cpu.mem.read(cpu.regs.pc) as i8 as i16;
        cpu.regs.pc = ((cpu.regs.pc as i16).wrapping_add(n.wrapping_add(1))) as u16 /* +1 for n size */;
        return 12 /* Cycles */;
    } else {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(1);
        return 8 /* Cycles */;
    }
}

/// **0x28** - *JR Z,n* - Jump if Z flag is set
pub fn jr_z_n(cpu : &mut CPU) -> u8 {
    if cpu.regs.get_flag_z() {
        let n = cpu.mem.read(cpu.regs.pc) as i8 as i16;
        cpu.regs.pc = ((cpu.regs.pc as i16).wrapping_add(n.wrapping_add(1))) as u16 /* +1 for n size */;
        return 12 /* Cycles */;
    } else {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(1);
        return 8 /* Cycles */;
    }
}

/// **0x30** - *JR NC,n* - Jump if C flag is reset
pub fn jr_nc_n(cpu : &mut CPU) -> u8 {
    if !cpu.regs.get_flag_c() {
        let n = cpu.mem.read(cpu.regs.pc) as i8 as i16;
        cpu.regs.pc = ((cpu.regs.pc as i16).wrapping_add(n.wrapping_add(1))) as u16 /* +1 for n size */;
        return 12 /* Cycles */;
    } else {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(1);
        return 8 /* Cycles */;
    }
}

/// **0x38** - *JR C,n* - Jump if C flag is set
pub fn jr_c_n(cpu : &mut CPU) -> u8 {
    if cpu.regs.get_flag_c() {
        let n = cpu.mem.read(cpu.regs.pc) as i8 as i16;
        cpu.regs.pc = ((cpu.regs.pc as i16).wrapping_add(n.wrapping_add(1))) as u16 /* +1 for n size */;
        return 12 /* Cycles */;
    } else {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(1);
        return 8 /* Cycles */;
    }
}

/// **0xC2** - *JP NZ,nn* - Jump to address nn (two byte) if Z flag is reset
pub fn jp_nz_nn(cpu : &mut CPU) -> u8 {
    if !cpu.regs.get_flag_z() {
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);
        return 16 /* Cycles */;
    } else {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(2);
        return 12 /* Cycles */;
    }
}

/// **0xC3** - *JMP nn* - Jump to address nn (two byte)
pub fn jmp_nn(cpu : &mut CPU) -> u8 {
    cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);
    
    return 16 /* Cycles */;
}

/// **0xD2** - *JP NC,nn* - Jump to address nn (two byte) if C flag is reset
pub fn jp_nc_nn(cpu : &mut CPU) -> u8 {
    if !cpu.regs.get_flag_c() {
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);

        return 16 /* Cycles */;
    } else {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(2);
        
        return 12 /* Cycles */;
    }
}

/// **0xDA** - *JP C,nn* - Jump to address nn (two byte) if C flag is set
pub fn jp_c_nn(cpu : &mut CPU) -> u8 {
    if cpu.regs.get_flag_c() {
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);
        
        return 16 /* Cycles */;
    } else {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(2);
        
        return 12 /* Cycles */;
    }
}

/// -- Calls. --

/// **0xC4** - *CALL NZ,nn* - If Z is false jump to address nn and store current pc in stack
pub fn call_nz_nn(cpu : &mut CPU) -> u8 {
    if !cpu.regs.get_flag_z() {
        cpu.regs.sp = cpu.regs.sp.wrapping_sub(2);
        cpu.mem.write_short(cpu.regs.sp, cpu.regs.pc.wrapping_add(2));
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);
        
        return 24 /* Cycles */;
    } else {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(2);
        
        return 12 /* Cycles */;
    }
}

/// **0xCA** - *JP Z,nn* - Jump to address nn (two byte) if Z flag is set
pub fn jp_z_nn(cpu : &mut CPU) -> u8 {
    if cpu.regs.get_flag_z() {
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);

        return 16 /* Cycles */;
    } else {
        cpu.regs.pc = cpu.regs.pc.wrapping_add(2);

        return 12 /* Cycles */;
    }
}

/// **0xCC** - *CALL Z,nn* - If Z is true jump to address nn and store current pc in stack
pub fn call_z_nn(cpu : &mut CPU) -> u8 {
    if cpu.regs.get_flag_z() {
        cpu.regs.sp -= 2;
        cpu.mem.write_short(cpu.regs.sp, cpu.regs.pc + 2);
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);
        
        return 24 /* Cycles */;
    } else {
        cpu.regs.pc += 2;
        
        return 12 /* Cycles */;
    }
}

/// **0xCD** - *CALL nn* - Jump to address nn and store current pc in stack
pub fn call_nn(cpu : &mut CPU) -> u8 {
    cpu.regs.sp -= 2;
    cpu.mem.write_short(cpu.regs.sp, cpu.regs.pc + 2);
    cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);
    
    return 24 /* Cycles */;
}

/// **0xD4** - *CALL NC,nn* - If N is false jump to address nn and store current pc in stack
pub fn call_nc_nn(cpu : &mut CPU) -> u8 {
    if !cpu.regs.get_flag_c() {
        cpu.regs.sp -= 2;
        cpu.mem.write_short(cpu.regs.sp, cpu.regs.pc + 2);
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);
        
        return 24 /* Cycles */;
    } else {
        cpu.regs.pc += 2;
        
        return 12 /* Cycles */;
    }
}

/// **0xDC** - *CALL C,nn* - If N is true jump to address nn and store current pc in stack
pub fn call_c_nn(cpu : &mut CPU) -> u8 {
    if cpu.regs.get_flag_c() {
        cpu.regs.sp -= 2;
        cpu.mem.write_short(cpu.regs.sp, cpu.regs.pc + 2);
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.pc);
        
        return 24 /* Cycles */;
    } else {
        cpu.regs.pc += 2;
        
        return 12 /* Cycles */;
    }
}

/// -- Returns. --

/// **0xC0** - *RET nz* - Return if Z flag is reset
pub fn ret_nz(cpu : &mut CPU) -> u8 {
    if !cpu.regs.get_flag_z() {
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.sp);
        cpu.regs.sp += 2;
        
        return 20 /* Cycles */;
    } else {
        return 8 /* Cycles */;
    }
}

/// **0xC8** - *RET z* - Return if Z flag is set
pub fn ret_z(cpu : &mut CPU) -> u8 {
    if cpu.regs.get_flag_z() {
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.sp);
        cpu.regs.sp = cpu.regs.sp.wrapping_add(2);
        
        return 20 /* Cycles */;
    } else {
        return 8 /* Cycles */;
    }
}

/// **0xC9** - *RET* - Pop from stack, and jump to this address
pub fn ret(cpu : &mut CPU) -> u8 {
    cpu.regs.pc = cpu.mem.read_short(cpu.regs.sp);
    cpu.regs.sp = cpu.regs.sp.wrapping_add(2);
    
    return 16 /* Cycles */;
}

/// **0xD0** - *RET nc* - Return if C flag is reset
pub fn ret_nc(cpu : &mut CPU) -> u8 {
    if !cpu.regs.get_flag_c() {
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.sp);
        cpu.regs.sp = cpu.regs.sp.wrapping_add(2);
        
        return 20 /* Cycles */;
    } else {
        return 8 /* Cycles */;
    }
}

/// **0xD8** - *RET c* - Return if C flag is set
pub fn ret_c(cpu : &mut CPU) -> u8 {
    if cpu.regs.get_flag_c() {
        cpu.regs.pc = cpu.mem.read_short(cpu.regs.sp);
        cpu.regs.sp = cpu.regs.sp.wrapping_add(2);
        
        return 20 /* Cycles */;
    } else {
        return 8 /* Cycles */;
    }
}

/// **0xD9** - *RETI* - Return and enable interrupts
pub fn reti(cpu : &mut CPU) -> u8 {
    cpu.regs.pc = cpu.mem.read_short(cpu.regs.sp);
    cpu.regs.sp = cpu.regs.sp.wrapping_add(2);

    cpu.interrupts_countdown = 1;
    
    return 16 /* Cycles */;
}


/// **0xE9** - *JMP hl* - Jump to hl.
pub fn jmp_hl(cpu : &mut CPU) -> u8 {
    cpu.regs.pc = cpu.regs.get_hl();
    
    return 4 /* Cycles */;
}

/// -- Restarts. --
pub fn rst(cpu : &mut CPU, step : u16) -> u8 {
    cpu.regs.sp = cpu.regs.sp.wrapping_sub(2);
    cpu.mem.write_short(cpu.regs.sp, cpu.regs.pc);
    cpu.regs.pc = 0 + step;

    return 32 /* Cycles */;
}
