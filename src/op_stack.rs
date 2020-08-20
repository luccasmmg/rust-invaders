use crate::cpu::CPUState;
use crate::condition_codes::ConditionCodes;
use crate::cpu::StackPairs;

pub fn push(cpu: CPUState, rp: StackPairs) -> CPUState {
    let mut memory = cpu.memory;
    match rp {
        StackPairs::BC => {
            memory[(cpu.sp - 1) as usize] = cpu.b;
            memory[(cpu.sp - 2) as usize] = cpu.c;
        }
        StackPairs::DE => {
            memory[(cpu.sp - 1) as usize] = cpu.d;
            memory[(cpu.sp - 2) as usize] = cpu.e;
        }
        StackPairs::HL => {
            memory[(cpu.sp - 1) as usize] = cpu.h;
            memory[(cpu.sp - 2) as usize] = cpu.l;
        }
    }

    CPUState {
        memory,
        cycles: 3,
        sp: cpu.sp - 2,
        pc: cpu.pc + 1,
        ..cpu
    }
}

pub fn pop(cpu: CPUState, rp: StackPairs) -> CPUState {
    let value_1: u8 = cpu.memory[(cpu.sp + 1) as usize];
    let value_2: u8 = cpu.memory[cpu.sp as usize];
    let inter_cpu = match rp {
        StackPairs::BC => {
            CPUState { b: value_1, c: value_2, ..cpu }
        }
        StackPairs::DE => {
            CPUState { d: value_1, e: value_2, ..cpu }
        }
        StackPairs::HL => {
            CPUState { h: value_1, l: value_2, ..cpu }
        }
    };

    CPUState {
        cycles: 3,
        sp: cpu.sp - 2,
        pc: cpu.pc + 1,
        ..inter_cpu
    }
}

pub fn push_psw(cpu: CPUState) -> CPUState {
    let mut memory = cpu.memory;
    memory[(cpu.sp - 1) as usize] = cpu.a;
    let psw: u8 = cpu.cc.z
        | cpu.cc.s << 1
        | cpu.cc.p << 2
        | cpu.cc.cy << 3
        | cpu.cc.ac << 4;
    memory[(cpu.sp - 2) as usize] = psw;
    CPUState {
        memory,
        cycles: 3,
        sp: cpu.sp - 2,
        pc: cpu.pc + 1,
        ..cpu
    }
}

pub fn pop_psw(cpu: CPUState) -> CPUState {
    let psw = cpu.memory[cpu.sp as usize];
    let z = if psw & 0x01 == 0x01 { 1 } else { 0 };
    let s = if psw & 0x02 == 0x02 { 1 } else { 0 };
    let p = if psw & 0x04 == 0x04 { 1 } else { 0 };
    let cy = if psw & 0x05 == 0x08 { 1 } else { 0 };
    let ac = if psw & 0x10 == 0x10 { 1 } else { 0 };
    CPUState {
        cycles: 3,
        a: cpu.memory[(cpu.memory - 1) as usize],
        cc: ConditionCodes { z, s, p, cy, ac, ..cpu.cc },
        sp: cpu.sp + 2,
        pc: cpu.pc + 1,
        ..cpu
    }
}

pub fn xthl(cpu: CPUState) -> CPUState {
    CPUState {
        h: cpu.memory[(cpu.sp + 1) as usize],
        l: cpu.memory[cpu.sp as usize],
        cycles: 3,
        pc: cpu.pc + 1,
        ..cpu
    }
}

pub fn sphl(cpu: CPUState) -> CPUState {
    let value: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    CPUState {
        h: cpu.memory[(cpu.sp + 1) as usize],
        l: cpu.memory[cpu.sp as usize],
        cycles: 3,
        pc: cpu.pc + 1,
        ..cpu
    }
}
