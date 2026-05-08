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

    fn execute(&mut self, bus: &mut Bus, opcode: u16) {
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
            (0x0, 0x0, 0xE, 0x0) => self.op_00e0(bus),    // 00E0
            (0x0, 0x0, 0xE, 0xE) => self.op_00ee(),       // 00EE
            (0x1, _, _, _) => self.op_1nnn(nnn),          // 1nnn
            (0x2, _, _, _) => self.op_2nnn(nnn),          // 2nnn
            (0x3, _, _, _) => self.op_3xkk(x, kk),        // 3xkk
            (0x4, _, _, _) => self.op_4xkk(x, kk),        // 4xkk
            (0x5, _, _, 0x0) => self.op_5xy0(x, y),       // 5xy0
            (0x6, _, _, _) => self.op_6xkk(x, kk),        // 6xkk
            (0x7, _, _, _) => self.op_7xkk(x, kk),        // 7xkk
            (0x8, _, _, 0x0) => self.op_8xy0(x, y),       // 8xy0
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),       // 8xy1
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),       // 8xy2
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),       // 8xy3
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),       // 8xy4
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),       // 8xy5
            (0x8, _, _, 0x6) => self.op_8xy6(x),          // 8xy6
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),       // 8xy7
            (0x8, _, _, 0xE) => self.op_8xye(x),          // 8xyE
            (0x9, _, _, 0x0) => self.op_9xy0(x, y),       // 9xy0
            (0xA, _, _, _) => self.op_annn(nnn),          // Annn
            (0xB, _, _, _) => self.op_bnnn(nnn),          // Bnnn
            (0xC, _, _, _) => self.op_cxkk(x, kk),        // Cxkk
            (0xD, _, _, _) => self.op_dxyn(bus, x, y, n), // Dxyn
            (0xE, _, 0x9, 0xE) => self.op_ex9e(bus, x),   // Ex9E
            (0xE, _, 0xA, 0x1) => self.op_exa1(bus, x),   // ExA1
            (0xF, _, 0x0, 0x7) => self.op_fx07(x),        // Fx07
            (0xF, _, 0x0, 0xA) => self.op_fx0a(bus, x),   // Fx0A
            (0xF, _, 0x1, 0x5) => self.op_fx15(x),        // Fx15
            (0xF, _, 0x1, 0x8) => self.op_fx18(x),        // Fx18
            (0xF, _, 0x1, 0xE) => self.op_fx1e(x),        // Fx1E
            (0xF, _, 0x2, 0x9) => self.op_fx29(x),        // Fx29
            (0xF, _, 0x3, 0x3) => self.op_fx33(bus, x),   // Fx33
            (0xF, _, 0x5, 0x5) => self.op_fx55(bus, x),   // Fx55
            (0xF, _, 0x6, 0x5) => self.op_fx65(bus, x),   // Fx65
            _ => (),
        }
    }

    fn op_00e0(&mut self, bus: &mut Bus) {
        bus.display.clear();
        self.registers.df = true;
    }

    fn op_00ee(&mut self) {
        self.registers.pc = self.stack.pop(&mut self.registers.sp);
    }

    fn op_1nnn(&mut self, nnn: u16) {
        self.registers.pc = nnn;
    }

    fn op_2nnn(&mut self, nnn: u16) {
        self.stack.push(&mut self.registers.sp, self.registers.pc);
        self.registers.pc = nnn;
    }

    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.registers.v[x] == kk {
            self.registers.pc += 2;
        }
    }

    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.registers.v[x] != kk {
            self.registers.pc += 2;
        }
    }

    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.registers.v[x] == self.registers.v[y] {
            self.registers.pc += 2;
        }
    }

    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.registers.v[x] = kk;
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.registers.v[x] = self.registers.v[x].wrapping_add(kk);
    }

    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.registers.v[x] = self.registers.v[y];
    }

    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.registers.v[x] = self.registers.v[x] | self.registers.v[y];
    }

    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.registers.v[x] = self.registers.v[x] & self.registers.v[y];
    }

    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.registers.v[x] ^= self.registers.v[y];
    }

    fn op_8xy4(&mut self, x: usize, y: usize) {
        let (result, carry) = self.registers.v[x].overflowing_add(self.registers.v[y]);
        self.registers.v[x] = result;
        self.registers.v[0xF] = carry as u8;
    }

    fn op_8xy5(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.registers.v[x].overflowing_sub(self.registers.v[y]);
        self.registers.v[x] = result;
        self.registers.v[0xF] = !borrow as u8;
    }

    fn op_8xy6(&mut self, x: usize) {
        self.registers.v[0xF] = self.registers.v[x] & 0x1;
        self.registers.v[x] >>= 1;
    }

    fn op_8xy7(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.registers.v[y].overflowing_sub(self.registers.v[x]);
        self.registers.v[x] = result;
        self.registers.v[0xF] = !borrow as u8;
    }

    fn op_8xye(&mut self, x: usize) {
        self.registers.v[0xF] = (self.registers.v[x] >> 7) & 0x1;
        self.registers.v[x] <<= 1;
    }

    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.registers.v[x] != self.registers.v[y] {
            self.registers.pc += 2;
        }
    }

    fn op_annn(&mut self, nnn: u16) {
        self.registers.i = nnn;
    }

    fn op_bnnn(&mut self, nnn: u16) {
        self.registers.pc = nnn.wrapping_add(self.registers.v[0] as u16);
    }

    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let rnd: u8 = rand::random();
        self.registers.v[x] = rnd & kk;
    }

    fn op_dxyn(&mut self, bus: &mut Bus, x: usize, y: usize, n: u8) {
        let vx = self.registers.v[x];
        let vy = self.registers.v[y];
        let start = self.registers.i as usize;
        let sprite = bus.memory.data[start..start + n as usize].to_vec();
        let collision = bus.display.draw(vx, vy, &sprite);
        self.registers.v[0xF] = collision as u8;
        self.registers.df = true;
    }

    fn op_ex9e(&mut self, bus: &mut Bus, x: usize) {
        if bus.keypad.is_pressed(self.registers.v[x]) {
            self.registers.pc += 2;
        }
    }

    fn op_exa1(&mut self, bus: &mut Bus, x: usize) {
        if !bus.keypad.is_pressed(self.registers.v[x]) {
            self.registers.pc += 2;
        }
    }

    fn op_fx07(&mut self, x: usize) {
        self.registers.v[x] = self.registers.dt;
    }

    fn op_fx0a(&mut self, bus: &mut Bus, x: usize) {
        match bus.keypad.get_pressed_key() {
            Some(key) => self.registers.v[x] = key,
            None => self.registers.pc -= 2,
        }
    }

    fn op_fx15(&mut self, x: usize) {
        self.registers.dt = self.registers.v[x];
    }

    fn op_fx18(&mut self, x: usize) {
        self.registers.st = self.registers.v[x];
    }

    fn op_fx1e(&mut self, x: usize) {
        self.registers.i = self.registers.i.wrapping_add(self.registers.v[x] as u16);
    }

    fn op_fx29(&mut self, x: usize) {
        self.registers.i = self.registers.v[x] as u16 * 5;
    }

    fn op_fx33(&mut self, bus: &mut Bus, x: usize) {
        let vx = self.registers.v[x];
        bus.memory.write_u8(self.registers.i, vx / 100);
        bus.memory.write_u8(self.registers.i + 1, (vx / 10) % 10);
        bus.memory.write_u8(self.registers.i + 2, vx % 10);
    }

    fn op_fx55(&mut self, bus: &mut Bus, x: usize) {
        for idx in 0..=x {
            bus.memory
                .write_u8(self.registers.i + idx as u16, self.registers.v[idx]);
        }
    }

    fn op_fx65(&mut self, bus: &mut Bus, x: usize) {
        for idx in 0..=x {
            self.registers.v[idx] = bus.memory.read_u8(self.registers.i + idx as u16);
        }
    }
}
