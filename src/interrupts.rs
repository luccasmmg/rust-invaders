use crate::invaders::Machine;
use crate::cpu::CPUState;
use crate::op_special_io::op_in;

pub fn handle_interrupts(machine: Machine) -> Machine {
    let opcode: u8 = machine.cpu.memory[machine.cpu.pc as usize];
    let next_opcode: u8 = machine.cpu.memory[machine.cpu.pc.wrapping_add(1) as usize];
    match opcode {
        0xdb => in_space_invaders(machine, next_opcode),
        0xd3 => out_space_invaders(machine, next_opcode),
        _ => machine
    }
}

fn in_space_invaders(machine: Machine, port: u8) -> Machine {
    match port {
        0 => Machine { cpu: op_in(machine.cpu, 0xf), ..machine },
        1 => Machine { cpu: op_in(machine.cpu, machine.in_port1), ..machine },
        2 => Machine { cpu: op_in(machine.cpu, machine.in_port2), ..machine },
        3 => {
            let v = ((machine.shift1 as u16) << 8) | (machine.shift0 as u16);
            let value = (v >> (8-(machine.shift_offset as u16))) as u8;
            Machine { cpu: op_in(machine.cpu, value), ..machine }
        }
        _ => Machine { cpu: op_in(machine.cpu, 0), ..machine }
    }
}

fn out_space_invaders(machine: Machine, port: u8) -> Machine {
    let value = machine.cpu.a;
    match port {
        2 => {
            Machine { cpu: CPUState { pc: machine.cpu.pc + 1, ..machine.cpu },
                      shift_offset: value & 0x7,
                      ..machine}
        },
        4 => {
            Machine {
                shift0: machine.shift1,
                shift1: value,
                cpu: CPUState { pc: machine.cpu.pc + 1, ..machine.cpu },
                ..machine
            }
        },
        6 => Machine { cpu: CPUState { pc: machine.cpu.pc + 1, ..machine.cpu }, ..machine},
        _ => {
            Machine { cpu: CPUState { pc: machine.cpu.pc + 1, ..machine.cpu }, ..machine}
        }
    }
}
