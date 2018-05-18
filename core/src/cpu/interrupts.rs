/**
 * interrupts.rs
 *
 * Contains types for interrupts.
**/

#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(dead_code)] // For debug messages
pub enum InterruptType {
    VBLANK = 0,
    LCDC = 1,
    TIMER = 2,
    SERIAL = 3,
    KEYPAD = 4
}

impl InterruptType {
    pub fn get_by_bit(bit : u8) -> Option<InterruptType> {
        return match bit {
            0 => Some(InterruptType::VBLANK),
            1 => Some(InterruptType::LCDC),
            2 => Some(InterruptType::TIMER),
            3 => Some(InterruptType::SERIAL),
            4 => Some(InterruptType::KEYPAD),
            _ => {
                warn!("Unknown interrupt type: {}", bit);
                None
            }
        }
    }
}