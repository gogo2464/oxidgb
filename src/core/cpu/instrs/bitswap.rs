/**
 * bitswap.rs
 *
 * Operations to swap nibbles in bits.
**/

use core::cpu::CPU;

/// **0xCB 0x30 ~ 0x37** - *SWAP X* - Swaps upper and lower nibbles of X.
macro_rules! swap {
    ($name:ident, $reg:ident) => (
        pub fn $name(cpu : &mut CPU) -> u8 {
            let current_value = cpu.regs.$reg;
            // TODO: Check output
            cpu.regs.f = 0;

            let new_value = ((current_value & 0xF0) >> 4) | ((current_value & 0x0F) << 4);
            cpu.regs.set_flag_z(new_value == 0);
            cpu.regs.$reg = new_value;

            return 8 /* Cycles */;
        }
    )
}

swap!(swap_b, b);
swap!(swap_c, c);
swap!(swap_d, d);
swap!(swap_e, e);
swap!(swap_h, h);
swap!(swap_l, l);
swap!(swap_a, a);