use std::fmt::{Debug, Formatter};

pub enum InstructionType {
    R_INSTR,
    I_INSTR,
    S_INSTR,
    B_INSTR,
    U_INSTR,
    J_INSTR,
}

#[inline(always)]
fn opcode_f_u32(n: u32) -> u8 {
    (n & 0x7F) as u8
}

#[inline(always)]
fn rd_f_u32(n: u32) -> u8 {
    ((n >> 7) & 0x1F) as u8
}

#[inline(always)]
fn funct3_f_u32(n: u32) -> u8 {
    ((n >> 12) & 0x7) as u8
}

#[inline(always)]
fn rs1_f_u32(n: u32) -> u8 {
    ((n >> 15) & 0x1F) as u8
}

#[inline(always)]
fn rs2_f_u32(n: u32) -> u8 {
    ((n >> 20) & 0x7) as u8
}

#[inline(always)]
fn funct7_f_u32(n: u32) -> u8 {
    ((n >> 25) & 0x7F) as u8
}

#[inline(always)]
fn imm_i_f_u32(n: u32) -> u16 {
    ((n >> 20) & 0xFFF) as u16
}

pub trait Instruction {
    fn get_type(&self) -> InstructionType;
    fn get_opcode(&self) -> u8;
}

pub struct RInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}

impl RInstruction {
    pub fn new(n: u32) -> RInstruction {
        RInstruction {
            opcode: opcode_f_u32(n),
            rd: rd_f_u32(n),
            funct3: funct3_f_u32(n),
            rs1: rs1_f_u32(n),
            rs2: rs2_f_u32(n),
            funct7: funct7_f_u32(n),
        }
    }
}

impl Debug for RInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OPCODE: {:b} | RD: R{} | RS1: R{} | RS2: R{} ",
            self.opcode, self.rd, self.rs1, self.rs2
        )
    }
}

impl Instruction for RInstruction {
    fn get_type(&self) -> InstructionType {
        InstructionType::R_INSTR
    }

    fn get_opcode(&self) -> u8 {
        self.opcode
    }
}

pub struct IInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub imm: u16,
}

impl IInstruction {
    pub fn new(n: u32) -> Self {
        Self {
            opcode: opcode_f_u32(n),
            rd: rd_f_u32(n),
            funct3: funct3_f_u32(n),
            rs1: rs1_f_u32(n),
            imm: imm_i_f_u32(n),
        }
    }
}

impl Debug for IInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OPCODE: {:b} | RD: R{} | RS1: R{} | IMM: {} ",
            self.opcode, self.rd, self.rs1, self.imm
        )
    }
}

impl Instruction for IInstruction {
    fn get_type(&self) -> InstructionType {
        InstructionType::I_INSTR
    }

    fn get_opcode(&self) -> u8 {
        self.opcode
    }
}
