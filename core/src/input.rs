/**
 * input.rs
 *
 * Handles input devices for the Gameboy.
**/

#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serialisation", derive(Serialize, Deserialize))]
pub enum GameboyButton {
    A,
    B,
    LEFT,
    RIGHT,
    UP,
    DOWN,
    START,
    SELECT,
}

#[cfg_attr(feature = "serialisation", derive(Serialize, Deserialize))]
pub struct GameboyInput {
    pub p14: u8,
    pub p15: u8,
}

pub fn build_input(input: &[GameboyButton]) -> GameboyInput {
    let mut p14 = 0;
    let mut p15 = 0;

    for key in input {
        match key {
            GameboyButton::DOWN => p14 |= 1 << 3,
            GameboyButton::UP => p14 |= 1 << 2,
            GameboyButton::LEFT => p14 |= 1 << 1,
            GameboyButton::RIGHT => p14 |= 1,
            GameboyButton::START => p15 |= 1 << 3,
            GameboyButton::SELECT => p15 |= 1 << 2,
            GameboyButton::B => p15 |= 1 << 1,
            GameboyButton::A => p15 |= 1,
        }
    }

    GameboyInput { p14, p15 }
}
