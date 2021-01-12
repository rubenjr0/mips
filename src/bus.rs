use crate::ram::Ram;

pub struct Bus {
    ram: Ram,
}

impl Bus {
    pub fn new() -> Bus {
        Bus { ram: Ram::new() }
    }

    pub fn read_byte(&self, index: usize) -> u8 {
        self.ram.read_byte(index)
    }

    pub fn read_word(&self, index: usize) -> u32 {
        self.ram.read_word(index)
    }

    pub fn write_word(&mut self, index: usize, data: u32) {
        self.ram.write_word(index, data);
    }
}
