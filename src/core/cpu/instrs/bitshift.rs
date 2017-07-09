/**
 * bitshift.rs
 *
 * Operations to shift bits.
**/

use core::cpu::CPU;

/// **0xCB 0x23** - *SLA X* - Shift X left into Carry. LSB of a set to 0.
macro_rules! sla {
    ($name:ident, $reg:ident) => (
        pub fn $name(cpu : &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            let new_value = (current_value << 1) & !1;// & (0b01111111);

            cpu.regs.$reg = new_value;

            cpu.regs.f = 0;
            cpu.regs.set_flag_c((current_value >> 7) & 0x1 == 1);
            cpu.regs.set_flag_z(new_value == 0);
            return 8 /* Cycles */;
        }
    )
}

sla!(sla_b, b);
sla!(sla_c, c);
sla!(sla_d, d);
sla!(sla_e, e);
sla!(sla_h, h);
sla!(sla_l, l);
sla!(sla_a, a);

/// **0xCB 0x38 ~ 0xCB 0x3F** - *SRL X* - Shift X right through Carry.
macro_rules! srl {
    ($name:ident, $reg:ident) => (
        pub fn $name(cpu : &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            let new_value = (current_value >> 1);// & (0b01111111);

            cpu.regs.$reg = new_value;

            // TODO: MSB set to 0?
            cpu.regs.f = 0;
            cpu.regs.set_flag_c(current_value & 0x1 == 1);
            cpu.regs.set_flag_z(new_value == 0);
            return 8 /* Cycles */;
        }
    )
}

srl!(srl_b, b);
srl!(srl_c, c);
srl!(srl_d, d);
srl!(srl_e, e);
srl!(srl_h, h);
srl!(srl_l, l);
srl!(srl_a, a);

/// **0xCB 0x28 ~ 0xCB 0x2F** - *SRA X* - Shift X right through Carry.
macro_rules! sra {
    ($name:ident, $reg:ident) => (
        pub fn $name(cpu : &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            let new_value = (current_value >> 1) | (current_value & 0b10000000);

            cpu.regs.$reg = new_value;

            cpu.regs.f = 0;
            cpu.regs.set_flag_c(current_value & 0x1 == 1);
            cpu.regs.set_flag_z(new_value == 0);
            return 8 /* Cycles */;
        }
    )
}

sra!(sra_b, b);
sra!(sra_c, c);
sra!(sra_d, d);
sra!(sra_e, e);
sra!(sra_h, h);
sra!(sra_l, l);
sra!(sra_a, a);

