use std::fs::File;
use crate::invaders::Machine;
use crate::cpu::CPUState;
use crate::condition_codes::{Flags as ConditionCodes};
use std::io::Read;

pub fn new_machine() -> (Machine, Vec<u8>) {
    let mut buffer = Vec::new();
    let mut f = File::open("invaders").unwrap();
    f.read_to_end(&mut buffer).unwrap();
    let mut machine = Machine::new();
    machine.load_rom(0x00);
    (machine , buffer)
}

pub fn parity(byte: u16) -> u16 {
    let mut y = byte;
    y ^= y >> 4;
    y ^= y >> 2;
    y ^= y >> 1;
    (!y) & 1
}

// This is dumb, i should use a HashMap and remove the order problem
pub fn arith_flags(answer: u16) -> (u8, u8, u8, u8, u8) {
    let cy = if answer > 0xff { 1 } else { 0 };
    let z = if answer & 0xff == 0 { 1 } else { 0 };
    let s = if answer & 0x80 == 0x80 { 1 } else { 0 };
    let p = if parity(answer) == 1 { 1 } else { 0 };
    let ac = if answer > 0xf { 1 } else { 0 };
    (z, s, cy, p, ac)
}

pub fn arith_flags_logs(answer: u16) -> (u8, u8, u8, u8, u8) {
    let cy = if answer > 0xff { 1 } else { 0 };
    let z = if answer & 0xff == 0 { 1 } else { 0 };
    let s = if answer & (1 << 7) != 0 { 1 } else { 0 };
    let p = if parity(answer) == 1 { 1 } else { 0 };
    let ac = if answer > 0xf { 1 } else { 0 };
    (z, s, cy, p, ac)
}

fn create_cc_with_arg(arith_flags: (u8, u8, u8, u8, u8)) -> ConditionCodes {
    ConditionCodes {
        z: arith_flags.0,
        s: arith_flags.1,
        cy: arith_flags.2,
        p: arith_flags.3,
        ac: arith_flags.4
    }
}

//Set flags from a byte
pub fn set_psw(psw: u8) -> ConditionCodes {
    let cy = if (psw & 1) != 0 { 1 } else { 0 };
    let p = if (psw & 1 << 2) != 0 { 1 } else { 0 };
    let ac = if (psw & 1 << 4) != 0 { 1 } else { 0 };
    let z = if (psw & 1 << 6) != 0 { 1 } else { 0 };
    let s = if (psw & 1 << 7) != 0 { 1 } else { 0 };
    create_cc_with_arg((z, s, cy, p, ac))
}

pub fn set_all_flags(answer: u16) -> ConditionCodes {
    create_cc_with_arg(arith_flags(answer))
}

pub fn get_value_memory(memory: &Vec<u8>, hr: u8, lr: u8) -> u8 {
    let address: u16 = (hr as u16) << 8 | lr as u16;
    memory[address as usize]
}

pub fn write_memory(mut memory: Vec<u8>, address: u16, value: u8) -> Vec<u8> {
    memory[address as usize] = value;
    memory
}

pub fn generate_interrupt(cpu: CPUState, interrupt_num: u32) -> CPUState {
    //println!("Pushing to Stack(Interrupt): {:04x}",cpu.pc);
    //println!("Pushing to Stack(Interrupt/CY): {:04x}{:04x}{:04x}", cpu.pc, cpu.cc.cy, interrupt_num);
    let pc = cpu.pc;
    let cpu = push_to_stack_addr(cpu, pc);
    //println!("Pushing to Stack(Interrupt): {:04x}",8*(interrupt_num));
    let x = CPUState {
        pc: 8*(interrupt_num as u16),
        int_enable: false,
        ..cpu
    };
    x
}

pub fn pop_from_stack(cpu: CPUState) -> (CPUState, u16) {
    let val = ((cpu.memory[cpu.sp as usize + 1] as u16) << 8) | (cpu.memory[cpu.sp as usize] as u16);
    (CPUState { sp: cpu.sp.wrapping_add(2), ..cpu}, val)
}

pub fn push_to_stack_addr(cpu: CPUState, addr : u16) -> CPUState {
    println!("Pushing to stack addr: {:04x} at SP: {:08x}",addr, cpu.sp);
    let mut memory = cpu.memory;
    memory[cpu.sp as usize - 1] = (addr >> 8) as u8;
    memory[cpu.sp as usize - 2] = addr as u8;
    CPUState {
        memory,
        sp: cpu.sp.wrapping_sub(2),
        ..cpu
    }
}
