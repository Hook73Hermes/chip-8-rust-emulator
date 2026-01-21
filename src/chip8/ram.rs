const RAM_SIZE: usize = 4096;

/* Ram con 4KB di memoria */
pub struct Ram {
    pub mem: [u8; RAM_SIZE],
}

impl Ram {
    /* Inizializza tutta la ram a 0 */
    pub fn new() -> Self {
        Ram {
            mem: [0; RAM_SIZE],
        }
    }

    /* Scrive un byte in memoria senza controllare che la locazione sia valida */
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.mem[addr as usize] = byte;
    }

    /* Legge u byte dalla memoria senza controllare che la locazione sia valida */
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
}