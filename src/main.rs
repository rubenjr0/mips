mod bus;
mod cpu;
mod ram;

use crate::cpu::Cpu;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!()
    }

    let mut file = File::open(&args[1]).expect("Couldn't open file");
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).expect("Coudln't read file");

    let mut cpu = Cpu::new(rom);
	// while cpu.can_run() {
	for _ in 0..3 {
		let i = cpu.fetch();
		println!("Executing {:#032b}", i);
		cpu.execute(i);
	}
}
