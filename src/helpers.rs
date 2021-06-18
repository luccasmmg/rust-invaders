use std::fs::File;
use crate::invaders::Machine;
use crate::cpu::CPUState;
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
pub fn arith_flags(answer: u16) -> (u8, u8, u8, u8) {
    let cy = if answer > 0xff { 1 } else { 0 };
    let z = if answer & 0xff == 0 { 1 } else { 0 };
    let s = if answer & 0x80 == 0x80 { 1 } else { 0 };
    let p = if parity(answer) == 1 { 1 } else { 0 };
    (z, s, cy, p)
}

pub fn arith_flags_logs(answer: u16) -> (u8, u8, u8, u8) {
    let cy = if answer > 0xff { 1 } else { 0 };
    let z = if answer & 0xff == 0 { 1 } else { 0 };
    let s = if answer & (1 << 7) != 0 { 1 } else { 0 };
    let p = if parity(answer) == 1 { 1 } else { 0 };
    (z, s, cy, p)
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
    println!("Pushing to static(interrupt): {:04x}", cpu.pc);
    let mut memory = cpu.memory;
    memory[(cpu.sp - 1) as usize] = (cpu.pc >> 8) as u8;
    memory[(cpu.sp - 2) as usize] = cpu.pc as u8;
    println!("PC is at: {:04x}", 8*(interrupt_num as u16));
    CPUState {
        memory,
        sp: cpu.sp.wrapping_sub(2),
        pc: 8*(interrupt_num as u16),
        int_enable: false,
        ..cpu
    }
}
