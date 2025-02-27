#[derive(Debug)]
pub enum Decoded {
    // TODO
    NOP,                // 0x0000
    ClearScreen,        // 0x00E0
    RET,                // 0x00EE
    Jump(u16),          // 0x1NNN (Chip-8 addresses are 12-bit wide)
    Call(u16),          // 0x2NNN
    SkipEq(u8, u8),     // 0x3XNN
    SkipNeq(u8, u8),    // 0x4XNN
    SkipEqReg(u8, u8),  // 0x5XY0
    SetReg(u8, u8),     // 0x6XNN
    AddReg(u8, u8),     // 0x7XNN
    SetRegReg(u8, u8),  // 0x8XY0
    Or(u8, u8),         // 0x8XY1
    And(u8, u8),        // 0x8XY2
    Xor(u8, u8),        // 0x8XY3
    AddRegReg(u8, u8),  // 0x8XY4
    SubRegReg(u8, u8),  // 0x8XY5
    RightShift(u8),     // 0x8XY6
    SubRegRegRev(u8, u8), // 0x8XY7
    LeftShift(u8),      // 0x8XYE
}
