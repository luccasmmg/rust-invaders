use crate::condition_codes::ConditionCodes;
use crate::cpu::CPUState;
use crate::cpu::WithSPPairs;
use crate::helpers::arith_flags;

pub fn add(addendum: u8, cycles: u8, cpu: CPUState) -> CPUState {
    let answer: u16 = (cpu.a as u16).wrapping_add(addendum as u16);
    let cc = arith_flags(answer);
    let a = answer as u8;
    let flags = ConditionCodes {
        z: cc.0,
        s: cc.1,
        cy: cc.2,
        p: cc.3,
        ..cpu.cc
    };
    CPUState {
        a,
        cc: flags,
        pc: cpu.pc + 1,
        cycles,
        ..cpu
    }
}

pub fn adi(addendum: u8, cycles: u8, cpu: CPUState) -> CPUState {
    let answer: u16 = (cpu.a as u16).wrapping_add(addendum as u16);
    let cc = arith_flags(answer);
    let a = answer as u8;
    let flags = ConditionCodes {
        z: cc.0,
        s: cc.1,
        cy: cc.2,
        p: cc.3,
        ..cpu.cc
    };
    CPUState {
        a,
        cc: flags,
        pc: cpu.pc + 2,
        cycles,
        ..cpu
    }
}

pub fn sub(subtraend: u8, cycles: u8, cpu: CPUState) -> CPUState {
    let answer: u16 = (cpu.a as u16).wrapping_sub(subtraend as u16);
    let cc = arith_flags(answer);
    let a = answer as u8;
    let flags = ConditionCodes {
        z: cc.0,
        s: cc.1,
        cy: cc.2,
        p: cc.3,
        ..cpu.cc
    };
    CPUState {
        a,
        cc: flags,
        pc: cpu.pc + 1,
        cycles,
        ..cpu
    }
}

pub fn sui(subtraend: u8, cycles: u8, cpu: CPUState) -> CPUState {
    let answer: u16 = (cpu.a as u16).wrapping_sub(subtraend as u16);
    let cc = arith_flags(answer);
    let flags = ConditionCodes {
        z: cc.0,
        s: cc.1,
        cy: cc.2,
        p: cc.3,
        ..cpu.cc
    };
    let a = answer as u8;
    CPUState {
        a,
        cc: flags,
        pc: cpu.pc + 2,
        cycles,
        ..cpu
    }
}

pub fn inr_r(cpu: CPUState, r: char) -> CPUState {
    let (inter_cpu, answer ) = match r {
        'a' => {
            let answer: u16 = (cpu.a as u16).wrapping_add(1 as u16);
            (CPUState { a: answer as u8, pc: cpu.pc + 1, ..cpu }, answer)
        }
        'b' => {
            let answer: u16 = (cpu.b as u16).wrapping_add(1 as u16);
            (CPUState { b: answer as u8, pc: cpu.pc + 1,  ..cpu }, answer)
        }
        'c' => {
            let answer: u16 = (cpu.c as u16).wrapping_add(1 as u16);
            (CPUState { c: answer as u8, pc: cpu.pc + 1,  ..cpu }, answer)
        }
        'd' => {
            let answer: u16 = (cpu.d as u16).wrapping_add(1 as u16);
            (CPUState { d: answer as u8, pc: cpu.pc + 1,  ..cpu }, answer)
        }
        'e' => {
            let answer: u16 = (cpu.e as u16).wrapping_add(1 as u16);
            (CPUState { e: answer as u8, pc: cpu.pc + 1,  ..cpu }, answer)
        }
        'h' => {
            let answer: u16 = (cpu.h as u16).wrapping_add(1 as u16);
            (CPUState { h: answer as u8, pc: cpu.pc + 1,  ..cpu }, answer)
        }
        'l' => {
            let answer: u16 = (cpu.l as u16).wrapping_add(1 as u16);
            (CPUState { l: answer as u8, pc: cpu.pc + 1,  ..cpu }, answer)
        }
        _ => (CPUState {pc: cpu.pc + 1,  ..cpu }, 0),
    };
    let cc = arith_flags(answer);
    let flags = ConditionCodes {
        z: cc.0,
        s: cc.1,
        p: cc.3,
        ..inter_cpu.cc
    };
    CPUState { cc: flags, pc: cpu.pc + 1, ..inter_cpu }
}
pub fn inr_m(cpu: CPUState) -> CPUState {
    let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    let mut memory = cpu.memory;
    let answer: u16 = (memory[address as usize] as u16).wrapping_add(1 as u16);
    let cc = arith_flags(answer);
    memory[address as usize] = answer.to_be_bytes()[1];
    let flags = ConditionCodes {
        z: cc.0,
        s: cc.1,
        p: cc.3,
        ..cpu.cc
    };
    CPUState {
        memory,
        cc: flags,
        pc: cpu.pc + 3,
        cycles: 3,
        ..cpu
    }
}

pub fn dcr_m(cpu: CPUState) -> CPUState {
    let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    let mut memory = cpu.memory;
    let answer: u16 = (memory[address as usize] as u16).wrapping_sub(1 as u16);
    memory[address as usize] = answer.to_be_bytes()[1];
    let cc = arith_flags(answer);
    let flags = ConditionCodes {
        z: cc.0,
        s: cc.1,
        p: cc.3,
        ..cpu.cc
    };
    CPUState {
        memory,
        cc: flags,
        pc: cpu.pc + 1,
        cycles: 3,
        ..cpu
    }
}

pub fn dcr_r(cpu: CPUState, r: char) -> CPUState {
    let (inter_cpu, answer ) = match r {
        'a' => {
            let answer: u16 = (cpu.a as u16).wrapping_sub(1 as u16);
            (CPUState { a: answer as u8, pc: cpu.pc + 1, ..cpu }, answer)
        }
        'b' => {
            let answer: u16 = (cpu.b as u16).wrapping_sub(1 as u16);
            (CPUState { b: answer as u8, pc: cpu.pc + 1, ..cpu }, answer)
        }
        'c' => {
            let answer: u16 = (cpu.c as u16).wrapping_sub(1 as u16);
            (CPUState { c: answer as u8, pc: cpu.pc + 1, ..cpu }, answer)
        }
        'd' => {
            let answer: u16 = (cpu.d as u16).wrapping_sub(1 as u16);
            (CPUState { d: answer as u8, pc: cpu.pc + 1, ..cpu }, answer)
        }
        'e' => {
            let answer: u16 = (cpu.e as u16).wrapping_sub(1 as u16);
            (CPUState { e: answer as u8, pc: cpu.pc + 1, ..cpu }, answer)
        }
        'h' => {
            let answer: u16 = (cpu.h as u16).wrapping_sub(1 as u16);
            (CPUState { h: answer as u8, pc: cpu.pc + 1, ..cpu }, answer)
        }
        'l' => {
            let answer: u16 = (cpu.l as u16).wrapping_sub(1 as u16);
            (CPUState { l: answer as u8, pc: cpu.pc + 1, ..cpu }, answer)
        }
        _ => (CPUState { ..cpu }, 0),
    };
    let cc = arith_flags(answer);
    let flags = ConditionCodes {
        z: cc.0,
        s: cc.1,
        p: cc.3,
        ..inter_cpu.cc
    };
    CPUState { cc: flags, pc: cpu.pc + 1, ..inter_cpu }
}

pub fn inx(cpu: CPUState, rp: WithSPPairs) -> CPUState {
    match rp {
        WithSPPairs::BC => {
            let result =
                (((cpu.b as u16) << 8 | cpu.c as u16).wrapping_add(1 as u16)).to_be_bytes();
            CPUState {
                b: result[0],
                c: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        }
        WithSPPairs::DE => {
            let result =
                (((cpu.d as u16) << 8 | cpu.e as u16).wrapping_add(1 as u16)).to_be_bytes();
            CPUState {
                d: result[0],
                e: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        }
        WithSPPairs::HL => {
            let result =
                (((cpu.h as u16) << 8 | cpu.l as u16).wrapping_add(1 as u16)).to_be_bytes();
            CPUState {
                h: result[0],
                l: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        }
        WithSPPairs::SP => CPUState {
            sp: cpu.sp.wrapping_add(1 as u16),
            cycles: 1,
            pc: cpu.pc + 1,
            ..cpu
        },
    }
}

pub fn dcx(cpu: CPUState, rp: WithSPPairs) -> CPUState {
    match rp {
        WithSPPairs::BC => {
            let result =
                (((cpu.b as u16) << 8 | cpu.c as u16).wrapping_sub(1 as u16)).to_be_bytes();
            CPUState {
                b: result[0],
                c: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        }
        WithSPPairs::DE => {
            let result =
                (((cpu.d as u16) << 8 | cpu.e as u16).wrapping_sub(1 as u16)).to_be_bytes();
            CPUState {
                d: result[0],
                e: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        }
        WithSPPairs::HL => {
            let result =
                (((cpu.h as u16) << 8 | cpu.l as u16).wrapping_sub(1 as u16)).to_be_bytes();
            CPUState {
                h: result[0],
                l: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        }
        WithSPPairs::SP => CPUState {
            sp: cpu.sp.wrapping_sub(1 as u16),
            pc: cpu.pc + 1,
            cycles: 1,
            ..cpu
        }
    }
}

pub fn dad(cpu: CPUState, rp: (char, char)) -> CPUState {
    let rp_hl: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    if rp == ('s', 'p') {
        return CPUState {
            sp: cpu.sp.wrapping_add(rp_hl),
            pc: cpu.pc + 1,
            cycles: 1,
            ..cpu
        }
    }
    let value_to_add = match rp {
        ('b', 'c') => (((cpu.b as u16) << 8 | cpu.c as u16).wrapping_add(rp_hl)).to_be_bytes(),
        ('d', 'e') => (((cpu.d as u16) << 8 | cpu.e as u16).wrapping_add(rp_hl)).to_be_bytes(),
        ('h', 'l') => (((cpu.h as u16) << 8 | cpu.l as u16).wrapping_add(rp_hl)).to_be_bytes(),
        _ => rp_hl.to_be_bytes(),
    };
    CPUState { h: value_to_add[0], l: value_to_add[1], pc: cpu.pc + 1, cycles: 3, ..cpu}
}
