use crate::cpu::CPUState;
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
        ..cpu
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
        ..cpu
    }
}

pub fn pop_psw(cpu: CPUState) -> CPUState {
    let psw = cpu.memory[cpu.sp];
    cpu
}
