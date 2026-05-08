use crate::chip8::{display::Display, keypad::Keypad, memory::Memory};

pub struct Bus {
    pub memory: Memory,
    pub display: Display,
    pub keypad: Keypad,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: Memory::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }
}
