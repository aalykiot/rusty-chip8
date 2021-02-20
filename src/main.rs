mod chip8;
mod display;

use chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load("./programs/ibm-logo.ch8");
    chip8.cycle();
}
