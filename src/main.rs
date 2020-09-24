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
        println!("{}", machine.cpu);
        n += 1;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Read;
    use cpu::CPUState;
    use dissassembler::disassemble;
    use std::fs::File;
    use cpu::emulate_8080_op;

    #[test]
    fn test_37_410_instructions_cpu() {
        let mut cpu = CPUState::new();
        let mut buffer = Vec::new();
        let mut f = File::open("invaders").unwrap();
        f.read_to_end(&mut buffer).unwrap();
        cpu.load_memory(&buffer, buffer.len());
        let mut n = 0;
        while n < 37410 {
            let opcodes: &[u8] = &buffer[cpu.pc as usize..];
            cpu = emulate_8080_op(cpu, opcodes);
            disassemble(&buffer[cpu.pc as usize..], cpu.pc as usize);
            n += 1;
        }
        assert_eq!(cpu.pc , 0x090e)
    }

    #[test]
    fn test_full_ops() {
        let mut cpu = CPUState::new();
        let mut buffer = Vec::new();
        let mut f = File::open("cpudiag.bin").unwrap();
        f.read_to_end(&mut buffer).unwrap();
        let buffer = {
            let mut padding = vec![0; 0x100];
            padding[0] = 0xc3;
            padding[1] = 0x00;
            padding[2] = 0x01;
            padding.append(&mut buffer);
            //padding[368] = 0x7;
            padding[0x59c] = 0xc3;
            padding[0x59d] = 0xc2;
            padding[0x59e] = 0x05;
            padding[0x319] = 0x00;
            padding[0x31a] = 0x00;
            padding[0x31b] = 0x00;
            padding[0x31c] = 0x00;
            cpu.pc = 0x100;
            padding
        };
        cpu.load_memory(&buffer, buffer.len());
        let mut n = 0;
        while n < 590 {
            let opcodes: &[u8] = &buffer[cpu.pc as usize..];
            cpu = emulate_8080_op(cpu, opcodes);
            disassemble(&buffer[cpu.pc as usize..], cpu.pc as usize);
            n += 1;
        }
        assert_eq!(cpu.pc, 0x0688);
    }

}
