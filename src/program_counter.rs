pub enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}
