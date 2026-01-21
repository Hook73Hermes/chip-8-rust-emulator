use std::fs::File;
use std::io::Read;
use std::path::Path;

mod chip8;
use chip8::bus::Bus;

fn main() {
    /* Bus che interconnette tutto l'hardware */
    let mut bus = Bus::new();

    /* Carica la rom in ram */
    let rom_path = "roms/IBM-logo.ch8";
    let rom_data = read_file(rom_path);
    bus.load_rom(&rom_data);

    /* Verifica il contenuto della ram */
    println!("Primi due byte della RAM: {:02X}{:02x}", bus.ram.read_byte(0x200), bus.ram.read_byte(0x201));
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