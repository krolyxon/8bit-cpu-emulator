pub struct Memory {
    pub data: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        Self { data: [0; 65536] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}
