use crate::{
    register::Register,
    font::{FONTSET, FONTSET_SIZE},
};
mod instructions;

const RAM_SIZE: usize = 0x4096; // 4KB of memory
const STACK_SIZE: usize = 16; // Stack can hold up to 16 return addresses
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const NUM_KEYS: usize = 16; // 16 keys (0-9, A-F)
const PROGRAM_START: u16 = 0x200; // Programs start at memory address 0x200

pub struct Chip8 {
    pc: u16, // Program counter
    pub ram: [u8; RAM_SIZE], // 4KB of memory
    registers: Register, // General-purpose registers
    sp: u8, // Stack pointer
    stack: [u16; STACK_SIZE], // Stack for subroutine calls (up to 16 levels)
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], // Monochrome display (64x32 pixels)
    keypad: [bool; NUM_KEYS], // Hexadecimal keypad state (16 keys)
    dt: u8, // Delay timer
    st: u8, // Sound timer
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            pc: PROGRAM_START, // Start program counter at 0x200
            ram: [0; RAM_SIZE], // Initialize RAM to 0
            registers: Register::new(), // Initialize registers
            sp: 0, // Initialize stack pointer to 0
            stack: [0; STACK_SIZE], // Initialize stack to 0
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT], // Initialize screen to all off
            keypad: [false; NUM_KEYS], // Initialize keypad to all off
            dt: 0, // Initialize delay timer
            st: 0, // Initialize sound timer
        };
        chip8.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET); // Load fontset into RAM
        chip8
    }
    fn push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
    pub fn load(&mut self, data: &[u8]) {
        let start = PROGRAM_START as usize;
        let end = start + data.len();
        self.ram[start..end].copy_from_slice(data);
    }
    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keypad[idx] = pressed;
    }
    pub fn emulate(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }
    pub fn emulate_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                // TODO: Play sound
            }
            self.st -= 1;
        }
    }
    fn fetch(&mut self) -> u16 {
        let hi = self.ram[self.pc as usize] as u16;
        let lo = self.ram[(self.pc + 1) as usize] as u16;
        let op = (hi << 8) | lo;
        self.pc += 2;
        op
    }
    fn execute(&mut self, op: u16) {
        let d1 = (op & 0xF000) >> 12;
        let d2 = (op & 0x0F00) >> 8;
        let d3 = (op & 0x00F0) >> 4;
        let d4 = op & 0x000F;

        match (d1, d2, d3, d4) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => self.cls(),
            (0, 0, 0xE, 0xE) => self.ret(),
            (1, _, _, _) => self.jp(op & 0x0FFF),
            (2, _, _, _) => self.call(op & 0x0FFF),
            (3, x, _, _) => self.se_vx_byte(x as usize, (op & 0x00FF) as u8),
            (4, x, _, _) => self.sne_vx_byte(x as usize, (op & 0x00FF) as u8),
            (5, x, y, 0) => self.se_vx_vy(x as usize, y as usize),
            (6, x, _, _) => self.ld_vx_byte(x as usize, (op & 0x00FF) as u8),
            (7, x, _, _) => self.add_vx_byte(x as usize, (op & 0x00FF) as u8),
            (8, x, y, 0) => self.ld_vx_vy(x as usize, y as usize),
            (8, x, y, 1) => self.or_vx_vy(x as usize, y as usize),
            (8, x, y, 2) => self.and_vx_vy(x as usize, y as usize),
            (8, x, y, 3) => self.xor_vx_vy(x as usize, y as usize),
            (8, x, y, 4) => self.add_vx_vy(x as usize, y as usize),
            (8, x, y, 5) => self.sub_vx_vy(x as usize, y as usize),
            (8, x, _, 6) => self.shr_vx(x as usize),
            (8, x, y, 7) => self.subn_vx_vy(x as usize, y as usize),
            (8, x, _, 0xE) => self.shl_vx(x as usize),
            (9, x, y, 0) => self.sne_vx_vy(x as usize, y as usize),
            (0xA, _, _, _) => self.ld_i_addr(op & 0x0FFF),
            (0xB, _, _, _) => self.jp_v0_addr(op & 0x0FFF),
            (0xC, x, _, _) => self.rnd_vx_byte(x as usize, (op & 0x00FF) as u8),
            (0xD, x, y, n) => self.drw_vx_vy_nibble(x as usize, y as usize, n as usize),
            (0xE, x, 9, 0xE) => self.skp_vx(x as usize),
            (0xE, x, 0xA, 1) => self.sknp_vx(x as usize),
            (0xF, x, 0, 7) => self.ld_vx_dt(x as usize),
            (0xF, x, 0, 0xA) => self.ld_vx_k(x as usize),
            (0xF, x, 1, 5) => self.ld_dt_vx(x as usize),
            (0xF, x, 1, 8) => self.ld_st_vx(x as usize),
            (0xF, x, 1, 0xE) => self.add_i_vx(x as usize),
            (0xF, x, 2, 9) => self.ld_f_vx(x as usize),
            (0xF, x, 3, 3) => self.ld_b_vx(x as usize),
            (0xF, x, 5, 5) => self.ld_i_vx(x as usize),
            (0xF, x, 6, 5) => self.ld_vx_i(x as usize),
            (_, _, _, _) => panic!("Unknown opcode: {:04X}", op),
        }
    }
    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }
}
