mod chip8;
use chip8::bus::Bus;

fn main() {
    let mut bus = Bus::new();

    println!("Attuale PC {}", bus.cpu.pc);
}