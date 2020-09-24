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
            let value = (machine.shift_value >> (8 - machine.shift_offset)) as u8;
            Machine { cpu: op_in(machine.cpu, value), ..machine }
        }
        _ => Machine { cpu: op_in(machine.cpu, machine.ports[port as usize]), ..machine }
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
                shift_value: (machine.shift_value >> 8 | u16::from(value) << 8),
                ..machine
            }
        },
        6 => machine,
        _ => {
            let mut new_ports = machine.ports;
            new_ports[port as usize] = value;
            Machine { cpu: CPUState { pc: machine.cpu.pc + 1, ..machine.cpu }, ports: new_ports, ..machine}
        }
    }
}
