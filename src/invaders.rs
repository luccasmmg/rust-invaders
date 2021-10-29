use crate::cpu::CPUState;
use crate::cpu::emulate_8080_op;
use crate::interrupts::handle_interrupts;
use std;

#[derive(Debug, PartialEq)]
pub struct Machine {
    pub cpu: CPUState,

    //LSB of Space Invader's external shift hardware
    pub shift0: u8,
    //MSB
    pub shift1: u8,
    // offset for external shift hardware
    pub shift_offset: u8,
    pub in_port1: u8,
    pub in_port2: u8,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: CPUState::new(),
            shift0: 0,
            shift1: 0,
            shift_offset: 0,
            in_port1: 0b0001_0000,
            in_port2: 0b0000_0000,
        }
    }

    pub fn load_rom(&mut self, start: usize) {
        let x = std::include_bytes!("invaders.rom");
        let mut i = 0;
        if x.len() > start+0xffff {
            panic!("PANIC: Rom size exceeds Memory!!");
        } else {
            while i< x.len() {
                self.cpu.memory[start+i] = x[i];
                i += 1;
            }
        }
    }

}

pub fn emulate_invaders(machine: Machine) -> Machine {
    let opcode: u8 = machine.cpu.memory[machine.cpu.pc as usize];
    //println!("Opcode = {} PC = {}, SP = {}", opcode, machine.cpu.pc, machine.cpu.sp);
    match opcode {
        0xdb | 0xd3 => handle_interrupts(machine),
        _ => Machine { cpu: emulate_8080_op(machine.cpu), ..machine}
    }
}
