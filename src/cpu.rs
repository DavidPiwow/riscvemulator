use std::fmt::Display;
use crate::instruction::{IInstruction, RInstruction};

// idk why i picked this number but i liked it 
const MEM_START: usize = 0x200;

pub struct CPU {
    registers: [i32; 32],
    memory: [u8; 0x5000],
    pc: u32,
    break_flag: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU::default()
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        let mut offset = 0;
        for word in program.iter() {
            self.memory[MEM_START + offset] = *word;
            offset += 1;
        }
        self.pc = MEM_START as u32;
    }
    pub fn run(&mut self) {
        // idk if the break flag is real but it works for now
        while self.pc <= (0x5000 - 0x4) && !self.break_flag {
            let instr: u32 = self.fetch();
            self.decode(instr);
            println!("{}\n", self);
            self.advance();
        }
    }

    fn fetch(&mut self) -> u32 {
        self.memory[self.pc as usize] as u32
            | (self.memory[self.pc as usize + 1] as u32) << 8
            | (self.memory[self.pc as usize + 2] as u32) << 16
            | (self.memory[self.pc as usize + 3] as u32) << 24
    }

    fn decode(&mut self, instruction: u32) {
        match (instruction & 0x7F) as u8 {
            0x33 => self.decode_r(instruction),
            0x13 => self.decode_i(instruction),
            0x73 => self.decode_i(instruction),
            _ => (),
        }
    }

    // ill make this better next time
    fn decode_r(&mut self, instruction: u32) {
        let ins = RInstruction::new(instruction);

        match ins.opcode {
            0x33 => {
                if ins.funct3 == 0x0 {
                    if ins.funct7 == 0x0 {
                        println!("ADD");
                        self.add(ins.rd, ins.rs1, ins.rs2);
                    } else if ins.funct7 == 0x20 {
                        println!("SUB");
                        self.sub(ins.rd, ins.rs1, ins.rs2);
                    } else {
                        println!("!!{:x}", ins.funct7);
                    }
                } else if ins.funct3 == 0x7 {
                    println!("AND {:b}", instruction);
                    self.and(ins.rd, ins.rs1, ins.rs2);
                } else if ins.funct7 == 0x6 {
                    println!("OR {:b}", instruction);
                    self.or(ins.rd, ins.rs1, ins.rs2);
                }
            }
            _ => println!(
                "UNKNOWN R {:b}\nRD: {:x} R1: {:x} R2: {:x} 3: {:x} 7: {:x} OPCODE: {:x}",
                instruction, ins.rd, ins.rs1, ins.rs2, ins.funct3, ins.funct7, ins.opcode
            ),
        }
    }

    fn decode_i(&mut self, instruction: u32) {
        let ins = IInstruction::new(instruction);

        match ins.opcode {
            0x13 => {
                if ins.funct3 == 0x0 {
                    println!("ADDI");
                    self.addimm(ins.rd, ins.rs1, ins.imm as i32);
                }
            }
            0x73 => {
                if ins.imm == 1 {
                    println!("BREAK");
                    self.break_flag = true;
                }
            }
            _ => println!("UNKNOWN I {:b}", instruction),
        }
    }

    fn advance(&mut self) {
        self.pc += 4;
    }

    // ARITHMETIC

    // rd = r1 + r2
    #[inline(always)]
    fn add(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = self.registers[r1 as usize] + self.registers[r2 as usize];
    }

    // rd = r1 - r2
    #[inline(always)]
    fn sub(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = self.registers[r1 as usize] - self.registers[r2 as usize];
    }

    #[inline(always)]
    fn and(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = self.registers[r1 as usize] & self.registers[r2 as usize];
    }

    #[inline(always)]
    fn or(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = self.registers[r1 as usize] | self.registers[r2 as usize];
    }

    // rd = r1 + imm (u32?)
    #[inline(always)]
    fn addimm(&mut self, rd: u8, r1: u8, imm: i32) {
        self.registers[rd as usize] = self.registers[r1 as usize] + imm;
    }

    // PROGRAM CONTROL
    #[inline(always)]
    fn brancheq(&mut self, r1: usize, r2: usize, imm: usize) {
        if (self.registers[r1]) == self.registers[r2] {
            self.pc += imm as u32;
        }
    }

    /*
        // MEMORY
        #[inline(always)]
        fn loadwrd(&mut self, rd: usize, r1: usize, offset: u32) {
            self.registers[rd] = self.memory[(self.registers[r1] + offset) as usize];
        }

        #[inline(always)]
        fn storewrd(&mut self, r1: usize, r2: usize, offset: u32) {
            self.memory[(self.registers[r1] + offset) as usize] = self.registers[r2];
        }
    */
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            registers: [0; 32],
            memory: [0; 0x5000],
            pc: 0,
            break_flag: false,
        }
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PC: {}\nREGISTERS:{:?}", self.pc, self.registers)
    }
}
