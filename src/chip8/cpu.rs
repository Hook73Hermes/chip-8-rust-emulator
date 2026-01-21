const N_REGS: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Cpu {
    /* Registri generali */
    pub v: [u8, N_REGS],

    /* Registri indice e program counter */
    pub i: u16,
    pub pc: u16,

    /* Stack e registro stack pointer */
    pub stack:  [u16, STACK_SIZE],
    pub sp: u8,

    /* Timer di delay e del suono */
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Cpu {
    pub fn new() -> self {
        v: [0, N_REGS],
        i: 0,
        pc: 0x200,
        stack: [0, STACK_SIZE],
        sp: 0,
        delay_timer: 0,
        sound_timer: 0,
    }
}