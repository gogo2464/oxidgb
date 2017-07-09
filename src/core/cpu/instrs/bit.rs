/**
 * bit.rs
 *
 * Operations to work on bits.
**/

use core::cpu::CPU;

/// **0xCB 0xC0 ~ 0xCB 0xFF** - *SET X,Y* - Set bit X in register Y
macro_rules! set {
    ($name:ident, $reg:ident) => (
        pub fn $name(cpu : &mut CPU, digit : u8) -> u8 {
            cpu.regs.$reg = cpu.regs.$reg | (1 << digit);
            return 8 /* Cycles */;
        }
    )
}

set!(set_b, a);
set!(set_c, a);
set!(set_d, a);
set!(set_e, a);
set!(set_h, a);
set!(set_l, a);
set!(set_a, a);

/// **0xCB 0xC0 ~ 0xCB 0xFF** - *SET X,(hl)* - Set bit X in (hl)
pub fn set_x_hl(cpu : &mut CPU, digit : u8) -> u8 {
    let read_data = cpu.mem.read(cpu.regs.get_hl()) | (1 << digit);
    cpu.mem.write(cpu.regs.get_hl(), read_data);

    return 16 /* Cycles */;
}