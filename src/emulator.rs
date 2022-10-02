use crate::font_set::*;
use std::fs::File;
use std::io::prelude::*;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const START_ADDRESS: u16 = 0x200;
pub struct Emulator {
    ram: [u16; 4096],
    display: [usize; SCREEN_WIDTH * SCREEN_HEIGHT],
    pc: u16,
    i_reg: u16,
    v_reg: [u8; 16],
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut emulator = Emulator { 
            ram: [0; 4096], 
            display: [0; SCREEN_WIDTH * SCREEN_HEIGHT], 
            pc: START_ADDRESS, 
            i_reg: 0, 
            v_reg: [0; 16], 
            stack: [0; 16], 
            delay_timer: 0, 
            sound_timer: 0 };
            
            for (i, &data) in FONT_SET.iter().enumerate() {
                emulator.ram[FONT_SET_START_ADDRESS as usize + i] = data as u16;
            }

            emulator
    }
    pub fn load_rom(&mut self, filename: &str)  {
        let mut file = File::open(filename).unwrap();
        let mut file_content: Vec<u8> = Vec::new();
        
        match file.read_to_end(&mut file_content) {
            Ok(_) => {
                // load ROM content into memory
                for (i, &data) in file_content.iter().enumerate() {
                    self.ram[START_ADDRESS as usize + i] = data as u16;
                }
                print!("{:?}", self.ram);
            },
            Err(err) => panic!("Problem opening the file, {:?}", err)

        };
    }
}