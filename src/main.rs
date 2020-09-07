use std::io;
use cpu::emulate_8080_op;
use helpers::load_memory;
use std::env;

mod condition_codes;
mod cpu;
mod helpers;
mod op_arithmetic;
mod op_branch;
mod op_data_transfer;
mod op_logical;
mod op_special_io;
mod op_stack;
mod dissassembler;
//mod interrupts;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let (mut cpu, buffer) = load_memory(filename);
    let mut n = 0;
    while n < args[2].parse::<i64>().unwrap() {
        let opcodes: &[u8] = &buffer[cpu.pc as usize..];
        cpu = emulate_8080_op(cpu, opcodes);
        n += 1;
    }
    println!("{}", cpu);
    Ok(())
}

// fn main_loop(n: usize, cpu: CPUState, rom: &[u8]) -> CPUState {
//     if n == 0 {
//         return cpu
//     } else {
//         let pc: u16 = cpu.pc;
//         main_loop(n - 1, emulate_8080_op(cpu , rom, pc), rom)
//     }
// }
