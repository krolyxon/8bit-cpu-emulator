#[repr(u8)]
pub enum Instruction {
    MOV = 0x01,
    ADD = 0x02,
    SUB = 0x03,
    JMP = 0x04,
    JZ = 0x05,
    JNZ = 0x06,
    HLT = 0xFF,
}

impl Instruction {
    pub  fn opcode_name(op: u8) -> &'static str{
        match op {
            0x01 => "MOV",
            0x02 => "ADD",
            0x03 => "SUB",
            0x04 => "JMP",
            0x05 => "JZ",
            0x06 => "JNZ",
            0xFF => "HLT",
            _ => "???",
        }
    }
}
