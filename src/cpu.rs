use crate::bus::Bus;
use crate::ram::RAM_SIZE;

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

    pub fn can_run(&self) -> bool {
        (self.pc as usize) < RAM_SIZE
    }

    pub fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        let instruction = self.bus.fetch_from_ram(index);
		println!("Got {:?}", instruction);
        return ((instruction[0] as u32) << 24)
            | ((instruction[1] as u32) << 16)
            | ((instruction[2] as u32) << 8)
            | ((instruction[3] as u32));
    }

    pub fn execute(&mut self, instruction: u32) {
        let opcode = instruction >> 26;
		print!("opcode: {:#02x} -> ", opcode);
		match opcode {
			0x2 => println!("J"),
			0x4 => println!("BEQ"),
			0x0 => println!("ALU"),
			_ => unimplemented!()
		}
		self.pc += 4;
    }
}
