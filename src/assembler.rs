use crate::instructions::Instruction;

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

pub fn assembler(source: &str) -> Vec<u8> {
    let mut bytes = Vec::new();

    for (line_no, line) in source.lines().enumerate() {
        let line = line.trim();

        // Comments in assembly start with ";"
        if line.is_empty() || line.starts_with(';') {
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

            "hlt" => {
                bytes.push(Instruction::HLT as u8);
            }

            _ => panic!("Line {}: unknown instruction", line_no + 1),
        }
    }

    bytes
}
