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
    sp: u8,
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
        // zero out the first nibble (to remove the JP bit)
        // 0x0FFF = 0000 1111 1111 1111
        let address = self.opcode & 0x0FFF;
        self.pc = address;
    }

    // call subroutine at nnn - CALL addr
    fn op_2nnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        // store pc on top of stack
        self.stack[self.sp as usize] = self.pc;
        self.sp = self.sp + 1;
        // set pc to opcode
        self.pc = address;
    }

    // Skip next instruction if Vx (8 bit register) == kk - SE Vx, byte
    fn op_3xkk(&mut self) {
        // get Vx (second nibble)
        // 0x0F00 = 0000 1111 0000 0000
        let Vx = (self.opcode & 0x0F00) >> 8;
         let byte = self.opcode & 0x00FF;
        if u16::from(self.v_reg[Vx as usize]) == byte {
            self.pc = self.pc + 2
        }
    }

    // Skip next instruction if Vx != kk - SNE Bx, byte
    fn op_4xkk(&mut self) {
        let Vx = (self.opcode & 0x0F00) >> 8;
        let byte = self.opcode & 0x00FF;
        if u16::from(self.v_reg[Vx as usize]) != byte {
            self.pc = self.pc + 2
        }
    }

    // Skip next instruction if Vx == Vy - SE Vx, Vy
    fn op_5xy0(&mut self) {
        let Vx = (self.opcode & 0x0F00) >> 8;
        let Vy = (self.opcode & 0x00F0) >> 4;
        if self.v_reg[Vx as usize] == self.v_reg[Vy as usize] {
            self.pc = self.pc + 2;
        }
    }

    // The interpreter puts value kk into register Vx - LD Vx, byte
    fn op_6xkk(&mut self) {
        let Vx = (self.opcode & 0x0F00) >> 8;
        let byte = self.opcode & 0x00FF;
        self.v_reg[Vx as usize] = byte as u8;
    }

    // Adds the value kk to the value of register Vx, then stores the result in Vx - ADD Vx, byte
    fn op_7xkk(&mut self) {
        let Vx = ()
    }


}