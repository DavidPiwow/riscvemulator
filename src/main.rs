use crate::assembler::Assembler;
use crate::cpu::CPU;
mod cpu;
mod instruction;
mod assembler;

fn main() {
    let assembler = Assembler::open_file("./programs/test1.rv");
    let instrs = assembler.assemble();
    let mut prgm: Vec<u8> = vec![];
    for instr in instrs {
        prgm.push((instr & 0xFF) as u8 );
        prgm.push(((instr & (0xFF << 8)) >> 8) as u8 );
        prgm.push(((instr & (0xFF << 16)) >> 16) as u8 );
        prgm.push(((instr & (0xFF << 24)) >> 24) as u8 );
    }
    let mut cpu = CPU::new();
    cpu.load_program(&prgm);
    cpu.run();
}
