use std::io;
use std::io::Read;
use cpu::CPUState;
use cpu::emulate_8080_op;
use dissassembler::disassemble;
use std::fs::File;
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

fn main() -> io::Result<()> {
    let mut cpu = CPUState::new();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut buffer = Vec::new();
    let mut f = File::open(filename)?;
    f.read_to_end(&mut buffer)?;
    let buffer = if args[3] == "test" {
        let mut padding = vec![0; 0x100];
        padding.append(&mut buffer);
        cpu.pc = 0xff;
        padding
    } else {
        buffer
    };
    cpu.load_memory(&buffer, buffer.len());
    let mut n = 0;
    while n < args[2].parse::<i64>().unwrap() {
        let pc: u16 = cpu.pc;
        cpu = emulate_8080_op(cpu, &buffer, pc);
        if args[3] == "test" {
            println!("PC - {:02x}", cpu.pc);
        } else {
            println!("{}", cpu);
        }
        n += 1;
    }
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
