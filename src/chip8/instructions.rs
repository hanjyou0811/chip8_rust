use super::{Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};

impl Chip8 {
    // 0nnn
    pub fn cls(&mut self) {
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }
    // 00E0
    pub fn ret(&mut self) {
        self.pc = self.pop();
    }
    // 1nnn
    pub fn jp(&mut self, addr: u16) {
        self.pc = addr;
    }
    // 2nnn
    pub fn call(&mut self, addr: u16) {
        self.push(self.pc);
        self.pc = addr;
    }
    // 3xkk
    pub fn se_vx_byte(&mut self, x: usize, byte: u8) {
        if self.registers.v[x] == byte {
            self.pc += 2;
        }
    }
    // 4xkk
    pub fn sne_vx_byte(&mut self, x: usize, byte: u8) {
        if self.registers.v[x] != byte {
            self.pc += 2;
        }
    }
    // 5xy0
    pub fn se_vx_vy(&mut self, x: usize, y: usize) {
        if self.registers.v[x] == self.registers.v[y] {
            self.pc += 2;
        }
    }
    // 6xkk
    pub fn ld_vx_byte(&mut self, x: usize, byte: u8) {
        self.registers.v[x] = byte;
    }
    // 7xkk
    pub fn add_vx_byte(&mut self, x: usize, byte: u8) {
        self.registers.v[x] = self.registers.v[x].wrapping_add(byte);
    }
    // 8xy0
    pub fn ld_vx_vy(&mut self, x: usize, y: usize) {
        self.registers.v[x] = self.registers.v[y];
    }
    // 8xy1
    pub fn or_vx_vy(&mut self, x: usize, y: usize) {
        self.registers.v[x] |= self.registers.v[y];
    }
    // 8xy2
    pub fn and_vx_vy(&mut self, x: usize, y: usize) {
        self.registers.v[x] &= self.registers.v[y];
    }
    // 8xy3
    pub fn xor_vx_vy(&mut self, x: usize, y: usize) {
        self.registers.v[x] ^= self.registers.v[y];
    }
    // 8xy4
    pub fn add_vx_vy(&mut self, x: usize, y: usize) {
        let (result, carry) = self.registers.v[x].overflowing_add(self.registers.v[y]);
        self.registers.v[x] = result;
        self.registers.v[0xF] = if carry { 1 } else { 0 };
    }
    // 8xy5
    pub fn sub_vx_vy(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.registers.v[x].overflowing_sub(self.registers.v[y]);
        self.registers.v[x] = result;
        self.registers.v[0xF] = if borrow { 0 } else { 1 };
    }
    // 8xy6
    pub fn shr_vx(&mut self, x: usize) {
        self.registers.v[0xF] = self.registers.v[x] & 0x1; // Store least significant bit in VF
        self.registers.v[x] >>= 1;
    }
    // 8xy7
    pub fn subn_vx_vy(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.registers.v[y].overflowing_sub(self.registers.v[x]);
        self.registers.v[x] = result;
        self.registers.v[0xF] = if borrow { 0 } else { 1 };
    }
    // 8xyE
    pub fn shl_vx(&mut self, x: usize) {
        self.registers.v[0xF] = ((self.registers.v[x] & 0x80) >> 7) & 1; // Store most significant bit in VF
        self.registers.v[x] <<= 1;
    }
    // 9xy0
    pub fn sne_vx_vy(&mut self, x: usize, y: usize) {
        if self.registers.v[x] != self.registers.v[y] {
            self.pc += 2;
        }
    }
    // Annn
    pub fn ld_i_addr(&mut self, addr: u16) {
        self.registers.i = addr;
    }
    // Bnnn
    pub fn jp_v0_addr(&mut self, addr: u16) {
        self.jp(self.registers.v[0] as u16 + addr);
    }
    // Cxkk
    pub fn rnd_vx_byte(&mut self, x: usize, byte: u8) {
        let n: u8 = rand::random();
        self.registers.v[x] = n & byte;
    }
    // Dxyn
    pub fn drw_vx_vy_nibble(&mut self, x: usize, y: usize, n: usize) {
        let x_coord = self.registers.v[x] as u16;
        let y_coord = self.registers.v[y] as u16;
        let num_rows = n;

        let mut flipped = false;
        for y_line in 0..num_rows {
            let addr = self.registers.i + y_line as u16;
            let pixels = self.ram[addr as usize];
            for x_line in 0..8 {
                if (pixels & (0b1000_0000) >> x_line) != 0 {
                    let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                    let y = (y_coord + y_line as u16) as usize % SCREEN_HEIGHT;
                    let idx = x + SCREEN_WIDTH * y;
                    flipped |= self.screen[idx];
                    self.screen[idx] ^= true;
                }
            }
        }
        if flipped {
            self.registers.v[0xF] = 1;
        } else {
            self.registers.v[0xF] = 0;
        }
    }
    // Ex9E
    pub fn skp_vx(&mut self, x: usize) {
        if self.keypad[self.registers.v[x] as usize] {
            self.pc += 2;
        }
    }
    // ExA1
    pub fn sknp_vx(&mut self, x: usize) {
        if !self.keypad[self.registers.v[x] as usize] {
            self.pc += 2;
        }
    }
    // Fx07
    pub fn ld_vx_dt(&mut self, x: usize) {
        self.registers.v[x] = self.dt;
    }
    // Fx0A
    pub fn ld_vx_k(&mut self, x: usize) {
        let mut is_pressed = false;
        for i in 0..self.keypad.len() {
            if self.keypad[i] {
                self.registers.v[x] = i as u8;
                is_pressed = true;
                break;
            }
        }
        if !is_pressed {
            self.pc -= 2;
        }
    }
    // Fx15
    pub fn ld_dt_vx(&mut self, x: usize) {
        self.dt = self.registers.v[x];
    }
    // Fx18
    pub fn ld_st_vx(&mut self, x: usize) {
        self.st = self.registers.v[x];
    }
    // Fx1E
    pub fn add_i_vx(&mut self, x: usize) {
        self.registers.i = self.registers.i.wrapping_add(self.registers.v[x] as u16);
    }
    // Fx29
    pub fn ld_f_vx(&mut self, x: usize) {
        self.registers.i = (self.registers.v[x] as u16) * 5;
    }
    // Fx33
    pub fn ld_b_vx(&mut self, x: usize) {
        let value = self.registers.v[x];
        self.ram[self.registers.i as usize] = value / 100;
        self.ram[self.registers.i as usize + 1] = (value / 10) % 10;
        self.ram[self.registers.i as usize + 2] = value % 10;
    }
    // Fx55
    pub fn ld_i_vx(&mut self, x: usize) {
        for idx in 0..=x {
            self.ram[self.registers.i as usize + idx] = self.registers.v[idx];
        }
    }
    // Fx65
    pub fn ld_vx_i(&mut self, x: usize) {
        for idx in 0..=x {
            self.registers.v[idx] = self.ram[self.registers.i as usize + idx];
        }
    }
}
