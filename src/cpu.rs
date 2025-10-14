use std::fmt::Display;
use crate::instruction::{BInstruction, IInstruction, InstructionType, RInstruction, SInstruction};

// idk why i picked this number but i liked it 
const MEM_START: usize = 0x100;
const MEM_SIZE: u32 = 0x200;

#[derive(Debug)]
pub struct InstructionInfo {
    pub instr_type: Option<InstructionType>,
    pub name: Option<String>,
    pub rd: Option<u8>,
    funct3: u8,
    pub rs1: u8,
    pub rs2: Option<u8>,
    funct7: Option<u8>,
    pub imm: Option<i16>
}

impl Default for InstructionInfo {
    fn default() -> InstructionInfo {
        InstructionInfo {
            instr_type: None,
            name: None,
            rd: None,
            funct3: 0,
            rs1: 0,
            rs2: None,
            funct7: None,
            imm: None,
        }
    }
}

pub struct CPU {
    registers: [u32; 32],
    memory: [u8; MEM_SIZE as usize],
    pc: u32,
    break_flag: bool,
    instruction_info: InstructionInfo,
}

impl CPU {
    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn view_instr_info(&self) -> &InstructionInfo {
        &self.instruction_info
    }
    pub fn view_registers(&self) -> &[u32; 32] {
        &self.registers
    }
    pub fn view_memory(&self) -> &[u8; MEM_SIZE as usize] {&self.memory}

    pub fn new() -> CPU {
        CPU::default()
    }

    pub fn reset(&mut self) {
        self.instruction_info = InstructionInfo::default();
        self.registers = [0; 32];
        self.memory = [0; MEM_SIZE as usize];
        self.pc = MEM_START as u32;
        self.break_flag = false;
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
        let mut result = self.step();
        while result {
            result = self.step();
        }
    }

    pub fn step(&mut self) -> bool {
        if self.break_flag || self.pc > (MEM_SIZE - 0x4) {
            return false
        }
        self.instruction_info = InstructionInfo::default();
        let instr: u32 = self.fetch();
        self.decode(instr);
        self.advance();

        true
    }

    fn fetch(&self) -> u32 {
        self.memory[self.pc as usize] as u32
            | (self.memory[self.pc as usize + 1] as u32) << 8
            | (self.memory[self.pc as usize + 2] as u32) << 16
            | (self.memory[self.pc as usize + 3] as u32) << 24
    }

    fn get_word(&self, base: u32, offset: i32) -> u32 {
        let start = (base as i32 + offset) as usize;
        println!("START: {}", start);
        self.memory[start] as u32
            | (self.memory[start + 1] as u32) << 8
            | (self.memory[start + 2] as u32) << 16
            | (self.memory[start + 3] as u32) << 24
    }

    fn set_word(&mut self, word: u32, base: u32, offset: i32) {
        let start = (base as i32 + offset) as usize;

        self.memory[start] = (word & 0xFF) as u8 ;
        self.memory[start+ 1] = (word >> 8 & 0xFF) as u8 ;
        self.memory[start+ 2] = (word >> 16 & 0xFF) as u8 ;
        self.memory[start + 3] = (word >> 24 & 0xFF) as u8 ;
    }

    fn decode(&mut self, instruction: u32) {
        match (instruction & 0x7F) as u8 {
            0x33 => self.decode_r(instruction),
            0x13 | 0x73 | 0x3 => self.decode_i(instruction),
            0x63 => self.decode_b(instruction),
            0x23 => self.decode_s(instruction),
            _ => (),
        }
    }

    fn decode_r(&mut self, instruction: u32) {
        let ins = RInstruction::new(instruction);

        if ins.opcode != 0x33 {
            return;
        }

        if ins.funct7 == 0x01  {
            // multiply extension
            return
        }

        self.instruction_info.instr_type = Some(InstructionType::RInstr);
        self.instruction_info.rs1 = ins.rs1;
        self.instruction_info.rs2 = Some(ins.rs2);
        self.instruction_info.rd = Some(ins.rd);

        match ins.funct3 {
            0x0 => {
                if ins.funct7 == 0x00 {
                    self.instruction_info.name = Some("Add".to_string());
                    self.add(ins.rd, ins.rs1, ins.rs2);
                } else if ins.funct7 == 0x20 {
                    self.instruction_info.name = Some("Sub".to_string());
                    self.sub(ins.rd, ins.rs1, ins.rs2);
                }
            },
            0x4 => {
                self.instruction_info.name = Some("Xor".to_string());
                self.xor(ins.rd, ins.rs1, ins.rs2);
            },
            0x6 => {
                self.instruction_info.name = Some("Or".to_string());
                self.or(ins.rd, ins.rs1, ins.rs2);
            },
            0x7 => {
                self.instruction_info.name = Some("And".to_string());
                self.and(ins.rd, ins.rs1, ins.rs2);
            },
            0x1 => {
                self.instruction_info.name = Some("Shift Left Logical".to_string());
                self.shift_left_logical(ins.rd, ins.rs1, ins.rs2);
            },
            0x5 => {
                if ins.funct7 == 0x00 {
                    self.instruction_info.name = Some("Shift Right Logical".to_string());
                    self.shift_right_logical(ins.rd, ins.rs1, ins.rs2);
                } else if ins.funct7 == 0x20 {
                    self.instruction_info.name = Some("Shift Right Arithmetic".to_string());
                    self.shift_right_arithmetic(ins.rd, ins.rs1, ins.rs2);
                }
            }
            _ => {
                self.instruction_info.name = Some("UNKNOWN R INSTRUCTION".to_string());
                println!(
                    "R {:b}\nRD: {:x} R1: {:x} R2: {:x} 3: {:x} 7: {:x} OPCODE: {:x}",
                    instruction, ins.rd, ins.rs1, ins.rs2, ins.funct3, ins.funct7, ins.opcode
                );
            }
        }
    }

    fn decode_i(&mut self, instruction: u32) {
        let ins = IInstruction::new(instruction);
        if ins.opcode == 0x73 && ins.imm == 1 {
            self.break_flag = true;
            return;
        }

        if (ins.opcode != 0x13) && (ins.opcode != 0x3) {
            return;
        }

        self.instruction_info.instr_type = Some(InstructionType::IInstr);
        self.instruction_info.rs1 = ins.rs1;
        self.instruction_info.rd = Some(ins.rd);
        self.instruction_info.imm = Some(ins.imm);

        if ins.opcode == 0x3 {
            match ins.funct3 {
                0x2 => {
                    self.instruction_info.name = Some("LW".to_string());
                    self.load_word(ins.rd, ins.rs1, ins.imm as i32);
                }
                _ => {
                    self.instruction_info.name = Some("Unknown I".to_string());
                }
            }
        } else {
            match ins.funct3 {
                0x0 => {
                    self.instruction_info.name = Some("AddI".to_string());
                    self.addimm(ins.rd, ins.rs1, ins.imm as i32);
                },
                0x4 => {
                    self.instruction_info.name = Some("XorI".to_string());
                    self.xorimm(ins.rd, ins.rs1, ins.imm as i32);
                },
                0x6 => {
                    self.instruction_info.name = Some("OrI".to_string());
                    self.orimm(ins.rd, ins.rs1, ins.imm as i32);
                },
                0x7 => {
                    self.instruction_info.name = Some("AndI".to_string());
                    self.andimm(ins.rd, ins.rs1, ins.imm as i32);
                },
                _ => {
                    self.instruction_info.name = Some("Unknown I".to_string());
                }
            }
        }


    }

    fn decode_b(&mut self, instruction: u32) {
        let ins = BInstruction::new(instruction);

        if ins.opcode != 0x63 {
            return;
        }

        self.instruction_info.instr_type = Some(InstructionType::BInstr);
        self.instruction_info.rs1 = ins.rs1;
        self.instruction_info.rs2 = Some(ins.rs2);
        self.instruction_info.imm = Some(ins.imm);

        match ins.funct3 {
            0x0 => {
                self.instruction_info.name = Some("BEQ".to_string());
                self.brancheq(ins.rs1, ins.rs2, ins.imm as i32)
            },
            0x1 => {
                self.instruction_info.name = Some("BNE".to_string());
                self.branchneq(ins.rs1, ins.rs2, ins.imm as i32)
            },
            0x4 => {
                self.instruction_info.name = Some("BLT".to_string());
                self.branchlt(ins.rs1, ins.rs2, ins.imm as i32)
            },
            0x5 => {
                self.instruction_info.name = Some("BGE".to_string());
                self.branchge(ins.rs1, ins.rs2, ins.imm as i32)
            },
            _ => println!("UNKNOWN B {:b}", instruction),
        }
    }

    fn decode_s(&mut self, instruction: u32) {
        let ins = SInstruction::new(instruction);

        if ins.opcode != 0x23 {
            return;
        }

        self.instruction_info.instr_type = Some(InstructionType::SInstr);
        self.instruction_info.rs1 = ins.rs1;
        self.instruction_info.rs2 = Some(ins.rs2);
        self.instruction_info.imm = Some(ins.imm);

        match ins.funct3 {
            0x2 => {
                self.instruction_info.name = Some("SW".to_string());
                self.store_word(ins.rs1, ins.rs2, ins.imm as i32);
            }
            _ => {
                self.instruction_info.name = Some("Unknown S".to_string());
            }
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
        self.registers[rd as usize] = (self.registers[r1 as usize] as i32 - self.registers[r2 as usize] as i32) as u32;
    }

    #[inline(always)]
    fn xor(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = self.registers[r1 as usize] ^ self.registers[r2 as usize];
    }

    #[inline(always)]
    fn or(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = self.registers[r1 as usize] | self.registers[r2 as usize];
    }

    #[inline(always)]
    fn and(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = self.registers[r1 as usize] & self.registers[r2 as usize];
    }

    #[inline(always)]
    fn shift_left_logical(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = (self.registers[r1 as usize] ) << self.registers[r2 as usize];
    }

    #[inline(always)]
    fn shift_right_logical(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = (self.registers[r1 as usize]) >> self.registers[r2 as usize];
    }

    // keeps sign
    #[inline(always)]
    fn shift_right_arithmetic(&mut self, rd: u8, r1: u8, r2: u8) {
        self.registers[rd as usize] = self.registers[r1 as usize]  >> self.registers[r2 as usize];
    }

    // rd = r1 + imm (u32?)
    #[inline(always)]
    fn addimm(&mut self, rd: u8, r1: u8, imm: i32) {
        self.registers[rd as usize] = (self.registers[r1 as usize] as i32 + imm) as u32;
    }

    #[inline(always)]
    fn xorimm(&mut self, rd: u8, r1: u8, imm: i32) {
        self.registers[rd as usize] = (self.registers[r1 as usize] as i32 ^ imm) as u32;
    }

    #[inline(always)]
    fn orimm(&mut self, rd: u8, r1: u8, imm: i32) {
        self.registers[rd as usize] = (self.registers[r1 as usize] as i32| imm) as u32;
    }

    #[inline(always)]
    fn andimm(&mut self, rd: u8, r1: u8, imm: i32) {
        self.registers[rd as usize] = (self.registers[r1 as usize] as i32 & imm) as u32;
    }

    // BRANCHING
    #[inline(always)]
    fn branch(&mut self, imm: i32) {
        if self.pc as i32 + imm - 4  >= 0 {
            self.pc = (self.pc as i32 + imm  - 4) as u32;
        }
    }

    #[inline(always)]
    fn brancheq(&mut self, r1: u8, r2: u8, imm: i32) {
        if imm % 4 != 0 {
            return;
        }
        if self.registers[r1 as usize] == self.registers[r2 as usize] {
            self.branch(imm);
        }
    }

    #[inline(always)]
    fn branchneq(&mut self, r1: u8, r2: u8, imm: i32) {
        if imm % 4 != 0 {
            return;
        }
        if self.registers[r1 as usize] != self.registers[r2 as usize] {
            self.branch(imm);
        }
    }

    #[inline(always)]
    fn branchlt(&mut self, r1: u8, r2: u8, imm: i32) {
        if imm % 4 != 0 {
            return;
        }
        if self.registers[r1 as usize] < self.registers[r2 as usize] {
            self.branch(imm);
        }
    }

    #[inline(always)]
    fn branchge(&mut self, r1: u8, r2: u8, imm: i32) {
        if imm % 4 != 0 {
            return;
        }
        if self.registers[r1 as usize] >= self.registers[r2 as usize] {
            self.branch(imm);
        }
    }
        // MEMORY

    fn load_word(&mut self, rd: u8, r1: u8, imm: i32) {
        let word = self.get_word(self.registers[r1 as usize], imm);
        println!("loading word {} from {} + {}", word, r1, imm);
        self.registers[rd as usize] = word;
    }

    fn store_word(&mut self, r1: u8, r2: u8, imm: i32) {
        let word = self.registers[r2 as usize];
        self.set_word(word, self.registers[r1 as usize], imm);

    }

}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            registers: [0; 32],
            memory: [0; MEM_SIZE as usize],
            pc: 0,
            break_flag: false,
            instruction_info: InstructionInfo::default(),
        }
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PC: {}\nREGISTERS:{:?}", self.pc, self.registers)
    }
}
