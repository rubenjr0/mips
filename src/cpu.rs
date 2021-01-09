use crate::bus::Bus;

pub struct Cpu {
    pc: u32,
    regs: [u32; 32],
    bus: Bus,
}

impl Cpu {
    pub fn new(rom: Vec<u8>) -> Cpu {
        Cpu {
            pc: 0,
            regs: [0; 32],
            bus: Bus::new(rom),
        }
    }

    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        let instruction = self.bus.fetch_from_ram(index);
        return (instruction[0] as u32)
            | ((instruction[1] as u32) << 8)
            | ((instruction[2] as u32) << 16)
            | ((instruction[3] as u32) << 24);
    }

    pub fn execute(&mut self, instruction: u32) {
        todo!()
    }
}
