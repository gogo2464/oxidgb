/**
 * instrs.rs
 *
 * The primary switchboard for CPU instructions.
**/

mod special;
mod jumps;

use core::cpu::CPU;

use core::cpu::instrs::special::*;
use core::cpu::instrs::jumps::*;

#[inline]
pub fn execute_instruction(cpu : &mut CPU, instr : u16, origin : u16) -> u8 {
    return match instr {
        0x00 => nop(cpu),
        0x18 => jr_n(cpu),
        0x20 => jr_nz_n(cpu),
        0x28 => jr_z_n(cpu),
        0x30 => jr_nc_n(cpu),
        0x38 => jr_c_n(cpu),
        0xC2 => jp_nz_nn(cpu),
        0xC3 => jmp_nn(cpu),
        0xD2 => jp_nc_nn(cpu),
        0xDA => jp_c_nn(cpu),
        0xC4 => call_nz_nn(cpu),
        0xCA => jp_z_nn(cpu),
        0xCC => call_z_nn(cpu),
        0xCD => call_nn(cpu),
        0xD4 => call_nc_nn(cpu),
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
        0xE7 => rst(cpu, 0x20),
        0xE9 => jmp_hl(cpu),
        0xEF => rst(cpu, 0x28),
        0xF7 => rst(cpu, 0x30),
        0xFF => rst(cpu, 0x38),


        _ => {
            panic!("Unknown instruction: ${:02X} at ${:04X}", instr, origin);
        }
    }
}
