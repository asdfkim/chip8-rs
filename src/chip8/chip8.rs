use crate::chip8::{bus::Bus, cpu::Cpu};

pub struct Chip8 {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.bus.memory.load_rom(data);
    }
}
