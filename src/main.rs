use std::io;
use std::io::Read;
use cpu::CPUState;
use cpu::emulate_8080_op;
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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut buffer = Vec::new();
    let mut f = File::open(filename)?;
    f.read_to_end(&mut buffer)?;
    let mut cpu = CPUState::new();
    //Load ROM
    cpu.load_memory(&buffer);
    let mut n = 0;
    while n < args[2].parse::<i64>().unwrap() {
        n += 1;
        let pc: u16 = cpu.pc;
        println!("{:02x}", pc);
        cpu = emulate_8080_op(cpu, &buffer, pc);
        println!("{}", cpu);
    }
    Ok(())
}

fn main_loop(n: usize, cpu: CPUState, rom: &[u8]) -> CPUState {
    if n == 0 {
        return cpu
    } else {
        let pc: u16 = cpu.pc;
        main_loop(n - 1, emulate_8080_op(cpu , rom, pc), rom)
    }
}
