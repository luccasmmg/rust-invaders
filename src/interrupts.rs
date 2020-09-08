use crate::invaders::Machine;
use crate::cpu::CPUState;
use crate::op_special_io::op_in;

pub fn handle_interrupts(machine: Machine, opcode: &[u8]) -> Machine {
    match opcode[0] {
        0xdb => in_space_invaders(machine, opcode[1]),
        0xd3 => out_space_invaders(machine, opcode[1]),
        _ => machine
    }
}

fn in_space_invaders(machine: Machine, port: u8) -> Machine {
    match port {
        3 => {
            let value = machine.shift1 << 8 | machine.shift0;
            Machine { cpu: op_in(machine.cpu, value), ..machine }
        }
        _ => machine
    }
}

fn out_space_invaders(machine: Machine, port: u8) -> Machine {
    let value = machine.cpu.a;
    match port {
        2 => {
            Machine { shift_offset: value & 0x7, ..machine}
        },
        4 => {
            Machine {
                shift0: machine.shift1,
                shift1: value,
                ..machine
            }
        },
        _ => machine,
    }
}
