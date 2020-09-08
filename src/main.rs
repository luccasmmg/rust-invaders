mod helpers;

mod invaders;

mod cpu;
mod condition_codes;
mod op_arithmetic;
mod op_branch;
mod op_data_transfer;
mod op_logical;
mod op_special_io;
mod op_stack;
mod dissassembler;

use std::io;
use invaders::emulate_invaders;
use helpers::new_machine;
use std::env;
mod interrupts;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (mut machine, buffer) = new_machine();
    let mut n = 0;
    while n < args[1].parse::<i64>().unwrap() {
        let opcodes: &[u8] = &buffer[machine.cpu.pc as usize..];
        machine = emulate_invaders(machine, opcodes);
        n += 1;
    }
    println!("{}", machine.cpu);
    Ok(())
}
