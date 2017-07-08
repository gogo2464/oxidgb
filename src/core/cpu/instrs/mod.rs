/**
 * instrs.rs
 *
 * The primary switchboard for CPU instructions.
**/

mod special;
mod jumps;
mod loads;
mod loads16;

use core::cpu::CPU;

use core::cpu::instrs::special::*;
use core::cpu::instrs::jumps::*;
use core::cpu::instrs::loads::*;
use core::cpu::instrs::loads16::*;

#[inline]
pub fn execute_instruction(cpu : &mut CPU, instr : u16, origin : u16) -> u8 {
    return match instr {
        0x00 => nop(cpu),
        0x01 => ld_bc_nnnn(cpu),
        0x02 => ld_pxx_x(cpu.regs.get_bc(), cpu.regs.a, cpu),
        0x06 => ld_b_n(cpu),
        0x08 => ld_pnn_sp(cpu),
        0x0A => ld_n_pxx(&cpu.mem, cpu.regs.get_bc(), &mut cpu.regs.a),
        0x0E => ld_c_n(cpu),
        0x11 => ld_de_nn(cpu),
        0x12 => ld_pxx_x(cpu.regs.get_de(), cpu.regs.a, cpu),
        0x16 => ld_d_n(cpu),
        0x18 => jr_n(cpu),
        0x1A => ld_n_pxx(&cpu.mem, cpu.regs.get_de(), &mut cpu.regs.a),
        0x1E => ld_e_n(cpu),
        0x20 => jr_nz_n(cpu),
        0x21 => ld_hl_nnnn(cpu),
        0x22 => ldi_phl_a(cpu),
        0x26 => ld_h_n(cpu),
        0x28 => jr_z_n(cpu),
        0x2A => ldi_a_phl(cpu),
        0x31 => ld_sp_nn(cpu),
        0x32 => ldd_phl_a(cpu),
        0x2E => ld_l_n(cpu),
        0x30 => jr_nc_n(cpu),
        0x36 => ld_phl_n(cpu),
        0x38 => jr_c_n(cpu),
        0x3A => ldd_a_phl(cpu),
        0x3E => ld_a_n(cpu),
        0x40 => ld_x_y(cpu.regs.b, &mut cpu.regs.b),
        0x41 => ld_x_y(cpu.regs.c, &mut cpu.regs.b),
        0x42 => ld_x_y(cpu.regs.d, &mut cpu.regs.b),
        0x43 => ld_x_y(cpu.regs.e, &mut cpu.regs.b),
        0x44 => ld_x_y(cpu.regs.h, &mut cpu.regs.b),
        0x45 => ld_x_y(cpu.regs.l, &mut cpu.regs.b),
        0x46 => ld_x_phl(cpu.regs.get_hl(), &cpu.mem, &mut cpu.regs.b),
        0x47 => ld_x_y(cpu.regs.a, &mut cpu.regs.b),
        0x48 => ld_x_y(cpu.regs.b, &mut cpu.regs.c),
        0x49 => ld_x_y(cpu.regs.c, &mut cpu.regs.c),
        0x4A => ld_x_y(cpu.regs.d, &mut cpu.regs.c),
        0x4B => ld_x_y(cpu.regs.e, &mut cpu.regs.c),
        0x4C => ld_x_y(cpu.regs.h, &mut cpu.regs.c),
        0x4D => ld_x_y(cpu.regs.l, &mut cpu.regs.c),
        0x4E => ld_x_phl(cpu.regs.get_hl(), &cpu.mem, &mut cpu.regs.c),
        0x4F => ld_x_y(cpu.regs.a, &mut cpu.regs.c),
        0x50 => ld_x_y(cpu.regs.b, &mut cpu.regs.d),
        0x51 => ld_x_y(cpu.regs.c, &mut cpu.regs.d),
        0x52 => ld_x_y(cpu.regs.d, &mut cpu.regs.d),
        0x53 => ld_x_y(cpu.regs.e, &mut cpu.regs.d),
        0x54 => ld_x_y(cpu.regs.h, &mut cpu.regs.d),
        0x55 => ld_x_y(cpu.regs.l, &mut cpu.regs.d),
        0x56 => ld_x_phl(cpu.regs.get_hl(), &cpu.mem, &mut cpu.regs.d),
        0x57 => ld_x_y(cpu.regs.a, &mut cpu.regs.d),
        0x58 => ld_x_y(cpu.regs.b, &mut cpu.regs.e),
        0x59 => ld_x_y(cpu.regs.c, &mut cpu.regs.e),
        0x5A => ld_x_y(cpu.regs.d, &mut cpu.regs.e),
        0x5B => ld_x_y(cpu.regs.e, &mut cpu.regs.e),
        0x5C => ld_x_y(cpu.regs.h, &mut cpu.regs.e),
        0x5D => ld_x_y(cpu.regs.l, &mut cpu.regs.e),
        0x5E => ld_x_phl(cpu.regs.get_hl(), &cpu.mem, &mut cpu.regs.e),
        0x5F => ld_x_y(cpu.regs.a, &mut cpu.regs.e),
        0x60 => ld_x_y(cpu.regs.b, &mut cpu.regs.h),
        0x61 => ld_x_y(cpu.regs.c, &mut cpu.regs.h),
        0x62 => ld_x_y(cpu.regs.d, &mut cpu.regs.h),
        0x63 => ld_x_y(cpu.regs.e, &mut cpu.regs.h),
        0x64 => ld_x_y(cpu.regs.h, &mut cpu.regs.h),
        0x65 => ld_x_y(cpu.regs.l, &mut cpu.regs.h),
        0x66 => ld_x_phl(cpu.regs.get_hl(), &cpu.mem, &mut cpu.regs.h),
        0x67 => ld_x_y(cpu.regs.a, &mut cpu.regs.h),
        0x68 => ld_x_y(cpu.regs.b, &mut cpu.regs.l),
        0x69 => ld_x_y(cpu.regs.c, &mut cpu.regs.l),
        0x6A => ld_x_y(cpu.regs.d, &mut cpu.regs.l),
        0x6B => ld_x_y(cpu.regs.e, &mut cpu.regs.l),
        0x6C => ld_x_y(cpu.regs.h, &mut cpu.regs.l),
        0x6D => ld_x_y(cpu.regs.l, &mut cpu.regs.l),
        0x6E => ld_x_phl(cpu.regs.get_hl(), &cpu.mem, &mut cpu.regs.l),
        0x6F => ld_x_y(cpu.regs.a, &mut cpu.regs.l),
        0x70 => ld_phl_x(cpu.regs.b, cpu),
        0x71 => ld_phl_x(cpu.regs.c, cpu),
        0x72 => ld_phl_x(cpu.regs.d, cpu),
        0x73 => ld_phl_x(cpu.regs.e, cpu),
        0x74 => ld_phl_x(cpu.regs.h, cpu),
        0x75 => ld_phl_x(cpu.regs.l, cpu),
        0x77 => ld_phl_x(cpu.regs.a, cpu),
        0x78 => ld_x_y(cpu.regs.b, &mut cpu.regs.a),
        0x79 => ld_x_y(cpu.regs.c, &mut cpu.regs.a),
        0x7A => ld_x_y(cpu.regs.d, &mut cpu.regs.a),
        0x7B => ld_x_y(cpu.regs.e, &mut cpu.regs.a),
        0x7C => ld_x_y(cpu.regs.h, &mut cpu.regs.a),
        0x7D => ld_x_y(cpu.regs.l, &mut cpu.regs.a),
        0x7E => ld_x_phl(cpu.regs.get_hl(), &cpu.mem, &mut cpu.regs.a),
        0x7F => ld_x_y(cpu.regs.a, &mut cpu.regs.a),
        0xC2 => jp_nz_nn(cpu),
        0xC3 => jmp_nn(cpu),
        0xD2 => jp_nc_nn(cpu),
        0xDA => jp_c_nn(cpu),
        0xC1 => pop_bc(cpu),
        0xC4 => call_nz_nn(cpu),
        0xC5 => push_bc(cpu),
        0xCA => jp_z_nn(cpu),
        0xCC => call_z_nn(cpu),
        0xCD => call_nn(cpu),
        0xD1 => pop_de(cpu),
        0xD4 => call_nc_nn(cpu),
        0xD5 => push_de(cpu),
        0xDC => call_c_nn(cpu),
        0xC0 => ret_nz(cpu),
        0xC7 => rst(cpu, 0x00),
        0xC8 => ret_z(cpu),
        0xC9 => ret(cpu),
        0xCF => rst(cpu, 0x08),
        0xD0 => ret_nc(cpu),
        0xD7 => rst(cpu, 0x10),
        0xD8 => ret_c(cpu),
        0xD9 => reti(cpu),
        0xDF => rst(cpu, 0x18),
        0xE0 => ldh_pn_a(cpu),
        0xE1 => pop_hl(cpu),
        0xE3 => ld_pc(cpu),
        0xE5 => push_hl(cpu),
        0xE7 => rst(cpu, 0x20),
        0xE9 => jmp_hl(cpu),
        0xEA => ld_pnn_a(cpu),
        0xEF => rst(cpu, 0x28),
        0xF0 => ldh_a_pn(cpu),
        0xF1 => pop_af(cpu),
        0xF2 => ld_a_ptrc(cpu),
        0xF5 => push_af(cpu),
        0xF7 => rst(cpu, 0x30),
        0xF8 => ldhl_sp_n(cpu),
        0xF9 => ld_sp_hl(cpu),
        0xFA => ld_a_pnn(cpu),
        0xFF => rst(cpu, 0x38),


        _ => {
            panic!("Unknown instruction: ${:02X} at ${:04X}", instr, origin);
        }
    }
}
