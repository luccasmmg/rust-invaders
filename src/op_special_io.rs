#![allow(dead_code)]
use std::process;
use crate::cpu::CPUState;

pub fn ei(cpu: CPUState) -> CPUState {
    CPUState {
        pc: cpu.pc.wrapping_add(1),
        int_enable: true,
        cycles: 1,
        ..cpu
    }
}

pub fn di(cpu: CPUState) -> CPUState {
    CPUState {
        pc: cpu.pc.wrapping_add(1),
        int_enable: false,
        cycles: 1,
        ..cpu
    }
}

pub fn hlt() {
    process::exit(0);
}

pub fn op_in(cpu: CPUState, value: u8) -> CPUState {
    CPUState {
        a: value,
        pc: cpu.pc.wrapping_add(2),
        cycles: 3,
        ..cpu
    }
}

pub fn out(cpu: CPUState) -> CPUState {
    CPUState {
        pc: cpu.pc.wrapping_add(2),
        cycles: 3,
        ..cpu
    }
}

pub fn nop(cpu: CPUState) -> CPUState {
    CPUState {
        pc: cpu.pc.wrapping_add(1),
        cycles: 1,
        ..cpu
    }
}
