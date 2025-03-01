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
    SkipNeqReg(u8, u8), // 0x9XY0
    SetIReg(u16),       // 0xANNN
    JumpOffset(u16),    // 0xBNNN
    Rand(u8, u8),       // 0xCXNN
    Draw(u8, u8, u8),   // 0xDXYN
    SkipKey(u8),        // 0xEX9E
    SkipNKey(u8),       // 0xEXA1
    GetDelay(u8),       // 0xFX07
    WaitKey(u8),        // 0xFX0A
    SetDelay(u8),       // 0xFX15
    SetSound(u8),       // 0xFX18
    AddIReg(u8),        // 0xFX1E
    SetIRegFont(u8),    // 0xFX29
    StoreBCD(u8),       // 0xFX33
    StoreRegsToMem(u8), // 0xFX55
    LoadMemToRegs(u8),    // 0xFX65
}
