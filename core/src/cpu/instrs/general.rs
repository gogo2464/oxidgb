/**
 * general.rs
 *
 * General arithmetic.
**/
use cpu::CPU;

use cpu::instrs::utils::*;

/// **0x80 ~ 0xE8** - *ADD X* - Add X to a.
macro_rules! add {
    ($name:ident, $reg:ident) => {
        pub fn $name(cpu: &mut CPU) -> u8 {
            let x = cpu.regs.a;
            let y = cpu.regs.$reg;
            let new_value = x.wrapping_add(y);
            cpu.regs.a = new_value;

            cpu.regs.set_flag_z(new_value == 0);
            cpu.regs.set_flag_n(false);
            cpu.regs.set_flag_h(((y & 0x0F) + (x & 0x0F)) > 0xF);
            cpu.regs.set_flag_c((y as u16 + x as u16) > 0xFF);

            4 /* Cycles */
        }
    };
}

add!(add_b, b);
add!(add_c, c);
add!(add_d, d);
add!(add_e, e);
add!(add_h, h);
add!(add_l, l);
add!(add_a, a);

/// **0x80 ~ 0xE8** - *ADC X* - Add X to a with carry.
macro_rules! adc {
    ($name:ident, $reg:ident) => {
        pub fn $name(cpu: &mut CPU) -> u8 {
            let x = cpu.regs.a;
            let y = cpu.regs.$reg;
            let carry = if cpu.regs.get_flag_c() { 1 } else { 0 };
            let new_value = x.wrapping_add(y).wrapping_add(carry);
            cpu.regs.a = new_value;

            cpu.regs.set_flag_z(new_value == 0);
            cpu.regs.set_flag_n(false);
            cpu.regs.set_flag_h(((y & 0x0F) + (x & 0x0F) + carry) > 0xF);
            cpu.regs
                .set_flag_c((y as u16 + x as u16 + carry as u16) > 0xFF);

            4 /* Cycles */
        }
    };
}

adc!(adc_b, b);
adc!(adc_c, c);
adc!(adc_d, d);
adc!(adc_e, e);
adc!(adc_h, h);
adc!(adc_l, l);
adc!(adc_a, a);

/**
 * **0xC6** - *ADD a,#* - Add # to a.
 */
pub fn add_a_n(cpu: &mut CPU) -> u8 {
    let prev_value = cpu.regs.a;
    let value = get_n(cpu);
    let new_value = prev_value.wrapping_add(value);

    cpu.regs.a = new_value;

    cpu.regs.set_flag_z(new_value == 0);
    cpu.regs.set_flag_n(false);
    cpu.regs
        .set_flag_h(((value & 0x0F) + (prev_value & 0x0F)) > 0xF);
    cpu.regs
        .set_flag_c(((prev_value as u16) + (value as u16)) > 0xFF);

    8 /* Cycles */
}

/**
 * **0x86** - *ADD (hl)* - Add *hl to a.
 */
pub fn add_a_phl(cpu: &mut CPU) -> u8 {
    let prev_value = cpu.regs.a;
    let value = cpu.mem.read(cpu.regs.get_hl());
    let new_value = prev_value.wrapping_add(value);

    cpu.regs.a = new_value;

    cpu.regs.set_flag_z(new_value == 0);
    cpu.regs.set_flag_n(false);
    cpu.regs
        .set_flag_h(((value & 0x0F) + (prev_value & 0x0F)) > 0xF);
    cpu.regs
        .set_flag_c(((prev_value as u16) + (value as u16)) > 0xFF);

    8 /* Cycles */
}

/**
 * **0xD6** - *SUB a,#* - Subtract # to a.
 */
pub fn sub_a_n(cpu: &mut CPU) -> u8 {
    let prev_value = cpu.regs.a;
    let value = get_n(cpu);
    let new_value = prev_value.wrapping_sub(value);

    cpu.regs.a = new_value;

    cpu.regs.set_flag_z(new_value == 0);
    cpu.regs.set_flag_n(true);
    cpu.regs
        .set_flag_h(((prev_value as i16 & 0x0F) - (value as i16 & 0x0F)) < 0);
    cpu.regs
        .set_flag_c(((prev_value as i16) - (value as i16)) < 0);

    8 /* Cycles */
}

/// **0x19 ~ 0x39** - *ADD HL,X* - Add XX to HL.
pub fn add_hl_x(val: u16, cpu: &mut CPU) -> u8 {
    // TODO: Check accuracy of flags
    let prev_value = cpu.regs.get_hl();
    cpu.regs.set_hl(prev_value.wrapping_add(val));

    cpu.regs.set_flag_n(false);

    cpu.regs
        .set_flag_h(((val & 0x0FFF) + (prev_value & 0x0FFF)) > 0xFFF);
    cpu.regs
        .set_flag_c(((prev_value as u32) + (val as u32)) > 0xFFFF);

    8 /* Cycles */
}

/// **0xCE** - *ADC a,#* - Add # + Carry flag to a.
pub fn adc_a_n(cpu: &mut CPU) -> u8 {
    let value = get_n(cpu);
    let old_value = cpu.regs.a;
    let new_value = old_value
        .wrapping_add(value)
        .wrapping_add(if cpu.regs.get_flag_c() { 1 } else { 0 });

    let unwrapped_value =
        (old_value as u16) + (value as u16) + if cpu.regs.get_flag_c() { 1 } else { 0 };

    cpu.regs.a = new_value;

    cpu.regs.f = 0;
    cpu.regs.set_flag_z(new_value == 0);

    cpu.regs
        .set_flag_h((((old_value as u16) ^ unwrapped_value ^ (value as u16)) & 0x10) == 0x10);
    cpu.regs
        .set_flag_c((((old_value as u16) ^ unwrapped_value ^ (value as u16)) & 0x100) == 0x100);

    8 /* Cycles */
}

/// **0xCE** - *ADC a,#* - Add # + Carry flag to a.
pub fn adc_a_phl(cpu: &mut CPU) -> u8 {
    let value = cpu.mem.read(cpu.regs.get_hl());
    let old_value = cpu.regs.a;
    let new_value = old_value
        .wrapping_add(value)
        .wrapping_add(if cpu.regs.get_flag_c() { 1 } else { 0 });

    let unwrapped_value =
        (old_value as u16) + (value as u16) + if cpu.regs.get_flag_c() { 1 } else { 0 };

    cpu.regs.a = new_value;

    cpu.regs.f = 0;
    cpu.regs.set_flag_z(new_value == 0);

    cpu.regs
        .set_flag_h((((old_value as u16) ^ unwrapped_value ^ (value as u16)) & 0x10) == 0x10);
    cpu.regs
        .set_flag_c((((old_value as u16) ^ unwrapped_value ^ (value as u16)) & 0x100) == 0x100);

    8 /* Cycles */
}

/// **0xE8** - *ADD sp,#S* - Add signed value # to sp.
pub fn add_sp_ns(cpu: &mut CPU) -> u8 {
    let prev_value = cpu.regs.sp;
    let value = get_n(cpu) as i8;
    let result = (prev_value as i16).wrapping_add(value as i16) as u16;

    let unwrapped_value = (prev_value as i16).wrapping_add(value as i16);

    cpu.regs.sp = result;

    cpu.regs.set_flag_z(false);
    cpu.regs.set_flag_n(false);

    // Credit to Gearboy for this one:
    // https://github.com/drhelius/Gearboy
    cpu.regs
        .set_flag_h((((prev_value as i16) ^ unwrapped_value ^ (value as i16)) & 0x10) == 0x10);
    cpu.regs
        .set_flag_c((((prev_value as i16) ^ unwrapped_value ^ (value as i16)) & 0x100) == 0x100);

    16 /* Cycles */
}

// Subtraction

/// **0x90 ~ 0x95** - *SUB A,X* - Subtract X from A.
macro_rules! sub {
    ($name:ident, $reg:ident) => {
        pub fn $name(cpu: &mut CPU) -> u8 {
            let x = cpu.regs.$reg;
            let prev_value = cpu.regs.a;
            let new_value = prev_value.wrapping_sub(x);

            cpu.regs.a = new_value;

            cpu.regs.set_flag_z(new_value == 0);
            cpu.regs.set_flag_n(true);
            cpu.regs.set_flag_h((prev_value & 0xF) < (new_value & 0xF));
            cpu.regs.set_flag_c((prev_value as i16 - x as i16) < 0);

            4 /* Cycles */
        }
    };
}

sub!(sub_b, b);
sub!(sub_c, c);
sub!(sub_d, d);
sub!(sub_e, e);
sub!(sub_h, h);
sub!(sub_l, l);
sub!(sub_a, a);

/// **0x96** - *SUB (hl)* - Subtract *hl from a.
pub fn sub_phl(cpu: &mut CPU) -> u8 {
    let x = cpu.mem.read(cpu.regs.get_hl());
    let prev_value = cpu.regs.a;
    let new_value = prev_value.wrapping_sub(x);

    cpu.regs.a = new_value;

    cpu.regs.set_flag_z(new_value == 0);
    cpu.regs.set_flag_n(true);
    cpu.regs.set_flag_h((prev_value & 0xF) < (new_value & 0xF));
    cpu.regs.set_flag_c((prev_value as i16 - x as i16) < 0);

    8 /* Cycles */
}

/// **0x80 ~ 0xE8** - *SBC X* - Subtract X from a with carry.
macro_rules! sbc {
    ($name:ident, $reg:ident) => {
        pub fn $name(cpu: &mut CPU) -> u8 {
            let x = cpu.regs.a;
            let y = cpu.regs.$reg;
            let carry = if cpu.regs.get_flag_c() { 1 } else { 0 };
            let new_value = x.wrapping_sub(y).wrapping_sub(carry);
            cpu.regs.a = new_value;

            cpu.regs.set_flag_z(new_value == 0);
            cpu.regs.set_flag_n(true);
            cpu.regs
                .set_flag_h(((x as i16 & 0x0F) - (y as i16 & 0x0F) - carry as i16) < 0);
            cpu.regs
                .set_flag_c((x as i16 - y as i16 - carry as i16) < 0);

            4 /* Cycles */
        }
    };
}

sbc!(sbc_b, b);
sbc!(sbc_c, c);
sbc!(sbc_d, d);
sbc!(sbc_e, e);
sbc!(sbc_h, h);
sbc!(sbc_l, l);
sbc!(sbc_a, a);

/// **0x9E** - *SBC a,(hl)* - Subtract (hl) + Carry flag from A.
pub fn sbc_phl(cpu: &mut CPU) -> u8 {
    let prev_value = cpu.regs.a;
    let value = cpu.mem.read(cpu.regs.get_hl());
    let flag = if cpu.regs.get_flag_c() { 1 } else { 0 };
    let new_value = prev_value.wrapping_sub(value).wrapping_sub(flag);

    cpu.regs.a = new_value;

    cpu.regs.set_flag_z(new_value == 0);
    cpu.regs.set_flag_n(true);
    cpu.regs
        .set_flag_h((prev_value as i16 & 0x0f) - (value as i16 & 0x0f) - (flag as i16) < 0);
    cpu.regs
        .set_flag_c((prev_value as i16) - (value as i16) - (flag as i16) < 0);

    8 /* Cycles */
}

/// **0xDE** - *SBC n* -  Subtract n + Carry from a.
pub fn sbc_n(cpu: &mut CPU) -> u8 {
    let prev_value = cpu.regs.a;
    let value = get_n(cpu);
    let flag = if cpu.regs.get_flag_c() { 1 } else { 0 };
    let new_value = prev_value.wrapping_sub(value).wrapping_sub(flag);

    cpu.regs.a = new_value;

    cpu.regs.set_flag_z(new_value == 0);
    cpu.regs.set_flag_n(true);
    cpu.regs
        .set_flag_h((prev_value as i16 & 0x0f) - (value as i16 & 0x0f) - (flag as i16) < 0);
    cpu.regs
        .set_flag_c((prev_value as i16) - (value as i16) - (flag as i16) < 0);

    8 /* Cycles */
}
