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
}
