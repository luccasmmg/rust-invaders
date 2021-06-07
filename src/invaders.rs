use crate::cpu::CPUState;
use crate::cpu::emulate_8080_op;
use crate::interrupts::handle_interrupts;

const SCALE: u32 = 8;

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

    pub fn load_memory(&mut self, rom: &Vec<u8>, size: usize) {
        self.cpu.load_memory(rom, size);
    }

}

pub fn emulate_invaders(mut machine: Machine, opcode: &[u8]) -> Machine {
    match opcode[0] {
        0xdb | 0xd3 => handle_interrupts(machine, opcode),
        _ => Machine { cpu: emulate_8080_op(machine.cpu, opcode), ..machine}
    }
}
