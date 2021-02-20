use crate::display::Display;
use std::fs::File;
use std::io::Read;

use rand::prelude::*;

pub type Address = u16;
pub type Register = u8;

pub const PROGRAM_MEMORY_OFFSET: usize = 0x200;

#[derive(Debug)]
pub enum Instruction {
    ClearDisplay,                       // 00E0
    Return,                             // 00EE
    Jump(Address),                      // 1nnn
    Call(Address),                      // 2nnn
    SkipIfEqualsByte(Register, u8),     // 3xkk
    SkipIfNotEqualsByte(Register, u8),  // 4xkk
    SkipIfEqual(Register, Register),    // 5xy0
    LoadByte(Register, u8),             // 6xkk
    AddByte(Register, u8),              // 7xkk
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
    Random(Register, u8),               // Cxkk
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

pub struct Chip8 {
    pc: u16,          // program counter
    v: [u8; 16],      // registers
    i: u16,           // i register
    sp: u8,           // stack pointer
    stack: [u16; 16], // stack
    v_delay: u8,      // delay register
    v_sound: u8,      // sound register
    memory: [u8; 4096],
    pub display: Display,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            pc: 0x200,
            v: [0; 16],
            i: 0,
            sp: 0,
            stack: [0; 16],
            v_delay: 0,
            v_sound: 0,
            memory: [0; 4096],
            display: Display::new(),
        }
    }

    pub fn load(&mut self, path: &str) {
        let memory = &mut self.memory[PROGRAM_MEMORY_OFFSET..];
        let mut file = File::open(path).unwrap();

        file.read(memory).unwrap();
    }

    pub fn to_instruction(&self, opcode: u16) -> Option<Instruction> {
        let nibbles = (
            (opcode >> 12) as u8,
            (opcode >> 8 & 0x0F) as u8,
            (opcode >> 4 & 0x00F) as u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;

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

    pub fn run_instruction(&mut self, instruction: Instruction) -> ProgramCounter {
        // run instruction and return next program counter
        match instruction {
            Instruction::ClearDisplay => {
                self.display.clear();
                ProgramCounter::Next
            }
            Instruction::Return => {
                let addr = self.stack[self.sp as usize - 1];
                self.sp -= 1;
                ProgramCounter::Jump(addr)
            }
            Instruction::Jump(addr) => ProgramCounter::Jump(addr),
            Instruction::Call(addr) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                ProgramCounter::Jump(addr)
            }
            Instruction::SkipIfEqualsByte(x, value) => {
                if self.read_register(x) == value {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }
            Instruction::SkipIfNotEqualsByte(x, value) => {
                if self.read_register(x) != value {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }
            Instruction::SkipIfEqual(x, y) => {
                if self.read_register(x) == self.read_register(y) {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }
            Instruction::LoadByte(x, value) => {
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::AddByte(x, value) => {
                let value = self.read_register(x).wrapping_add(value);
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::Move(x, y) => {
                let value = self.read_register(y);
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::Or(x, y) => {
                let value = self.read_register(x) | self.read_register(y);
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::And(x, y) => {
                let value = self.read_register(x) & self.read_register(y);
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::Xor(x, y) => {
                let value = self.read_register(x) ^ self.read_register(y);
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::Add(x, y) => {
                let value = self.read_register(x) as u16 + self.read_register(y) as u16;
                self.load_register(0xF, (value > 255) as u8);
                self.load_register(x, value as u8);
                ProgramCounter::Next
            }
            Instruction::Sub(x, y) => {
                let value = self.read_register(x).wrapping_sub(self.read_register(y));
                let difference = self.read_register(x) > self.read_register(y);
                self.load_register(0xF, difference as u8);
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::ShiftRight(x) => {
                let value = self.read_register(x) >> 1;
                self.load_register(0xF, self.read_register(x) & 0b1);
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::ReverseSub(x, y) => {
                let value = self.read_register(y).wrapping_sub(self.read_register(x));
                let difference = self.read_register(y) > self.read_register(x);
                self.load_register(0xF, difference as u8);
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::ShiftLeft(x) => {
                let value = self.read_register(x) << 1;
                self.load_register(0xF, self.read_register(x) >> 7);
                self.load_register(x, value);
                ProgramCounter::Next
            }
            Instruction::SkipIfNotEqual(x, y) => {
                if self.read_register(x) != self.read_register(y) {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }
            Instruction::LoadI(value) => {
                self.i = value;
                ProgramCounter::Next
            }
            Instruction::JumpPlusZero(address) => {
                let address = address + self.read_register(0) as u16;
                ProgramCounter::Jump(address)
            }
            Instruction::Random(x, value) => {
                let mut range = rand::thread_rng();
                let random = range.gen_range(0..255) as u8;
                self.load_register(x, random & value);
                ProgramCounter::Next
            }
            Instruction::Draw(x, y, n) => {
                let start = self.i as usize;
                let end = start + n as usize;
                let sprite = &self.memory[start..end];

                let x = self.read_register(x);
                let y = self.read_register(y);

                let collision = self.display.draw(x, y, sprite);

                self.load_register(0xF, collision as u8);

                ProgramCounter::Next
            }
            Instruction::SkipIfPressed(x) => {
                panic!("Instruction {:?} not yet implemented", instruction);
            }
            Instruction::SkipIfNotPressed(x) => {
                panic!("Instruction {:?} not yet implemented", instruction);
            }
            Instruction::LoadDelayTimer(x) => {
                self.load_register(x, self.v_delay);
                ProgramCounter::Next
            }
            Instruction::WaitForKeyPress(x) => {
                panic!("Instruction {:?} not yet implemented", instruction);
            }
            Instruction::SetDelayTimer(x) => {
                let value = self.read_register(x);
                self.v_delay = value;
                ProgramCounter::Next
            }
            Instruction::SetSoundTimer(x) => {
                let value = self.read_register(x);
                self.v_sound = value;
                ProgramCounter::Next
            }
            Instruction::AddToI(x) => {
                self.i += self.read_register(x) as u16;
                ProgramCounter::Next
            }
            Instruction::LoadSprite(x) => {
                panic!("Instruction {:?} not yet implemented", instruction);
            }
            Instruction::BCDRepresentation(x) => {
                let address = self.i as usize;
                let value = self.read_register(x);

                self.memory[address] = value / 100;
                self.memory[address + 1] = (value / 10) % 10;
                self.memory[address + 2] = (value / 1) % 10;

                ProgramCounter::Next
            }
            Instruction::StoreRegisters(x) => {
                let limit = x as usize + 1;
                let address = self.i as usize;

                for offset in 0..limit {
                    self.memory[address + offset] = self.read_register(offset as u8);
                }
                ProgramCounter::Next
            }
            Instruction::LoadRegisters(x) => {
                let limit = x as usize + 1;
                let address = self.i as usize;

                for offset in 0..limit {
                    self.load_register(offset as u8, self.memory[address + offset]);
                }
                ProgramCounter::Next
            }
        }
    }

    pub fn read_register(&self, idx: u8) -> u8 {
        self.v[idx as usize]
    }

    pub fn load_register(&mut self, idx: u8, value: u8) {
        self.v[idx as usize] = value;
    }

    pub fn cycle(&mut self) {
        let pc = self.pc as usize;
        let opcode: u16 = (self.memory[pc] as u16) << 8 | self.memory[pc + 1] as u16;

        let instruction = self.to_instruction(opcode).unwrap();
        let next = self.run_instruction(instruction);

        match next {
            ProgramCounter::Next => self.pc += 2,
            ProgramCounter::Skip => self.pc += 4,
            ProgramCounter::Jump(address) => self.pc = address,
        }
    }
}
