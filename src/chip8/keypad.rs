const N_KEYS: usize = 16;

/* Tastierino con 16 tasti che possono essere premuti e rilasciati */
pub struct Keypad {
    pub keys: [bool, N_KEYS],
}

impl Keypad {
    /* Inizializza tutti i tasti come rilasciati */
    pub fn new() -> self {
        keys: [false, N_KEYS],
    }

    /* Preme o rilascia un tasto */
    pub fn set_pressed(&mut self, index: usize, pressed: bool) {
        self.keys[index] = pressed;
    }

    /* Dice se un tasto è premuto */
    pub fn is_pressed(&self, index: usize) -> bool {
        self.keys[index]
    }
}