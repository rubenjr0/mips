mod bus;
mod cpu;
mod ram;

use crate::cpu::Cpu;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!()
    }
    let mut file = File::open(&args[1]).expect("re");
    let mut rom = Vec::new();
    file.read_to_end(&mut rom);

    let cpu = Cpu::new(rom);
    let i = cpu.fetch();
    println!("{}", i);
}
