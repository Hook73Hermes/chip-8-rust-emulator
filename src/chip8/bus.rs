use super::ram::Ram;
use super::cpu::Cpu;
use super::display::Display;
use super::keypad::Keypad;

/* Bus che interconnette tutto l'hardware */
pub struct Bus {
    pub ram: Ram,
    pub cpu: Cpu,
    pub display: Display,
    pub keypad: Keypad,
}

impl Bus {
    /* Inizializza dell'hardware */
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(),
            cpu: Cpu::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    /* Carcica la rom in ram */
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.ram.write_rom(rom);
    }
}