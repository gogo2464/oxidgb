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

set!(set_b, b);
set!(set_c, c);
set!(set_d, d);
set!(set_e, e);
set!(set_h, h);
set!(set_l, l);
set!(set_a, a);

/// **0xCB 0xC0 ~ 0xCB 0xFF** - *SET X,(hl)* - Set bit X in (hl)
pub fn set_x_hl(cpu : &mut CPU, digit : u8) -> u8 {
    let read_data = cpu.mem.read(cpu.regs.get_hl()) | (1 << digit);
    cpu.mem.write(cpu.regs.get_hl(), read_data);

    return 16 /* Cycles */;
}

/// **0xCB 0x80 ~ 0xCB 0xBF** - *RES X,Y* - Reset bit X in register Y
macro_rules! res {
    ($name:ident, $reg:ident) => (
        pub fn $name(cpu : &mut CPU, digit : u8) -> u8 {
            cpu.regs.$reg = cpu.regs.$reg & !(1 << digit);
            return 8 /* Cycles */;
        }
    )
}

res!(res_b, b);
res!(res_c, c);
res!(res_d, d);
res!(res_e, e);
res!(res_h, h);
res!(res_l, l);
res!(res_a, a);

/// **0xCB 0x80 ~ 0xCB 0xBF** - *RES X,(hl)* - Reset bit X in (hl)
pub fn res_x_hl(cpu : &mut CPU, digit : u8) -> u8 {
    let read_data = cpu.mem.read(cpu.regs.get_hl()) & !(1 << digit);
    cpu.mem.write(cpu.regs.get_hl(), read_data);

    return 16 /* Cycles */;
}
