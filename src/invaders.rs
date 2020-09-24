use crate::cpu::CPUState;
use crate::cpu::emulate_8080_op;
use crate::interrupts::handle_interrupts;

pub struct Machine {
    pub cpu: CPUState,
    pub last_timer: u64,
    pub next_interrupt: u64,
    pub which_interrupt: usize,

    pub shift0: u8, //REGISTER PAIR ACTUALLY
    pub shift1: u8,
    pub shift_offset: u8, //SHIFT AMOUNT
    pub out_bus: u8,

    pub port0: u8,
    pub port1: u8,
    pub port2: u8,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: CPUState::new(),
            last_timer: 0,
            next_interrupt: 0,
            which_interrupt: 0,
            shift0: 0,
            shift1: 0,
            shift_offset: 0,
            out_bus: 0,

            port0: 0b0111_0000,
            port1: 0b0001_0000,
            port2: 0b0000_0000,
        }
    }

    pub fn load_memory(&mut self, rom: &Vec<u8>, size: usize) {
        self.cpu.load_memory(rom, size);
    }


}

pub fn emulate_invaders(machine: Machine, opcode: &[u8]) -> Machine {
    match opcode[0] {
        0xdb | 0xd3 => handle_interrupts(machine, opcode),
        _ => Machine { cpu: emulate_8080_op(machine.cpu, opcode), ..machine}
    }
}
