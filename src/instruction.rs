/*
  Information about chip8 opcodes
  http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
*/

pub type Address = u16;
pub type Register = u8;

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
