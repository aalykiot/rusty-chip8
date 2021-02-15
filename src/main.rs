use std::{fs::File, io::Read};

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
}

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load("./roms/airplane.ch8");
}
