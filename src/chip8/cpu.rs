use crate::chip8::{bus::Bus, registers::Registers, stack::Stack};

pub struct Cpu {
    registers: Registers,
    stack: Stack,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            stack: Stack::new(),
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        // fetch
        let opcode = bus.memory.read_u16(self.registers.pc);
        self.registers.pc += 2;

        // decode + execute
        self.execute(bus, opcode);
    }

    fn execute(&self, bus: &mut Bus, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as u8;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => todo!(), // 00E0
            (0x0, 0x0, 0xE, 0xE) => todo!(), // 00EE
            (0x1, _, _, _) => todo!(),       // 1nnn
            (0x2, _, _, _) => todo!(),       // 2nnn
            (0x3, _, _, _) => todo!(),       // 3xkk
            (0x4, _, _, _) => todo!(),       // 4xkk
            (0x5, _, _, 0x0) => todo!(),     // 5xy0
            (0x6, _, _, _) => todo!(),       // 6xkk
            (0x7, _, _, _) => todo!(),       // 7xkk
            (0x8, _, _, 0x0) => todo!(),     // 8xy0
            (0x8, _, _, 0x1) => todo!(),     // 8xy1
            (0x8, _, _, 0x2) => todo!(),     // 8xy2
            (0x8, _, _, 0x3) => todo!(),     // 8xy3
            (0x8, _, _, 0x4) => todo!(),     // 8xy4
            (0x8, _, _, 0x5) => todo!(),     // 8xy5
            (0x8, _, _, 0x6) => todo!(),     // 8xy6
            (0x8, _, _, 0x7) => todo!(),     // 8xy7
            (0x8, _, _, 0xE) => todo!(),     // 8xyE
            (0x9, _, _, 0x0) => todo!(),     // 9xy0
            (0xA, _, _, _) => todo!(),       // Annn
            (0xB, _, _, _) => todo!(),       // Bnnn
            (0xC, _, _, _) => todo!(),       // Cxkk
            (0xD, _, _, _) => todo!(),       // Dxyn
            (0xE, _, 0x9, 0xE) => todo!(),   // Ex9E
            (0xE, _, 0xA, 0x1) => todo!(),   // ExA1
            (0xF, _, 0x0, 0x7) => todo!(),   // Fx07
            (0xF, _, 0x0, 0xA) => todo!(),   // Fx0A
            (0xF, _, 0x1, 0x5) => todo!(),   // Fx15
            (0xF, _, 0x1, 0x8) => todo!(),   // Fx18
            (0xF, _, 0x1, 0xE) => todo!(),   // Fx1E
            (0xF, _, 0x2, 0x9) => todo!(),   // Fx29
            (0xF, _, 0x3, 0x3) => todo!(),   // Fx33
            (0xF, _, 0x5, 0x5) => todo!(),   // Fx55
            (0xF, _, 0x6, 0x5) => todo!(),   // Fx65
            _ => (),
        }
    }
}
