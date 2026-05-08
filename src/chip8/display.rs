const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    vram: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Display {
            vram: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (row, byte) in sprite.iter().enumerate() {
            for col in 0..8 {
                let pixel = (byte >> (7 - col)) & 1 == 1;
                if !pixel {
                    continue;
                }

                let px = (x as usize + col) % DISPLAY_WIDTH;
                let py = (y as usize + row) % DISPLAY_HEIGHT;
                let idx = py * DISPLAY_WIDTH + px;

                if self.vram[idx] {
                    collision = true;
                }

                self.vram[idx] ^= true;
            }
        }

        collision
    }

    pub fn clear(&mut self) {
        self.vram = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }
}
