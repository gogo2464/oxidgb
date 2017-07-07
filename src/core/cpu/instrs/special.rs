/**
 * special.rs
 *
 * Special CPU instructions.
**/

use core::cpu::CPU;

/// **0x00** - *NOP* - No operation.
pub fn nop(_ : &mut CPU) -> u8 {
    return 4 /* Cycles */
}
