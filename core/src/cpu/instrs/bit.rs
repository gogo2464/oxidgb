/**
 * bit.rs
 *
 * Operations to work on bits.
**/
use cpu::cpu::CPU;

/// **0xCB 0x40~0x7D** - *BIT (HL),b* - Test bit X in (hl)
pub fn bit_phl(cpu: &mut CPU, digit: u8) -> u8 {
    let cur_value = cpu.mem.read(cpu.regs.get_hl());
    cpu.regs.set_flag_z((cur_value >> digit) & 0x1 == 0);
    cpu.regs.set_flag_n(false);
    cpu.regs.set_flag_h(true);
    16 /* Cycles */
}

/// *0xCB 0x40~0x7D** - *BIT X,b* - Test bit X in Y
macro_rules! bit {
    ($name:ident, $reg:ident) => {
        pub fn $name(cpu: &mut CPU, digit: u8) -> u8 {
            let cur_value = cpu.regs.$reg;
            cpu.regs.set_flag_z((cur_value >> digit) & 0x1 == 0);
            cpu.regs.set_flag_n(false);
            cpu.regs.set_flag_h(true);
            8 /* Cycles */
        }
    };
}

bit!(bit_b, b);
bit!(bit_c, c);
bit!(bit_d, d);
bit!(bit_e, e);
bit!(bit_h, h);
bit!(bit_l, l);
bit!(bit_a, a);

/// **0xCB 0xC0 ~ 0xCB 0xFF** - *SET X,Y* - Set bit X in register Y
macro_rules! set {
    ($name:ident, $reg:ident) => {
        pub fn $name(cpu: &mut CPU, digit: u8) -> u8 {
            cpu.regs.$reg |= 1 << digit;
            8 /* Cycles */
        }
    };
}

set!(set_b, b);
set!(set_c, c);
set!(set_d, d);
set!(set_e, e);
set!(set_h, h);
set!(set_l, l);
set!(set_a, a);

/// **0xCB 0xC0 ~ 0xCB 0xFF** - *SET X,(hl)* - Set bit X in (hl)
pub fn set_x_hl(cpu: &mut CPU, digit: u8) -> u8 {
    let read_data = cpu.mem.read(cpu.regs.get_hl()) | (1 << digit);
    cpu.mem.write(cpu.regs.get_hl(), read_data);

    16 /* Cycles */
}

/// **0xCB 0x80 ~ 0xCB 0xBF** - *RES X,Y* - Reset bit X in register Y
macro_rules! res {
    ($name:ident, $reg:ident) => {
        pub fn $name(cpu: &mut CPU, digit: u8) -> u8 {
            cpu.regs.$reg &= !(1 << digit);
            8 /* Cycles */
        }
    };
}

res!(res_b, b);
res!(res_c, c);
res!(res_d, d);
res!(res_e, e);
res!(res_h, h);
res!(res_l, l);
res!(res_a, a);

/// **0xCB 0x80 ~ 0xCB 0xBF** - *RES X,(hl)* - Reset bit X in (hl)
pub fn res_x_hl(cpu: &mut CPU, digit: u8) -> u8 {
    let read_data = cpu.mem.read(cpu.regs.get_hl()) & !(1 << digit);
    cpu.mem.write(cpu.regs.get_hl(), read_data);

    16 /* Cycles */
}
