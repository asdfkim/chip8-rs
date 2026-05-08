pub struct Registers {
    pub v: [u8; 16],
    pub i: u16,

    pub pc: u16, // program counter
    pub sp: u8,  // stack pointer

    pub dt: u8, // delay timer
    pub st: u8, // sound timer

    pub df: bool, // draw flag
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            dt: 0,
            st: 0,
            df: true,
        }
    }
}
