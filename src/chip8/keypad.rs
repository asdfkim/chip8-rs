const KEYPAD_SIZE: usize = 16;

pub struct Keypad {
    pub keys: [bool; KEYPAD_SIZE],
}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys: [false; KEYPAD_SIZE],
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn press(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn release(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.keys.iter().position(|&k| k).map(|k| k as u8)
    }
}
