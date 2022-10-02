use std::fs::File;
use std::io::prelude::*;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct Emulator {
    ram: [usize; 4096],
    display: [usize; SCREEN_WIDTH * SCREEN_HEIGHT],
    pc: u16,
    i_reg: u16,
    v_reg: [u8; 16],
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Emulator {
    pub fn loadROM(filename: &str) -> std::io::Result<(Vec<u8>)> {
        let mut file = File::open(filename)?;
        let mut file_content: Vec<u8> = Vec::new();
        file.read_to_end(&mut file_content);

        Ok((file_content))
    }
}