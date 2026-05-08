const MEMORY_SIZE: usize = 4096;
const FONT_START: usize = 0x000;
const ROM_START: usize = 0x200;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Memory {
    pub data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let mut data = [0; MEMORY_SIZE];

        // load fontset
        data[FONT_START..FONT_START + FONT_SET.len()].copy_from_slice(&FONT_SET);

        Memory { data: data }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.data[ROM_START..ROM_START + data.len()].copy_from_slice(data);
    }

    // u8
    pub fn read_u8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    // u16
    pub fn read_u16(&self, addr: u16) -> u16 {
        let hi = self.data[addr as usize] as u16;
        let lo = self.data[addr as usize + 1] as u16;
        (hi << 8) | lo
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        self.data[addr as usize] = (value >> 8) as u8;
        self.data[addr as usize + 1] = value as u8;
    }
}
