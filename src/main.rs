mod cpu;
mod instructions;
mod memory;

use cpu::CPU;
use memory::Memory;
use instructions::Instruction;

fn main() {
    let mut cpu = CPU::default();
    let mut mem = Memory::new();



    // a = 10
    mem.write(0x0000, Instruction::MOV as u8);
    mem.write(0x0001, 0);
    mem.write(0x0002, 5);

    // b = 2
    mem.write(0x0003, Instruction::MOV as u8);
    mem.write(0x0004, 1);
    mem.write(0x0005, 10);

    // a = a + b
    mem.write(0x0006, Instruction::SUB as u8);
    mem.write(0x0007, 0);
    mem.write(0x0008, 1);

    // set b = 0
    mem.write(0x0009, Instruction::MOV as u8);
    mem.write(0x000a, 1);
    mem.write(0x000b, 0);

    // halt
    mem.write(0x000c, Instruction::HLT as u8);

    while !cpu.halted {
        let opcode = mem.read(cpu.pc);
        cpu.inc_cp();

        match opcode {
            x if x == Instruction::MOV as u8 => {
                let reg = mem.read(cpu.pc); cpu.inc_cp();
                let val = mem.read(cpu.pc); cpu.inc_cp();

                match reg {
                    0 => cpu.a = val,
                    1 => cpu.b = val,
                    2 => cpu.c = val,
                    3 => cpu.d = val,
                    _ => {}
                }
            }

            x if x == Instruction::ADD as u8 => {
                let dest = mem.read(cpu.pc); cpu.pc += 1;
                let src  = mem.read(cpu.pc); cpu.pc += 1;

                let (result, carry) = match (dest, src) {
                    // What the fuck do these tuples mean?
                    // so basically they are the numbers assigned to register
                    // 0 => A, 1 => B ....
                    // so when it is (0, 0), it basically says add the
                    // value of register B into register A,
                    // thats exactly whats replicated in the code below
                    (0, 0) => cpu.a.overflowing_add(cpu.a),
                    (0, 1) => cpu.a.overflowing_add(cpu.b),
                    (0, 2) => cpu.a.overflowing_add(cpu.c),
                    (0, 3) => cpu.a.overflowing_add(cpu.d),

                    (1, 0) => cpu.b.overflowing_add(cpu.a),
                    (1, 1) => cpu.b.overflowing_add(cpu.b),
                    (1, 2) => cpu.b.overflowing_add(cpu.c),
                    (1, 3) => cpu.b.overflowing_add(cpu.d),

                    (2, 0) => cpu.c.overflowing_add(cpu.a),
                    (2, 1) => cpu.c.overflowing_add(cpu.b),
                    (2, 2) => cpu.c.overflowing_add(cpu.c),
                    (2, 3) => cpu.c.overflowing_add(cpu.d),

                    (3, 0) => cpu.d.overflowing_add(cpu.a),
                    (3, 1) => cpu.d.overflowing_add(cpu.b),
                    (3, 2) => cpu.d.overflowing_add(cpu.c),
                    (3, 3) => cpu.d.overflowing_add(cpu.d),

                    _ => (0, false),
                };



                match dest {
                    0 => cpu.a = result,
                    1 => cpu.b = result,
                    2 => cpu.c = result,
                    3 => cpu.d = result,
                    _ => {}
                }

                cpu.zero = result == 0;
                cpu.carry = carry;
            }
            x if x == Instruction::SUB as u8 => {
                let dest = mem.read(cpu.pc); cpu.pc += 1;
                let src  = mem.read(cpu.pc); cpu.pc += 1;

                let (result, borrow) = match (dest, src) {
                    // What the fuck do these tuples mean?
                    // so basically they are the numbers assigned to register
                    // 0 => A, 1 => B ....
                    // so when it is (0, 0), it basically says add the
                    // value of register B into register A,
                    // thats exactly whats replicated in the code below
                    (0, 0) => cpu.a.overflowing_sub(cpu.a),
                    (0, 1) => cpu.a.overflowing_sub(cpu.b),
                    (0, 2) => cpu.a.overflowing_sub(cpu.c),
                    (0, 3) => cpu.a.overflowing_sub(cpu.d),

                    (1, 0) => cpu.b.overflowing_sub(cpu.a),
                    (1, 1) => cpu.b.overflowing_sub(cpu.b),
                    (1, 2) => cpu.b.overflowing_sub(cpu.c),
                    (1, 3) => cpu.b.overflowing_sub(cpu.d),

                    (2, 0) => cpu.c.overflowing_sub(cpu.a),
                    (2, 1) => cpu.c.overflowing_sub(cpu.b),
                    (2, 2) => cpu.c.overflowing_sub(cpu.c),
                    (2, 3) => cpu.c.overflowing_sub(cpu.d),

                    (3, 0) => cpu.d.overflowing_sub(cpu.a),
                    (3, 1) => cpu.d.overflowing_sub(cpu.b),
                    (3, 2) => cpu.d.overflowing_sub(cpu.c),
                    (3, 3) => cpu.d.overflowing_sub(cpu.d),

                    _ => (0, false),
                };



                match dest {
                    0 => cpu.a = result,
                    1 => cpu.b = result,
                    2 => cpu.c = result,
                    3 => cpu.d = result,
                    _ => {}
                }

                cpu.zero = result == 0;
                cpu.carry = borrow;
            }


            x if x == Instruction::JMP as u8 => {}

            x if x == Instruction::JZ  as u8 => {}

            x if x == Instruction::HLT as u8 => {
                cpu.halted = true;
            }

            _ => panic!("Unknown opcode {:02X}", opcode),
        }
    }

    println!("{:#?}", cpu);

}
