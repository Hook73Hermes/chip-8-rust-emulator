mod chip8;

use chip8::bus::Bus;
use chip8::display::{SCREEN_HEIGHT, SCREEN_WIDTH};
use minifb::{Key, Scale, Window, WindowOptions};
use std::fs::File;
use std::io::Read;

const CPU_TICKS_PER_FRAME: usize = 10;
const ROM: &str = "roms/space-invaders.ch8";

fn main() {
    /* Setup della finestra di emulazione */
    let mut window = Window::new(
        "Rust CHIP-8 Emulator",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limita la finestra a 60 FPS
    window.set_target_fps(60);

    /* Bus che interconnette tutto l'hardware */
    let mut bus = Bus::new();

    /* Carica la rom in ram */
    let rom_data = read_file(ROM);
    bus.load_rom(&rom_data);

    /* Ciclo del gioco */
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        process_input(&window, &mut bus);

        /* Esecuzione di vari cicli di CPU  */
        for _ in 0..CPU_TICKS_PER_FRAME {
            bus.tick_cpu();
        }

        // Aggiorna Timer a 60 Hz
        if bus.cpu.delay_timer > 0 {
            bus.cpu.delay_timer -= 1;
        }
        if bus.cpu.sound_timer > 0 {
            bus.cpu.sound_timer -= 1;
        }

        /* Aggiornamento dello schermo */
        let buffer = buffer_to_u32(&bus.display.buffer, SCREEN_WIDTH, SCREEN_HEIGHT);
        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}

/* Legge un file restituendolo come array di caratteri */
fn read_file(path: &str) -> Vec<u8> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Impossibile aprire il file {}: {}", path, e),
    };

    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer) {
        Ok(_) => buffer,
        Err(e) => panic!("Impossibile leggere il file {}: {}", path, e),
    }
}

/* Trasforma il buffer di bool in buffer di RGB */
fn buffer_to_u32(buffer: &[bool], width: usize, height: usize) -> Vec<u32> {
    let mut result = vec![0; width * height];

    for (i, pixel) in buffer.iter().enumerate() {
        if *pixel {
            result[i] = 0xFFFFFFFF;
        } else {
            result[i] = 0xFF000000;
        }
    }
    result
}

/* Processa un input dal tastierino numerico */
fn process_input(window: &Window, bus: &mut Bus) {
    let key_map = [
        (Key::X, 0x0),
        (Key::Key1, 0x1),
        (Key::Key2, 0x2),
        (Key::Key3, 0x3),
        (Key::Q, 0x4),
        (Key::W, 0x5),
        (Key::E, 0x6),
        (Key::A, 0x7),
        (Key::S, 0x8),
        (Key::D, 0x9),
        (Key::Z, 0xA),
        (Key::C, 0xB),
        (Key::Key4, 0xC),
        (Key::R, 0xD),
        (Key::F, 0xE),
        (Key::V, 0xF),
    ];

    for i in 0..16 {
        bus.keypad.set_pressed(i, false);
    }

    for (key_code, chip8_key) in key_map {
        if window.is_key_down(key_code) {
            bus.keypad.set_pressed(chip8_key, true);
        }
    }
}
