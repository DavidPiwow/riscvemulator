use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum InstructionType {
    RInstr,
    IInstr,
    BInstr,
    SInstr,
  //  UInstr,
   // JInstr,
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
fn imm_i_f_u32(n: u32) -> i16 {
    ((((n >> 20) & 0xFFF) as i16) << 4) >> 4
}

#[inline(always)]
fn imm_b_f_u32(n: u32) -> i16 {
    (((((n>>31) & 0x1) << 11 | ((n>>7) & 0x1) << 10 |  ((n>>25)&0x3F) << 5 |  ((n >> 8) & 0xF) << 1  ) as i16) << 4) >> 4
}

#[inline(always)]
fn imm_s_f_u32(n: u32) -> i16 {
    ((((n >> 25 & 0x7F) << 5 |  (n >> 7) & 0x1F) as i16) << 4) >> 4
}


#[inline(always)]
fn imm_j_f_u32(n: u32) -> i32 {
    ((n >> 12 & 0x7f) << 12 )  as i32
}

pub struct RInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub funct3:u8,
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

pub struct IInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub imm: i16,
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

pub struct BInstruction {
    pub opcode: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: i16,
}

impl BInstruction {
    pub fn new(n: u32) -> Self {
        Self {
            opcode: opcode_f_u32(n),
            funct3: funct3_f_u32(n),
            rs1: rs1_f_u32(n),
            rs2: rs2_f_u32(n),
            imm: imm_b_f_u32(n),
        }
    }
}

impl Debug for BInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OPCODE: {:b} | RS1: R{} | RS2: R{} | IMM: {} ",
            self.opcode, self.rs1, self.rs2, self.imm
        )
    }
}


pub struct SInstruction {
    pub opcode: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub imm: i16,
}

impl SInstruction {
    pub fn new(n: u32) -> Self {
        Self {
            opcode: opcode_f_u32(n),
            funct3: funct3_f_u32(n),
            rs1: rs1_f_u32(n),
            rs2: rs2_f_u32(n),
            imm: imm_s_f_u32(n),
        }
    }
}

impl Debug for SInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OPCODE: {:b} | RS1: R{} | RS2: R{} | IMM: {} ",
            self.opcode, self.rs1, self.rs2, self.imm
        )
    }
}


pub struct JInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub imm: i32,
}

impl JInstruction {
    pub fn new(n: u32) -> Self {
        Self {
            opcode: opcode_f_u32(n),
            rd: rd_f_u32(n),
            imm: imm_j_f_u32(n),
        }
    }
}

impl Debug for JInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OPCODE: {:b} | RD: R{} | IMM: {} ",
            self.opcode, self.rd, self.imm
        )
    }
}