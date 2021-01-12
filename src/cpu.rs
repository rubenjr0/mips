use crate::bus::Bus;

pub struct Cpu {
    pc: u32,
    regs: [u32; 32],
    rom: Vec<u8>,
    bus: Bus,
}

impl Cpu {
    pub fn new(rom: Vec<u8>) -> Cpu {
        Cpu {
            pc: 0,
            regs: [0; 32],
            rom: rom,
            bus: Bus::new(),
        }
    }

    pub fn can_run(&self) -> bool {
        (self.pc as usize) < (self.rom.len() - 4)
    }

    fn read_reg(&self, rx: usize) -> u32 {
        self.regs[rx]
    }

    fn write_reg(&mut self, rx: usize, dt: u32) {
        if rx != 0 {
            self.regs[rx] = dt;
        }
    }

    pub fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        ((self.rom[index] as u32) << 24) |
        ((self.rom[index + 1] as u32) << 16) |
        ((self.rom[index + 2] as u32) << 8)|
        (self.rom[index + 3] as u32)
    }

    pub fn execute(&mut self, instruction: u32) {
		if instruction != 0 {
			let opcode = (instruction >> 26) as u8;
			let jdir = instruction & 0x3FFFFFF;
            let rs = ((instruction >> 21) & 0x1F) as usize;
            let rt = ((instruction >> 16) & 0x1F) as usize;
            let ct = (instruction & 0xFFFF) as u32;
			print!("{:04X} - {:08X} -> ", self.pc, instruction);
			match opcode {
				0x2 => {
                    println!("J {}", jdir);
                    self.pc = (((self.pc + 4) >> 26) | jdir) << 2;
                },
				0x4 => {
                    println!("BEQ ${}, ${}, {:04X}", rs, rt, ct);
                    if self.read_reg(rs) == self.read_reg(rt) {
                        self.pc += 4 + (ct << 2);
                    } else {
                        self.pc += 4;
                    }
                },
				0x0 => {
                    println!("ALU");
                    self.pc += 4;
                },
				0x8 => {
                    println!("ADDI {}, {}, {}", rt, rs, ct);
                    self.write_reg(rt, self.read_reg(rs) + ct);
                    self.pc += 4;
                },
				0x23 => {
                    println!("LW ${}, {}(${})", rt, ct, rs);
                    self.write_reg(
                        rt, 
                        self.bus.read_word((self.read_reg(rs) + ct) as usize)
                    );
                    self.pc += 4;
                },
				0x2B => {
                    println!("SW ${}, {}(${})", rt, ct, rs);
                    self.bus.write_word(
                        (self.read_reg(rs) + ct) as usize, 
                        self.read_reg(rt)
                    );
                    self.pc += 4;
                },
				0x3F => {
                    println!("HALT");
                    self.pc += 4;
                },
				_ => unimplemented!("OPCODE {:#02x} is not implemented", opcode)
			}
		} else { 
            self.pc += 4;
        }
    }

    pub fn print_mem(&self, to: usize) {
        for row in 0..(to/4)+1 {
            print!("{:02X} | ", row * 4);
            for col in 0..4 {
                print!("{:02X} ", self.bus.read_byte((4 * row + col) as usize));
            }
            println!();
        }
    }
}
