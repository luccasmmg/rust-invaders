#![allow(dead_code)]
use std::process;
use crate::cpu::CPUState;

pub fn ei(cpu: CPUState) -> CPUState {
    CPUState {
        pc: cpu.pc + 1,
        int_enable: 1,
        cycles: 1,
        ..cpu
    }
}

pub fn di(cpu: CPUState) -> CPUState {
    CPUState {
        pc: cpu.pc + 1,
        int_enable: 0,
        cycles: 1,
        ..cpu
    }
}

pub fn hlt() {
    process::exit(0);
}

pub fn in(cpu: CPUState) -> CPUState {
    CPUState {
        pc: cpu.pc + 1,
        cycles: 3,
        ..cpu
    }
}

pub fn out(cpu: CPUState) -> CPUState {
    CPUState {
        pc: cpu.pc + 1,
        cycles: 3,
        ..cpu
    }
}

pub fn nop(cpu: CPUState) -> CPUState {
    CPUState {
        pc: cpu.pc + 1,
        cycles: 1,
        ..cpu
    }
}
