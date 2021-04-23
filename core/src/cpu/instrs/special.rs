/**
 * special.rs
 *
 * Special CPU instructions.
**/
use cpu::cpu::CPU;

/// **0x00** - *NOP* - No operation.
pub fn nop(_: &mut CPU) -> u8 {
    4 /* Cycles */
}

/// **0x10** (0x00) - *STOP* - Halt CPU & LCD display until button pressed.
pub fn stop(cpu: &mut CPU) -> u8 {
    cpu.stopped = true;
    4 /* Cycles */
}

/// **0x27** - *DAA* - Convert to a Binary Coded Decimal
pub fn daa(cpu: &mut CPU) -> u8 {
    let mut value = cpu.regs.a;

    // Uses Blarrg's implementation
    if !cpu.regs.get_flag_n() {
        if cpu.regs.get_flag_c() || value > 0x99 {
            value = value.wrapping_add(0x60);
            cpu.regs.set_flag_c(true);
        }

        if cpu.regs.get_flag_h() || value & 0xF > 0x9 {
            value = value.wrapping_add(0x06);
            cpu.regs.set_flag_h(false);
        }
    } else {
        if cpu.regs.get_flag_c() {
            value = value.wrapping_add(0xA0);
        }

        if cpu.regs.get_flag_h() {
            value = value.wrapping_add(0xFA);
            cpu.regs.set_flag_h(false);
        }
    }

    cpu.regs.a = value;

    cpu.regs.set_flag_z(value == 0);

    4 /* Cycles */
}

/// **0x2F** - *CPL* - Complement register a
pub fn cpl(cpu: &mut CPU) -> u8 {
    cpu.regs.a = !cpu.regs.a;

    cpu.regs.set_flag_n(true);
    cpu.regs.set_flag_h(true);

    4 /* Cycles */
}

/// **0x37** - *SCF* - Set carry flag.
pub fn scf(cpu: &mut CPU) -> u8 {
    cpu.regs.set_flag_n(false);
    cpu.regs.set_flag_h(false);
    cpu.regs.set_flag_c(true);

    4 /* Cycles */
}

/// **0x3F** - *CCF* - Compliment carry.
pub fn ccf(cpu: &mut CPU) -> u8 {
    cpu.regs.set_flag_n(false);
    cpu.regs.set_flag_h(false);
    let new_flag = !cpu.regs.get_flag_c();
    cpu.regs.set_flag_c(new_flag);

    4 /* Cycles */
}

/// **0xF3** - *DI* - Disable interrupts
pub fn di(cpu: &mut CPU) -> u8 {
    // TODO: On next instruction?
    cpu.interrupts_countdown = -1;
    cpu.interrupts_enabled = false;

    4 /* Cycles */
}

/// **0xFB** - *EI* - Enable interrupts
pub fn ei(cpu: &mut CPU) -> u8 {
    cpu.interrupts_countdown = 1; // Countdown to enable

    4 /* Cycles */
}

/// **0x76** - *HALT* - Halt the CPU until a interrupt occurs.
pub fn halt(cpu: &mut CPU) -> u8 {
    cpu.halted = true;

    4 /* Cycles */
}

/// Unknown instruction handler.
pub fn bad_instruction(instr: u16) -> u8 {
    panic!(
        "Bad instruction: ${:04x}. This would freeze a Gameboy!",
        instr
    );

    // 4 cycles for opcode fetch
}
