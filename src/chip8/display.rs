pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const N_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

/* Display 64x32 con pixel bianchi (true) o neri (false) */
pub struct Display {
    pub buffer: [bool; N_PIXELS],
}

impl Display {
    /* Inizializza tutti i pixel dello schermo spenti */
    pub fn new() -> Self {
        Display {
            buffer: [false; N_PIXELS],
        }
    }

    /* Spegne tutti i pixel dello schermo */
    pub fn clear(&mut self) {
        self.buffer = [false; N_PIXELS];
    } 
}