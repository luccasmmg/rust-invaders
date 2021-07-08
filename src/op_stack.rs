use crate::cpu::CPUState;
use crate::cpu::StackPairs;
use crate::helpers::{pop_from_stack, set_psw, write_memory};

fn push_to_stack_addr(cpu: CPUState, addr : u16) -> CPUState {
    let mut memory = cpu.memory;
    memory[cpu.sp as usize - 1] = (addr >> 8) as u8;
    memory[cpu.sp as usize - 2] = addr as u8;
    CPUState {
        memory,
        sp: cpu.sp.wrapping_sub(2),
        ..cpu
    }
}

pub fn push(cpu: CPUState, rp: StackPairs) -> CPUState {
    let new_cpu = match rp {
        StackPairs::BC => {
            let addr = ((cpu.b as u16) << 8) | cpu.c as u16;
            push_to_stack_addr(cpu, addr)
        }
        StackPairs::DE => {
            let addr = ((cpu.d as u16) << 8) | cpu.e as u16;
            push_to_stack_addr(cpu, addr)
        }
        StackPairs::HL => {
            let addr = ((cpu.h as u16) << 8) | cpu.l as u16;
            push_to_stack_addr(cpu, addr)
        }
    };

    CPUState {
        cycles: 3,
        pc: new_cpu.pc.wrapping_add(1),
        ..new_cpu
    }
}

pub fn pop(cpu: CPUState, rp: StackPairs) -> CPUState {
    let value_l: u8 = cpu.memory[cpu.sp as usize];
    let value_h: u8 = cpu.memory[(cpu.sp.wrapping_add(1)) as usize];
    let inter_cpu = match rp {
        StackPairs::BC => {
            CPUState { b: value_h, c: value_l, ..cpu }
        }
        StackPairs::DE => {
            CPUState { d: value_h, e: value_l, ..cpu }
        }
        StackPairs::HL => {
            CPUState { h: value_h, l: value_l, ..cpu }
        }
    };

    CPUState {
        cycles: 3,
        sp: cpu.sp.wrapping_add(2),
        pc: cpu.pc.wrapping_add(1),
        ..inter_cpu
    }
}

pub fn push_psw(cpu: CPUState) -> CPUState {
    let mut memory = cpu.memory;
    let mut psw:u16 = 0;
    let s = if cpu.cc.s == 1 { 1 } else { 0 };
    let z = if cpu.cc.z == 1 { 1 } else { 0 };
    let ac = if cpu.cc.ac == 1 { 1 } else { 0 };
    let p = if cpu.cc.p == 1 { 1 } else { 0 };
    let cy = if cpu.cc.cy == 1 { 1 } else { 0 };

    psw |= s << 7;
    psw |= z << 6;
    psw |= 0 << 5;
    psw |= ac << 4;
    psw |= 0 << 3;
    psw |= p << 2;
    psw |= 1 << 1;
    psw |= cy;
    psw |= (cpu.a as u16) << 8;
    memory[cpu.sp as usize - 1] = (psw >> 8) as u8;
    memory[cpu.sp as usize - 2] = psw as u8;
    CPUState {
        memory,
        cycles: 3,
        sp: cpu.sp.wrapping_sub(2),
        pc: cpu.pc.wrapping_add(1),
        ..cpu
    }
}

pub fn pop_psw(cpu: CPUState) -> CPUState {
    let (cpu, data) = pop_from_stack(cpu);
    let a = (data >> 8) as u8;
    CPUState {
        cycles: 3,
        a,
        cc: set_psw(data as u8),
        sp: cpu.sp.wrapping_add(2),
        pc: cpu.pc.wrapping_add(1),
        ..cpu
    }
}

pub fn xthl(cpu: CPUState) -> CPUState {
    let l = cpu.memory[cpu.sp as usize];
    let h = cpu.memory[(cpu.sp.wrapping_add(1)) as usize];
    let mut memory = write_memory(cpu.memory, cpu.sp, cpu.l);
    memory = write_memory(memory, cpu.sp.wrapping_add(1), cpu.h);
    CPUState {
        l,
        h,
        cycles: 3,
        pc: cpu.pc.wrapping_add(1),
        memory,
        ..cpu
    }
}

pub fn sphl(cpu: CPUState) -> CPUState {
    let value: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    CPUState {
        sp: value,
        cycles: 3,
        pc: cpu.pc.wrapping_add(1),
        ..cpu
    }
}
