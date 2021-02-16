use std::fs::File;
use std::io::Read;

type Address = u16;
type Register = u8;

#[derive(Debug)]
enum Instruction {
    ClearDisplay,                       // 00E0
    Return,                             // 00EE
    Jump(Address),                      // 1nnn
    Call(Address),                      // 2nnn
    SkipIfEqualsByte(Register, u16),    // 3xkk
    SkipIfNotEqualsByte(Register, u16), // 4xkk
    SkipIfEqual(Register, Register),    // 5xy0
    LoadByte(Register, u16),            // 6xkk
    AddByte(Register, u16),             // 7xkk
    Move(Register, Register),           // 8xy0
    Or(Register, Register),             // 8xy1
    And(Register, Register),            // 8xy2
    Xor(Register, Register),            // 8xy3
    Add(Register, Register),            // 8xy4
    Sub(Register, Register),            // 8xy5
    ShiftRight(Register),               // 8xy6
    ReverseSub(Register, Register),     // 8xy7
    ShiftLeft(Register),                // 8xyE
    SkipIfNotEqual(Register, Register), // 9xy0
    LoadI(u16),                         // Annn
    JumpPlusZero(Address),              // Bnnn
    Random(Register, u16),              // Cxkk
    Draw(Register, Register, u8),       // Dxyn
    SkipIfPressed(Register),            // Ex9E
    SkipIfNotPressed(Register),         // ExA1
    LoadDelayTimer(Register),           // Fx07
    WaitForKeyPress(Register),          // Fx0A
    SetDelayTimer(Register),            // Fx15
    SetSoundTimer(Register),            // Fx18
    AddToI(Register),                   // Fx1E
    LoadSprite(Register),               // Fx29
    BCDRepresentation(Register),        // Fx33
    StoreRegisters(Register),           // Fx55
    LoadRegisters(Register),            // Fx65
}

pub enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

struct Chip8 {
    pc: u16,            // program counter
    v: [u8; 16],        // registers
    i: u16,             // i register
    sp: u8,             // stack pointer
    stack: [u16; 16],   // stack
    v_delay: u8,        // delay register
    v_sound: u8,        // sound register
    memory: [u8; 4096], // memory
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            pc: 0x200,
            v: [0; 16],
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

    fn next_instruction(&self) -> Option<Instruction> {
        let pc = self.pc as usize;
        let opcode: u16 = (self.memory[pc] as u16) << 8 | self.memory[pc + 1] as u16;

        let nibbles = (
            (opcode >> 12) as u8,
            (opcode >> 8 & 0x0F) as u8,
            (opcode >> 4 & 0x00F) as u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = opcode & 0x0FFF;
        let kk = opcode & 0x00FF;

        let x = nibbles.1;
        let y = nibbles.2;
        let n = nibbles.3;

        match nibbles.0 {
            0x0 => match n {
                0x0 => Some(Instruction::ClearDisplay),
                0xE => Some(Instruction::Return),
                _ => None,
            },
            0x1 => Some(Instruction::Jump(nnn)),
            0x2 => Some(Instruction::Call(nnn)),
            0x3 => Some(Instruction::SkipIfEqualsByte(x, kk)),
            0x4 => Some(Instruction::SkipIfNotEqualsByte(x, kk)),
            0x5 => Some(Instruction::SkipIfEqual(x, y)),
            0x6 => Some(Instruction::LoadByte(x, kk)),
            0x7 => Some(Instruction::AddByte(x, kk)),
            0x8 => match n {
                0x0 => Some(Instruction::Move(x, y)),
                0x1 => Some(Instruction::Or(x, y)),
                0x2 => Some(Instruction::And(x, y)),
                0x3 => Some(Instruction::Xor(x, y)),
                0x4 => Some(Instruction::Add(x, y)),
                0x5 => Some(Instruction::Sub(x, y)),
                0x6 => Some(Instruction::ShiftRight(x)),
                0x7 => Some(Instruction::ReverseSub(x, y)),
                0xE => Some(Instruction::ShiftLeft(x)),
                _ => None,
            },
            0x9 => Some(Instruction::SkipIfNotEqual(x, y)),
            0xA => Some(Instruction::LoadI(nnn)),
            0xB => Some(Instruction::JumpPlusZero(nnn)),
            0xC => Some(Instruction::Random(x, kk)),
            0xD => Some(Instruction::Draw(x, y, n)),
            0xE => match kk {
                0x9E => Some(Instruction::SkipIfPressed(x)),
                0xA1 => Some(Instruction::SkipIfNotPressed(x)),
                _ => None,
            },
            0xF => match kk {
                0x07 => Some(Instruction::LoadDelayTimer(x)),
                0x0A => Some(Instruction::WaitForKeyPress(x)),
                0x15 => Some(Instruction::SetDelayTimer(x)),
                0x18 => Some(Instruction::SetSoundTimer(x)),
                0x1E => Some(Instruction::AddToI(x)),
                0x29 => Some(Instruction::LoadSprite(x)),
                0x33 => Some(Instruction::BCDRepresentation(x)),
                0x55 => Some(Instruction::StoreRegisters(x)),
                0x65 => Some(Instruction::LoadRegisters(x)),
                _ => None,
            },
            _ => None,
        }
    }
}

fn main() {
    let mut chip8 = Chip8::new();
    chip8.load("./programs/airplane.ch8");

    let next = chip8.next_instruction();
    println!("next chip8 instruction: {:?}", next);
}
