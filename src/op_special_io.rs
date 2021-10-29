#![allow(dead_code)]
use std::process;
use crate::cpu::CPUState;
use crate::helpers::{set_all_flags, arith_flags};

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

pub fn daa(cpu: CPUState) -> CPUState {
    let mut result = cpu.a as u16;
    let mut ac = cpu.cc.ac;
    let lsb = result & 0xf;
    if (cpu.cc.ac == 1) || lsb > 9 {
        result += 6;
        if result & 0xf < lsb { ac = 1 }
    }
    let lsb = result & 0xf;
    let mut msb = (result >> 4) & 0xf;
    if (cpu.cc.cy == 1) || msb > 9 { msb += 6 }
    let result = (msb << 4) | lsb;
    let mut all_flags = set_all_flags(result);
    all_flags.ac = ac;
    CPUState {
        pc: cpu.pc.wrapping_add(1),
        a: result as u8,
        cc: all_flags,
        ..cpu
    }
}
