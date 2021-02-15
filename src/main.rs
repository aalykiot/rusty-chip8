use std::fs::File;
use std::io::Read;
struct Chip8 {
    pc: u16,
    vx: [u8; 16],
    i: u16,
    sp: u8,
    stack: [u16; 16],
    v_delay: u8,
    v_sound: u8,
    memory: [u8; 4096],
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            pc: 0x200,
            vx: [0; 16],
            i: 0,
            sp: 0,
            stack: [0; 16],
            v_delay: 0,
            v_sound: 0,
            memory: [0; 4096],
        }
    }

    fn load(&mut self, path: &str) {
        // program space starts at address 0x200
        let low = 0x200;
        // open chip8 rom file
        let mut file = File::open(path).unwrap();
        // create a slice of the cpu's memory
        let buffer = &mut self.memory[low..];
        // read bytes into memory
        file.read(buffer).unwrap();
    }

    fn run(&mut self) {
        // convert u16 to usize to get the byte from memory
        let pc = self.pc as usize;
        // compute opcode from memory addresses
        let opcode: u16 = (self.memory[pc] as u16) << 8 | self.memory[pc + 1] as u16;
        // compute opcode nibbles
        let nibbles = (
            (opcode >> 12) as u8,
            (opcode >> 8 & 0x0F) as u8,
            (opcode >> 4 & 0x00F) as u8,
            (opcode & 0x000F) as u8,
        );
        // compute opcode variables
        let nnn = opcode & 0x0FFF;
        let kk = opcode & 0x00FF;
        let x = nibbles.1;
        let y = nibbles.2;
        let n = nibbles.3;
    }
}

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load("./programs/airplane.ch8");
    chip8.run();
}
