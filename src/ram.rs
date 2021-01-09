pub const RAM_SIZE: usize = 1024 * 64;

pub struct Ram {
    ram: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new(rom: Vec<u8>) -> Ram {
        let mut ram = Ram { ram: [0; RAM_SIZE] };
        let mut i: usize = 0;
        for instruction in rom {
            ram.ram[i] = instruction;
            i += 1;
        }
        return ram;
    }

    pub fn get_byte(&self, index: usize) -> u8 {
        return self.ram[index];
    }

    pub fn get_word(&self, index: usize) -> [u8; 4] {
        return [
            self.get_byte(index),
            self.get_byte(index + 1),
            self.get_byte(index + 2),
            self.get_byte(index + 3),
        ];
    }
}
