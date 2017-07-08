/**
 * bit.rs
 *
 * Operations to work on bits.
**/

use core::cpu::CPU;

/// **0xCB 0xC0 ~ 0xCB 0xFF** - *SET X,(hl)* - Set bit X in (hl)
pub fn set_x_hl(cpu : &mut CPU, digit : u8) -> u8 {
    let read_data = cpu.mem.read(cpu.regs.get_hl()) | (1 << digit);
    cpu.mem.write(cpu.regs.get_hl(), read_data);

    return 16 /* Cycles */;
}
