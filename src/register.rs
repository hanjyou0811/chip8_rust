pub const NUM_REGISTERS: usize = 16; // 16 general-purpose registers (V0 to VF)

pub struct Register {
    pub v: [u8; NUM_REGISTERS],    // 16 8-bit registers (V0 to VF)
    pub i: u16,     // 16-bit index register
}

impl Register {
    pub fn new() -> Self {
        Register {
            v: [0; NUM_REGISTERS], // Initialize all registers to 0
            i: 0, // Initialize index register to 0
        }
    }
}
