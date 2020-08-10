use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;

mod cpu;
mod condition_codes;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut buffer = Vec::new();
    let mut f = File::open(filename)?;
    f.read_to_end(&mut buffer)?;

    let num1: u8 = 0x7e;
    let num2: u8 = 0x7d;
    let num3: u16 = (num1 as u16) <<8|num2 as u16;
    println!("Num 3 is {:04x}", num3);

    Ok(())
}

