use crate::ram::Ram;

pub struct Bus {
    ram: Ram,
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Bus {
        Bus { ram: Ram::new(rom) }
    }

    pub fn fetch_from_ram(&self, index: usize) -> [u8; 4] {
        return self.ram.get_word(index);
    }
}
