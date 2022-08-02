/// Every program starts in 512 memory address;
const START_ADDR: u16 = 0x200;
/// 64x32 Screen monocrome (1 bit per pixel).
/// These are pub cause they will called by frontend.
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
const RAM_SIZE: usize = 4_096;
/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.2
const NUM_REGS: usize = 16;
/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.2
const STACK_SIZE: usize = 16;
/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.3
const NUM_KEYS: usize = 16;

/// ******** CHIP- 8  Technical Reference v1.0 ********
/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0
///
pub struct Emu {
    // Program Counter
    pc: u16,
    /// Since Chip-8 was never a phisicall system, but it is developed to be implemented
    /// on computers with 4KB (4096) of RAM
    ram: [u8; RAM_SIZE],
    /// Since it's a 1 bit display (b&w) we can represent with an array of boolean values
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    /// Chip-8 has 16 registers V0 to VF
    v_reg: [u8; NUM_REGS],
    /// Another i register for, which is used for indexing into RAM for read/write operations.
    i_reg: u16,
    /// The stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack.
    sp: u16,
    ///  The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to when finished with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.
    stack: [u16; STACK_SIZE],
    /// Keyboard keys
    keys: [bool; NUM_KEYS],
    /// Another registers Delay timer and Sound timer
    dt: u8,
    /// Emite noise when clock hitting 0
    st: u8,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }
}

// Push/Pop methods

impl Emu {
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self, val: u16) {
        // Note: verify if stack is not empty or result in underflow panic
        self.sp -= 1;
        self.stack[self.sp as usize] = val;
    }
}
