use crate::chip8::{display::Display, memory::Memory};

pub struct Bus {
    pub memory: Memory,
    pub display: Display,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: Memory::new(),
            display: Display::new(),
        }
    }
}
