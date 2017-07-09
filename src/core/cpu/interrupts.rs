/**
 * interrupts.rs
 *
 * Contains types for interrupts.
**/

#[derive(Debug)]
#[allow(dead_code)] // For debug messages
pub enum InterruptType {
    VBLANK = 0,
    LCDC = 1,
    TIMER = 2,
    SERIAL = 3,
    KEYPAD = 4
}
