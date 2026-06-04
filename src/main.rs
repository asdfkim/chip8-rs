use std::{env, fs};

use chp8_rs::chip8::{
    Chip8,
    display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Display},
    keypad::Keypad,
};
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const FPS: usize = 60;
const CPU_HZ: usize = 600;
const CYCLES_PER_FRAME: usize = CPU_HZ / FPS;
const COLOR_ON: u32 = 0x00FFFFFF;
const COLOR_OFF: u32 = 0x00000000;

const KEY_MAP: [(Key, u8); 16] = [
    (Key::Key1, 0x1),
    (Key::Key2, 0x2),
    (Key::Key3, 0x3),
    (Key::Key4, 0xC),
    (Key::Q, 0x4),
    (Key::W, 0x5),
    (Key::E, 0x6),
    (Key::R, 0xD),
    (Key::A, 0x7),
    (Key::S, 0x8),
    (Key::D, 0x9),
    (Key::F, 0xE),
    (Key::Z, 0xA),
    (Key::X, 0x0),
    (Key::C, 0xB),
    (Key::V, 0xF),
];

fn main() {
    let mut args = env::args();
    let exe = args.next().unwrap_or_else(|| "chip8-rs".to_string());
    let rom_path = args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} <path/to/rom>", exe);
        std::process::exit(1);
    });

    let rom = fs::read(&rom_path).unwrap_or_else(|err| {
        eprintln!("Failed to read ROM '{}': {}", rom_path, err);
        std::process::exit(1);
    });

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);

    let mut window = Window::new(
        "chip8-rs",
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X16,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|err| {
        eprintln!("Unable to open window: {}", err);
        std::process::exit(1);
    });

    window.set_target_fps(FPS);

    let mut buffer = vec![COLOR_OFF; DISPLAY_WIDTH * DISPLAY_HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        sync_keypad(&window, &mut chip8.bus.keypad);

        for _ in 0..CYCLES_PER_FRAME {
            chip8.cpu.tick(&mut chip8.bus);
        }
        chip8.cpu.tick_timers();

        render_display(&chip8.bus.display, &mut buffer);

        if let Err(err) = window.update_with_buffer(&buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT) {
            eprintln!("Window update error: {}", err);
            break;
        }
    }
}

fn sync_keypad(window: &Window, keypad: &mut Keypad) {
    for (key, hex) in KEY_MAP {
        if window.is_key_down(key) {
            keypad.press(hex);
        } else {
            keypad.release(hex);
        }
    }
}

fn render_display(display: &Display, buffer: &mut [u32]) {
    for (idx, pixel) in display.vram.iter().enumerate() {
        buffer[idx] = if *pixel { COLOR_ON } else { COLOR_OFF };
    }
}
