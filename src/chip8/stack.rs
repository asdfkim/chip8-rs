const STACK_SIZE: usize = 16;

pub struct Stack {
    pub data: [u16; STACK_SIZE],
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: [0; STACK_SIZE],
        }
    }

    pub fn push(&mut self, sp: &mut u8, value: u16) {
        *sp += 1;
        self.data[*sp as usize] = value;
    }

    pub fn pop(&mut self, sp: &mut u8) -> u16 {
        let value = self.data[*sp as usize];
        *sp -= 1;
        value
    }
}
