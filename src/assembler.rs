use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use crate::instruction::InstructionType;
use crate::instruction::InstructionType::{I_INSTR, R_INSTR};

const EBREAK: u32 = 0b00000000000100000000000001110011;

// store type, opcode, funct3, funct7/imm for some
fn get_info() -> HashMap<&'static str, (InstructionType, u8,u8,u8)> {
    HashMap::from([
        ("add",(R_INSTR,0b0110011,0x0,0x00)),
        ("sub",(R_INSTR,0b0110011,0x0,0x20)),
        ("addi",(I_INSTR,0b0010011,0x0,0x00)),
        ("ebreak",(I_INSTR,0b1110011,0x00,0x1)),
    ])
}


pub struct Assembler {
    program: Vec<String>,
    instructions: HashMap<&'static str, (InstructionType, u8,u8,u8)>,
}


impl Assembler {
    pub fn open_file(filename: &str) -> Assembler {
        let f = match File::open(filename) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut reader = BufReader::new(f);
        let mut str = String::new();

        if reader.read_to_string(&mut str).is_err() {
            panic!("Failed to read from file");
        };

        let lines: Vec<String> = str.lines().map(|s| s.to_string()).collect();

        Assembler {
            program: lines,
            instructions: get_info(),
        }
    }

    // instructions should be
    /*
    r -> NAME rd, rd1, rd2
    i -> NAME rd, rd1, imm

    like really i could just make an instruction struct directly
    but thats BORING <3
     */
    pub fn assemble(&self) -> Vec<u32> {
        let mut bins: Vec<u32> = vec![];
        for instruction in &self.program {
            println!("{:?}", instruction);
            if instruction == "ebreak" {
                bins.push(EBREAK);
                break;
            }

            let name = instruction.split_ascii_whitespace().next();
            if name.is_none() {
                break;
            }
            let val = match self.instructions.get(name.unwrap()) {
                Some(val) => val,
                None => panic!("Instruction {} not found", name.unwrap()),
            };
            let bin = match val.0 {
                R_INSTR => {
                    self.info_to_r(val, &r_to_values(instruction))
                },
                I_INSTR => {
                    self.info_to_i(val, &i_to_values(instruction))
                },
                InstructionType::S_INSTR => 0,
                InstructionType::B_INSTR => 0,
                InstructionType::U_INSTR => 0,
                InstructionType::J_INSTR => 0,
            };
            println!("{:#032b}",bin);
            bins.push(bin);

        }


        bins
    }

    fn info_to_r(&self, info: &(InstructionType, u8, u8, u8), registers: &(u8,u8,u8)) -> u32 {
        let mut binary = (info.1 & 0x7F) as u32; // opcode
        binary |= ((registers.0 & 0x1F) as u32) << 7; // rd
        binary |= ((info.2 & 0x7) as u32) << 12; // funct3
        binary |= ((registers.1 & 0x1F) as u32) << 15; // rs1
        binary |= ((registers.2 & 0x1F) as u32) << 20; //rs2
        binary |= ((info.3 & 0x7F) as u32) << 25; // funct7

        binary
    }

    fn info_to_i(&self, info: &(InstructionType, u8, u8, u8), data: &(u8,u8,i16)) -> u32 {
        let mut binary = (info.1 & 0x7F) as u32; // opcode
        binary |= ((info.2 & 0x7) as u32) << 12; // funct3

        binary |= ((data.0 & 0x1F) as u32) << 7; // rd
        binary |= ((data.1 & 0x1F) as u32) << 15; // rs1
        binary |= ((data.2 & 0x7FF) as u32) << 20; // imm

        binary
    }
}

// rd, rs1, rs2
fn r_to_values(str: &str) -> (u8,u8,u8) {
    let parts = str.split_ascii_whitespace().skip(1).filter_map(|s|  {
        s.replace('r',"").replace(',',"").parse::<u8>().ok()
    }).collect::<Vec<u8>>();
    if parts.len() != 3 {
        panic!("Malformed r instruction {}", str);
    }

    if (parts[0] > 32 || parts[1] > 32 || parts[2] > 32) {
        panic!("Malformed r instruction {}: invalid register(s) || {:?}", str, parts);
    }
    (parts[0], parts[1], parts[2])
}


// rd, rd1, imm
fn i_to_values(str: &String) -> (u8,u8,i16) {
    let parts = str.split_ascii_whitespace().skip(1).filter_map(|s|  {
        s.replace('r',"").replace(',',"").replace('#',"").parse::<i16>().ok()
    }).collect::<Vec<i16>>();

    if parts.len() != 3 {
        panic!("Malformed i instruction {}", str);
    }

    if parts[0] > 32 || parts[0] < 0 || parts[1] > 32 || parts[1] < 0 {
        panic!("Malformed i instruction {}: invalid register(s) || {:?}", str, parts);
    }
    if parts[2] < -1024 || parts[2] > 1023 {
        panic!("{} is not a valid imm value", parts[2]);
    }

    (parts[0] as u8, parts[1] as u8, parts[2])
}

