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
    /* Inizializzazione dell'hardware */
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(),
            cpu: Cpu::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }
}