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
    sp: u16,
    i_reg: u16,
    v_reg: [u8; 16],
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    opcode: u16
}

impl Emulator {
    pub fn new() -> Emulator {
        let mut emulator = Emulator { 
            ram: [0; 4096], 
            display: [0; SCREEN_WIDTH * SCREEN_HEIGHT], 
            pc: START_ADDRESS, 
            sp: 0,
            i_reg: 0, 
            v_reg: [0; 16], 
            stack: [0; 16], 
            delay_timer: 0, 
            sound_timer: 0,
            opcode: 0 };
            
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


    // opcodes

    // clear the display - CLS
    fn op_00e0(&mut self) {
        self.display = [0; self.display.len()];
    }

    // return from subroutine - RET
    fn op_00ee(&mut self) {
        self.sp = self.sp - 1;
        self.pc = self.stack[self.sp as usize];
    }

    // jump to location nnn - JP addr
    fn op_1nnn(&mut self) {
        // zero out the first 4 bits (to remove the JP bit)
        let address = self.opcode & 0x0FFF;
        self.pc = address;
    }

}