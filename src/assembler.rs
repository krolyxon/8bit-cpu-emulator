use crate::instructions::Instruction;
use std::collections::HashMap;

fn tokenize(line: &str) -> Vec<String> {
    line.split(|c| c == ' ' || c == ',' || c == '\t')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

fn parse_reg(s: &str) -> u8 {
    match s {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Unknown register {}", s),
    }
}

fn instr_size(tokens: &[String]) ->  u16 {
    match tokens[0].as_str() {
        "mov" | "add" | "sub" | "jmp" | "jz" | "jnz" => 3,
        "hlt" => 1,
        _ => panic!("Unknown instruction {}", tokens[0])

    }
}


fn first_pass(source: &str) -> HashMap<String, u16> {
    let mut symbols = HashMap::new();
    let mut addr: u16 = 0;

    for line in source.lines() {
        let line = line.trim();

        // Ignoring comments and empty lines
        if line.is_empty() || line.starts_with(";") {
            continue;
        }

        // Labels (ends with ":")
        if line.ends_with(":") {
            let label = line.trim_end_matches(":").to_string();
            symbols.insert(label, addr);
            continue;
        }

        let tokens = tokenize(line);
        addr += instr_size(&tokens);
    }

    symbols
}

pub fn assembler(source: &str) -> Vec<u8> {
    let symbols = first_pass(source);
    let mut bytes = Vec::new();

    for (line_no, line) in source.lines().enumerate() {
        let line = line.trim();

        // Comments in assembly start with ";"
        if line.is_empty() || line.starts_with(';') || line.ends_with(":") {
            continue;
        }

        let tokens = tokenize(line);

        match tokens[0].as_str() {
            "mov" => {
                // mov reg, imm
                let reg = parse_reg(&tokens[1]);
                let imm: u8 = tokens[2].parse().unwrap();

                bytes.push(Instruction::MOV as u8);
                bytes.push(reg);
                bytes.push(imm);
            }

            "add" => {
                // add a, b
                let r1 = parse_reg(&tokens[1]);
                let r2 = parse_reg(&tokens[2]);

                bytes.push(Instruction::ADD as u8);
                bytes.push(r1);
                bytes.push(r2);
            }

            "sub" => {
                // sub a, b
                let r1 = parse_reg(&tokens[1]);
                let r2 = parse_reg(&tokens[2]);

                bytes.push(Instruction::SUB as u8);
                bytes.push(r1);
                bytes.push(r2);
            }

            "jmp" | "jz" | "jnz" => {
                let opcode = match tokens[0].as_str() {
                    "jmp" => Instruction::JMP,
                    "jz" => Instruction::JZ,
                    "jnz" => Instruction::JNZ,
                    _ => unreachable!()
                };

                let label = &tokens[1];
                let addr = *symbols.get(label).expect("Uknown label");

                bytes.push(opcode as u8);
                bytes.push((addr & 0xFF) as u8); // low
                bytes.push((addr >> 8) as u8); // high
            }

            "hlt" => {
                bytes.push(Instruction::HLT as u8);
            }

            _ => panic!("Line {}: unknown instruction", line_no + 1),
        }
    }

    bytes
}
