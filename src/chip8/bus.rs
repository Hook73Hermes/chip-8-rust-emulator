use super::cpu::Cpu;
use super::display::Display;
use super::keypad::Keypad;
use super::ram::Ram;

// I byte che rappresentano i caratteri da 0 a F
const FONTSET: [u8; 80] = [
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

/* Bus che interconnette tutto l'hardware */
pub struct Bus {
    pub ram: Ram,
    pub cpu: Cpu,
    pub display: Display,
    pub keypad: Keypad,
}

impl Bus {
    /* Inizializza l'hardware e carica il fontset */
    pub fn new() -> Self {
        let mut bus = Bus {
            ram: Ram::new(),
            cpu: Cpu::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        };

        for (i, &byte) in FONTSET.iter().enumerate() {
            bus.ram.write_byte(0x50 + i as u16, byte);
        }

        bus
    }

    /* Carica la rom in ram */
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.ram.write_rom(rom);
    }

    /* Esegue un passo della cpu */
    pub fn tick_cpu(&mut self) {
        self.cpu
            .tick(&mut self.ram, &mut self.display, &mut self.keypad);
    }
}
