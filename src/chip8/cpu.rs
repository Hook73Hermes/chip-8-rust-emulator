use super::display::Display;
use super::display::SCREEN_HEIGHT;
use super::display::SCREEN_WIDTH;
use super::keypad::Keypad;
use super::ram::Ram;
use rand::Rng;

const N_REGS: usize = 16;
const STACK_SIZE: usize = 16;

// Cpu di un CHIP-8
pub struct Cpu {
    // Registri generali
    pub v: [u8; N_REGS],

    // Registri indice e program counter
    pub i: u16,
    pub pc: u16,

    // Stack e registro stack pointer
    pub stack: [u16; STACK_SIZE],
    pub sp: u8,

    // Timer di delay e del suono
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Cpu {
    // I programmi utente cominciano dopo 512 byte (0x200)
    pub fn new() -> Self {
        Cpu {
            v: [0; N_REGS],
            i: 0,
            pc: 0x200,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    // Esegue un passo della cpu
    pub fn tick(&mut self, ram: &mut Ram, display: &mut Display, keypad: &mut Keypad) {
        let opcode = self.fetch(ram);
        self.execute(opcode, ram, display, keypad);
    }

    // Fetcha l'istruzione su 2 byte e incrementa di due il program counter
    fn fetch(&mut self, ram: &Ram) -> u16 {
        let pc = self.pc;
        self.pc += 2;

        let byte_high = ram.read_byte(pc) as u16;
        let byte_low = ram.read_byte(pc + 1) as u16;
        (byte_high << 8) | byte_low
    }

    // Esegue l'istruzione
    fn execute(&mut self, opcode: u16, ram: &mut Ram, display: &mut Display, keypad: &mut Keypad) {
        let op = (opcode & 0xF000) >> 12;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match (op, x, y, n) {
            // 0000 NOP
            (0, 0, 0, 0) => {}

            // 00E0 CLS: pulisce lo schermo
            (0, 0, 0xE, 0) => {
                display.clear();
            }

            // 00EE RET: ritorno da una routine
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            // 1NNN JMP: salta all'indirizzo
            (1, _, _, _) => {
                self.pc = nnn;
            }

            // 2NNN CALL addr: chiama una routine
            (2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }

            // 3XNN SE Vx byte: salta la prossima istruzione se Vx == byte
            (3, _, _, _) => {
                if self.v[x] == nn {
                    self.pc += 2;
                }
            }

            // 4XNN SNE Vx byte: salta la prossima istruzione se Vx != byte
            (4, _, _, _) => {
                if self.v[x] != nn {
                    self.pc += 2;
                }
            }

            // 5XY0 SE Vx Vy: salta la prossima istruzione se Vx == Vy
            (5, _, _, 0) => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }

            // 6XNN LD Vx byte: carica un byte nel registro Vx
            (6, _, _, _) => {
                self.v[x] = nn;
            }

            // 7XNN ADD Vx byte: somma un byte a un registro Vx
            (7, _, _, _) => {
                self.v[x] = self.v[x].wrapping_add(nn);
            }

            // 8XYn: Operazioni Aritmetiche e Logiche
            (8, _, _, _) => match n {
                // LD Vx Vy: carica Vy in Vx
                0 => {
                    self.v[x] = self.v[y];
                }

                // OR Vx Vy: carica Vy | Vx in Vx
                1 => {
                    self.v[x] |= self.v[y];
                }

                // AND Vx Vy: carica Vy & Vx in Vx
                2 => {
                    self.v[x] &= self.v[y];
                }

                // XOR Vx Vy: carica Vy ^ Vx in Vx
                3 => {
                    self.v[x] ^= self.v[y];
                }

                // ADD Vx Vy: carica Vx + Vy in Vx e il riporto in VF
                4 => {
                    let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                    self.v[x] = res;
                    self.v[0xF] = if overflow { 1 } else { 0 };
                }

                // SUB Vx Vy: carica Vx - Vy in Vx e il prestito in VF
                5 => {
                    let (res, borrow) = self.v[x].overflowing_sub(self.v[y]);
                    self.v[x] = res;
                    self.v[0xF] = if !borrow { 1 } else { 0 };
                }

                // SHR Vx: shifta a destra Vx
                6 => {
                    self.v[0xF] = self.v[x] & 0x1; // Salva il bit perso in VF
                    self.v[x] >>= 1;
                }

                // SUBN Vx Vy: carica Vy - Vx in Vx e il prestito in VF
                7 => {
                    let (res, borrow) = self.v[y].overflowing_sub(self.v[x]);
                    self.v[x] = res;
                    self.v[0xF] = if !borrow { 1 } else { 0 };
                }

                // SHL Vx: shifta a sinistra Vx
                0xE => {
                    self.v[0xF] = (self.v[x] >> 7) & 0x1;
                    self.v[x] <<= 1;
                }

                // Opcode non valido
                _ => {
                    panic!("Opcode ALU non riconosciuto: {:04X}", opcode);
                }
            },

            // 9XY0 SNE Vx Vy: salta la prossima istruzione se Vx != Vy
            (9, _, _, 0) => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }

            // ANNN LD I addr: carica un indirizzo nel registro I
            (0xA, _, _, _) => {
                self.i = nnn;
            }

            // BNNN JP V0 addr: salta a nnn + V0
            (0xB, _, _, _) => {
                self.pc = nnn + self.v[0] as u16;
            }

            // CXNN RND Vx byte: genera byte randomico
            (0xC, _, _, _) => {
                let randomg: u8 = rand::rng().random();
                self.v[x] = randomg & nn;
            }

            // DXYN DRW Vx Vy byte: disegna uno sprite a schermo
            (0xD, _, _, _) => {
                let x = (self.v[x] as usize) % SCREEN_WIDTH;
                let y = (self.v[y] as usize) % SCREEN_HEIGHT;
                let height = n;

                self.v[0xF] = 0;
                for row in 0..height {
                    let cur_y = (y + row as usize) % SCREEN_HEIGHT;
                    let sprite_byte = ram.read_byte(self.i + row as u16);
                    for col in 0..8 {
                        let cur_x = (x + col as usize) % SCREEN_WIDTH;
                        if (sprite_byte >> (7 - col)) & 1 == 1 {
                            let screen_index = cur_x + cur_y * SCREEN_WIDTH;
                            if display.buffer[screen_index] {
                                self.v[0xF] = 1;
                            }
                            display.buffer[screen_index] ^= true;
                        }
                    }
                }
            }

            // EX9E SKP Vx: Salta la prossima istruzione se la chiave salvata in Vx è premuta
            (0xE, _, 0x9, 0xE) => {
                let key = self.v[x] as usize;
                if keypad.is_pressed(key) {
                    self.pc += 2;
                }
            }

            // EXA1 SKNP Vx: Salta la prossima istruzione se la chiave salvata in Vx non è premuta
            (0xE, _, 0xA, 0x1) => {
                let key = self.v[x] as usize;
                if !keypad.is_pressed(key) {
                    self.pc += 2;
                }
            }

            // FX0A LD Vx K: blocca l'esecuzione finché non viene premuto un tasto
            (0xF, _, 0x0, 0xA) => {
                let mut pressed = false;
                for i in 0..16 {
                    if keypad.is_pressed(i) {
                        self.v[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.pc -= 2;
                }
            }

            // FX series: Vari opcodes
            (0xF, _, _, _) => match nn {
                // LD Vx DT: carica il delay timer in Vx
                0x07 => {
                    self.v[x] = self.delay_timer;
                }

                // LD DT Vx: carica Vx nel delay timer
                0x15 => {
                    self.delay_timer = self.v[x];
                }

                // LD ST Vx: carica il sound timer in Vx
                0x18 => {
                    self.sound_timer = self.v[x];
                }

                // ADD I Vx: somma Vx a I
                0x1E => {
                    self.i = self.i.wrapping_add(self.v[x] as u16);
                }

                // LD F Vx: punta I allo sprite del font
                0x29 => {
                    self.i = 0x50 + (self.v[x] as u16 * 5);
                }

                // LD B Vx: binary coded decimal
                0x33 => {
                    let val = self.v[x];
                    ram.write_byte(self.i, val / 100);
                    ram.write_byte(self.i + 1, (val / 10) % 10);
                    ram.write_byte(self.i + 2, val % 10);
                }

                // LD [I] Vx: salva i registri nella ram
                0x55 => {
                    for idx in 0..=x {
                        ram.write_byte(self.i + idx as u16, self.v[idx]);
                    }
                }

                // LD Vx [I]: carica i registri dalla ram
                0x65 => {
                    for idx in 0..=x {
                        self.v[idx] = ram.read_byte(self.i + idx as u16);
                    }
                }

                _ => panic!("Opcode FX non riconosciuto: {:04X}", opcode),
            },

            // Opcode non valido
            _ => panic!("Opcode non riconosciuto: {:04X}", opcode),
        }
    }
}
