use std::collections::VecDeque;

mod instructions;
mod font;

use instructions::Decoded;
use font::{FONTSET, FONTSET_SIZE};

// The following are public because they are used in the main.rs file
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096; // 4KB
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200; // 512, CHIP-8 programs start at this address

pub struct Emu {
    pc: u16, // program counter
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS], // general purpose registers
    i_reg: u16, // index register
    stack: VecDeque<u16>,
    keys: [bool; NUM_KEYS], // keypad
    dt: u8, // delay timer
    st: u8, // sound timer
}

impl Emu {
    pub fn new() -> Self {
        let mut my_emu: Emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            stack: VecDeque::with_capacity(STACK_SIZE),
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        // Load fontset into memory
        my_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        return my_emu;
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.stack = VecDeque::with_capacity(STACK_SIZE);
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // Fetch opcode
        let opcode = self.fetch();
        // Decode opcode
        let decoded = self.decode(opcode);
        // Execute opcode
        self.execute(decoded);
    }

    fn fetch(&mut self) -> u16 {
        let h_byte = self.ram[self.pc as usize] as u16;
        let l_byte = self.ram[(self.pc + 1) as usize] as u16;
        self.pc += 2;

        return (h_byte << 8) | l_byte; // Big-endian
    }

    fn decode(&mut self, opcode: u16) -> Decoded {
        // TODO
        let nibble3 = ((opcode & 0xF000) >> 12) as u8;
        let nibble2 = ((opcode & 0x0F00) >> 8) as u8;
        let nibble1 = ((opcode & 0x00F0) >> 4) as u8;
        let nibble0 = (opcode & 0x000F) as u8;

        match (nibble3, nibble2, nibble1, nibble0) {
            (0, 0, 0, 0)        => Decoded::NOP,
            (0, 0, 0xE, 0)      => Decoded::ClearScreen,
            (0, 0, 0xE, 0xE)    => Decoded::RET,
            (1, _, _, _)        => Decoded::Jump(opcode & 0x0FFF),
            (2, _, _, _)        => Decoded::Call(opcode & 0x0FFF),
            (3, _, _, _)        => Decoded::SkipEq(nibble2, nibble1 + nibble0),
            (4, _, _, _)        => Decoded::SkipNeq(nibble2, nibble1 + nibble0),
            (5, _, _, 0)        => Decoded::SkipEqReg(nibble2, nibble1),
            (6, _, _, _)        => Decoded::SetReg(nibble2, nibble1 + nibble0),
            (7, _, _, _)        => Decoded::AddReg(nibble2, nibble1 + nibble0),
            (8, _, _, 0)        => Decoded::SetRegReg(nibble2, nibble1),
            (8, _, _, 1)        => Decoded::Or(nibble2, nibble1),
            (8, _, _, 2)        => Decoded::And(nibble2, nibble1),
            (8, _, _, 3)        => Decoded::Xor(nibble2, nibble1),
            (8, _, _, 4)        => Decoded::AddRegReg(nibble2, nibble1),
            (8, _, _, 5)        => Decoded::SubRegReg(nibble2, nibble1),
            (_, _, _, _) => unimplemented!("Unknown opcode: {:#06x}", opcode),
        }
    }

    fn execute(&mut self, instruction: Decoded) {
        // TODO
        match instruction {
            Decoded::NOP             => (),
            Decoded::ClearScreen     => self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            Decoded::RET             => self.pc = self.stack.pop_back().expect("Stack underflow on RET"),
            Decoded::Jump(addr) => self.pc = addr,
            Decoded::Call(addr) => {
                self.stack.push_back(self.pc);
                self.pc = addr;
            },
            Decoded::SkipEq(x, value) => {
                if self.v_reg[x as usize] == value {
                    self.pc += 2;
                }
            },
            Decoded::SkipNeq(x, value) => {
                if self.v_reg[x as usize] != value {
                    self.pc += 2;
                }
            },
            Decoded::SkipEqReg(x, y ) => {
                if self.v_reg[x as usize] == self.v_reg[y as usize] {
                    self.pc += 2;
                }
            },
            Decoded::SetReg(x, value) => {
                self.v_reg[x as usize] = value;
            },
            Decoded::AddReg(x, value ) => {
                self.v_reg[x as usize] = self.v_reg[x as usize].wrapping_add(value);
            },
            Decoded::SetRegReg(x, y) => {
                self.v_reg[x as usize] = self.v_reg[y as usize];
            },
            Decoded::Or(x, y) => {
                self.v_reg[x as usize] |= self.v_reg[y as usize];
            },
            Decoded::And(x, y) => {
                self.v_reg[x as usize] &= self.v_reg[y as usize];
            },
            Decoded::Xor(x, y) => {
                self.v_reg[x as usize] ^= self.v_reg[y as usize];
            },
            Decoded::AddRegReg(x, y) => {
                let (result, overflow) = self.v_reg[x as usize].overflowing_add(self.v_reg[y as usize]);
                self.v_reg[x as usize] = result;
                self.v_reg[NUM_REGS - 1] = overflow as u8; 
            },
            Decoded::SubRegReg(x, y ) => {
                let (result, overflow) = self.v_reg[x as usize].overflowing_sub(self.v_reg[y as usize]);
                self.v_reg[x as usize] = result;
                self.v_reg[NUM_REGS - 1] = overflow as u8;
            },
            _ => unimplemented!("Unknown instruction: {:?}", instruction), // impossible to reach
        }
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            // Beep: I'll implement this later
            
            self.st -= 1;
        }
    }
}

