use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;

mod cpu;
mod condition_codes;
mod helpers;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut buffer = Vec::new();
    let mut f = File::open(filename)?;
    f.read_to_end(&mut buffer)?;

    let num1: u8 = 230;
    let num2: u8 = 1;
    let num3: u8 = num2.wrapping_sub(num1);
    println!("Num 3 is {:02x}", num3);

    Ok(())
}

