pub const RAM_SIZE: usize = 4 * 1024;

pub struct Ram {
    ram: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        Ram { ram: [0; RAM_SIZE] }
    }

    pub fn read_byte(&self, index: usize) -> u8 {
        self.ram[index]
    }

    pub fn write_byte(&mut self, index: usize, data: u8) {
        self.ram[index] = data;
    }

    pub fn read_word(&self, index: usize) -> u32 {
            (self.read_byte(index) as u32) << 24 |
            (self.read_byte(index + 1) as u32) << 16 |
            (self.read_byte(index + 2) as u32) << 8 |
            self.read_byte(index + 3) as u32
    }

    pub fn write_word(&mut self, index: usize, data: u32) {
        for i in 0..4 {
            self.write_byte(index + i, (data >> (8 * (3 - i))  & 0xF) as u8);
        }
    }
}
