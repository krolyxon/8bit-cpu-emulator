mod assembler;
mod cpu;
mod instructions;
mod memory;

use cpu::CPU;
use memory::Memory;

use crate::assembler::assembler;

fn main() {
    let mut cpu = CPU::default();
    let mut mem = Memory::new();

    let asm = std::fs::read_to_string("program.asm").unwrap();
    let program = assembler(&asm);

    for (i, byte) in program.iter().enumerate() {
        mem.write(i as u16, *byte);
    }

    while !cpu.halted {
        cpu.step(&mut mem);
        println!("{:?}", cpu);
    }
}
