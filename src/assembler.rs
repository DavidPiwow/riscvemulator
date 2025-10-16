use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::instruction::InstructionType;
use crate::instruction::InstructionType::{BInstr, IInstr, RInstr, SInstr};

const EBREAK: u32 = 0b00000000000100000000000001110011;

// store type, opcode, funct3, funct7/imm for some
fn get_info() -> HashMap<&'static str, (InstructionType, u8,u8,u8)> {
    HashMap::from([
        ("add",(RInstr, 0b0110011, 0x0, 0x00)),
        ("sub",(RInstr, 0b0110011, 0x0, 0x20)),
        ("xor",(RInstr, 0b0110011, 0x4, 0x00)),
        ("or",(RInstr, 0b0110011, 0x6, 0x00)),
        ("and",(RInstr, 0b0110011, 0x7, 0x00)),
        ("sll",(RInstr, 0b0110011, 0x1, 0x00)),
        ("srl",(RInstr, 0b0110011, 0x5, 0x00)),
        ("sra",(RInstr, 0b0110011, 0x5, 0x20)),

        ("addi",(IInstr, 0b0010011, 0x0, 0x00)),
        ("xori",(IInstr, 0b0010011, 0x4, 0x00)),
        ("ori",(IInstr, 0b0010011, 0x6, 0x00)),
        ("andi",(IInstr, 0b0010011, 0x7, 0x00)),

        ("beq",(BInstr, 0b1100011, 0x0, 0x00)),
        ("bne",(BInstr, 0b1100011, 0x1, 0x00)),
        ("blt",(BInstr, 0b1100011, 0x4, 0x00)),
        ("bge",(BInstr, 0b1100011, 0x5, 0x00)),

        ("sw",(SInstr, 0b0100011, 0x2, 0x00)),
        ("lw", (IInstr, 0b0000011, 0x2, 0x00)),

        ("ebreak",(IInstr, 0b1110011, 0x00, 0x1)),
    ])
}

pub struct Assembler {
    program: Vec<String>,
    instructions: HashMap<&'static str, (InstructionType, u8,u8,u8)>,
    labels: HashMap<String, usize>,
}

impl Assembler {
    pub fn view_program(&self) -> &Vec<String> {
        &self.program
    }

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
        let mut ins_count: usize = 0;
        let mut labels: HashMap<String, usize> = HashMap::new();

        let lines: Vec<String> = str.lines().filter_map(|s|
            if s.is_empty() {
                None
            } else {
                if s.contains("#") {
                    ins_count += 1;
                    return Some(s.split("#").nth(0).unwrap().to_string());
                } else if s.contains(":") {
                    let name = s.split(":").nth(0).unwrap();
                    labels.insert(name.to_string(), ins_count);
                    return None;
                }
                ins_count += 1;
                Some(s.to_string())

            }
        ).collect();

        Assembler {
            program: lines,
            instructions: get_info(),
            labels,
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
        let mut index: usize = 0;
        for instruction in &self.program {
            if instruction.contains("ebreak") {
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

            let bin = match &val.0 {
                RInstr => {
                    self.info_to_r(val, &self.extract_vals(instruction))
                },
                IInstr => {
                    self.info_to_i(val, &self.extract_vals_i(instruction, index))
                },
                SInstr => {
                    self.info_to_s(val, &self.extract_vals_i(instruction, index))
                },
                BInstr => {
                    self.info_to_b(val, &self.extract_vals_i(instruction, index))
                },
                //    InstructionType::UInstr => 0,
                //    InstructionType::JInstr => 0,
            };
            bins.push(bin);
            index += 1;
        }
        bins
    }

    // returns rd, r1, r2
    fn extract_vals(&self, str: &String) -> (u8, u8, u8) {
        let parts = str.split_ascii_whitespace().skip(1).filter_map(|s|  {
            s.replace('x',"").replace(',',"").parse::<u8>().ok()
        }).collect::<Vec<u8>>();
        if parts.len() != 3 {
            panic!("Malformed r instruction {}", str);
        }

        if parts[0] >= 32 || parts[1] >= 32 || parts[2] >= 32 {
            panic!("Malformed r instruction {}: invalid register(s) || {:?}", str, parts);
        }
        (parts[0], parts[1], parts[2])
    }

    /*
// rd, rd1, imm for i
// r1, r2, imm for b
     */

    fn extract_vals_i(&self, str: &String, index: usize) -> (u8, u8, i16) {
        let mut parts = str.replace('('," ").replace(')',"").split_ascii_whitespace().skip(1).filter_map(|s|  {
            let val = s.replace('x',"").replace(',',"");
            if self.labels.contains_key(&val) {
                return Some((self.labels.get(&val).unwrap().clone() as i16 - index as i16) * 4)
            }
            val.parse::<i16>().ok()
        }).collect::<Vec<i16>>();

        if str.contains("(") {
            let t = parts[1];
            parts[1]=parts[2];
            parts[2]=t;
        }
        if parts[0] >= 32 || parts[0] < 0 || parts[1] >= 32 || parts[1] < 0 {
            panic!("Malformed i instruction {}: invalid register(s) || {:?}", str, parts);
        }
        if parts[2] < -2048 || parts[2] > 2047 {
            panic!("{} is not a valid imm value", parts[2]);
        }

        (parts[0] as u8, parts[1] as u8, parts[2])
    }

    fn info_to_r(&self, info: &(InstructionType, u8, u8, u8), registers: &(u8, u8, u8)) -> u32 {
        let mut binary = (info.1 & 0x7F) as u32; // opcode
        binary |= ((registers.0 & 0x1F) as u32) << 7; // rd
        binary |= ((info.2 & 0x7) as u32) << 12; // funct3
        binary |= ((registers.1 & 0x1F) as u32) << 15; // rs1
        binary |= ((registers.2 & 0x1F) as u32) << 20; //rs2
        binary |= ((info.3 & 0x7F) as u32) << 25; // funct7

        binary
    }

    fn info_to_i(&self, info: &(InstructionType, u8, u8, u8), data: &(u8, u8, i16)) -> u32 {
        let mut binary = (info.1 & 0x7F) as u32; // opcode
        binary |= ((info.2 & 0x7) as u32) << 12; // funct3

        binary |= ((data.0 & 0x1F) as u32) << 7; // rd
        binary |= ((data.1 & 0x1F) as u32) << 15; // rs1
        binary |= ((data.2 & 0xFFF) as u32) << 20; // imm

        binary
    }


    fn info_to_s(&self, info: &(InstructionType, u8, u8, u8), data: &(u8, u8, i16)) -> u32 {
        let mut binary = (info.1 & 0x7F) as u32;
        let immp1 = (data.2 as u32) & 0x1F;
        let immp2 = data.2 as u32 >> 5 & 0x7F;


        binary |= immp1 << 7;

        binary |= ((info.2 & 0x7) as u32) << 12; // funct3
        binary |= ((data.1 & 0x1F) as u32) << 15; // rs1
        binary |= ((data.0 & 0x1F) as u32) << 20; //rs2

        binary |= (immp2 & 0x7F) << 25;

        binary
    }

    fn info_to_b(&self, info: &(InstructionType, u8, u8, u8), data: &(u8, u8, i16)) -> u32 {
        let mut binary = (info.1 & 0x7F) as u32; // opcode
        let immp1 = ((data.2>>1 & 0xF) << 1 | (data.2 >> 11 & 0x1)) as u8;
        let immp2 = (((data.2 >> 12 & 0x1) << 6) | ((data.2) >> 5) & 0x3F) as u8;


        binary |= (immp1 as u32) << 7;

        binary |= ((info.2 & 0x7) as u32) << 12; // funct3
        binary |= ((data.0 & 0x1F) as u32) << 15; // rs1
        binary |= ((data.1 & 0x1F) as u32) << 20; //rs2

        binary |= (immp2 as u32) << 25;


        binary
    }
}