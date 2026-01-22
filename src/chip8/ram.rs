const RAM_SIZE: usize = 4096;

/* Ram con 4KB di memoria */
pub struct Ram {
    pub mem: [u8; RAM_SIZE],
}

impl Ram {
    /* Inizializza tutta la ram a 0 */
    pub fn new() -> Self {
        Ram { mem: [0; RAM_SIZE] }
    }

    /* Scrive un byte in memoria senza controllare che la locazione sia valida */
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.mem[addr as usize] = byte;
    }

    /* Legge u byte dalla memoria senza controllare che la locazione sia valida */
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    /*
    Copia il contenuto della rom dalla locazione 0x200 in poi
    Se la rom non può essere copiata in ram il calcolatore smette di funzionare
    */
    pub fn write_rom(&mut self, rom: &[u8]) {
        let start: usize = 0x200;
        let end: usize = start + rom.len();

        if end > RAM_SIZE {
            panic!(
                "ROM troppo grande: dimensione ROM = {}, dimensione RAM = {}",
                rom.len(),
                RAM_SIZE - start
            );
        }

        self.mem[start..end].copy_from_slice(rom);
    }
}
