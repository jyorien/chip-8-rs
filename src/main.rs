mod font_set;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

struct Emulator {
    ram: [usize; 4096],
    display: [usize; SCREEN_WIDTH * SCREEN_HEIGHT],
    pc: u16,
    i_reg: u16,
    v_reg: [u8; 16],
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
}

fn main() {
    
}
