#[derive(Default)]
#[derive(Debug)]
pub struct CPU{
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,

    pub pc: u16,
    pub sp: u16,

    pub zero: bool,
    pub carry: bool,

    pub halted: bool,
}

impl CPU {
    pub fn inc_cp(&mut self) {
        self.pc += 1;
    }
}
