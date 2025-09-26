use crate::cpu::CPU;
mod cpu;
mod instruction;

fn main() {
    let prgm: Vec<u8> = vec![
        0x93, 0x00, 0x30, 0x00,  // add 3 to  r1
        0x13, 0x01, 0xC0, 0x00, // add 12 to r2 
        0x73, 0x00, 0x10, 0x00 // break
    ];
    let mut cpu = CPU::new();
    cpu.load_program(&prgm);
    cpu.run();
}
