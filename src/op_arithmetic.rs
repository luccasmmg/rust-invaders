use crate::condition_codes::ConditionCodes;
use crate::cpu::CPUState;
use crate::helpers::arith_flags;

pub fn add(addendum: u8, cycles: u8, cpu: CPUState) -> CPUState {
    let answer: u16 = (cpu.a as u16).wrapping_add(addendum as u16);
    let cc = arith_flags(answer);
    let a = answer.to_le_bytes()[0];
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

pub fn adi(cpu: CPUState, addendum: u8, cycles: u8) -> CPUState {
    let answer: u16 = (cpu.a as u16).wrapping_add(addendum as u16);
    let cc = arith_flags(answer);
    let a = answer.to_le_bytes()[0];
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
    let a = answer.to_le_bytes()[0];
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

pub fn sui(cpu: CPUState, subtraend: u8, cycles: u8) -> CPUState {
    let answer: u16 = (cpu.a as u16).wrapping_sub(subtraend as u16);
    let cc = arith_flags(answer);
    let a = answer.to_le_bytes()[0];
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

pub fn inr_r(cpu: CPUState, r: char) -> CPUState {
    let (inter_cpu, answer ) = match r {
        'a' => {
            let answer: u16 = (cpu.a as u16).wrapping_add(1 as u16);
            (CPUState { a: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'b' => {
            let answer: u16 = (cpu.b as u16).wrapping_add(1 as u16);
            (CPUState { b: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'c' => {
            let answer: u16 = (cpu.c as u16).wrapping_add(1 as u16);
            (CPUState { c: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'd' => {
            let answer: u16 = (cpu.d as u16).wrapping_add(1 as u16);
            (CPUState { d: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'e' => {
            let answer: u16 = (cpu.e as u16).wrapping_add(1 as u16);
            (CPUState { e: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'h' => {
            let answer: u16 = (cpu.h as u16).wrapping_add(1 as u16);
            (CPUState { h: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'l' => {
            let answer: u16 = (cpu.l as u16).wrapping_add(1 as u16);
            (CPUState { l: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        _ => (cpu, 0),
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
    memory[address as usize] = answer.to_be_bytes()[0];
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
    memory[address as usize] = answer.to_be_bytes()[0];
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
            (CPUState { a: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'b' => {
            let answer: u16 = (cpu.b as u16).wrapping_sub(1 as u16);
            (CPUState { b: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'c' => {
            let answer: u16 = (cpu.c as u16).wrapping_sub(1 as u16);
            (CPUState { c: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'd' => {
            let answer: u16 = (cpu.d as u16).wrapping_sub(1 as u16);
            (CPUState { d: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'e' => {
            let answer: u16 = (cpu.e as u16).wrapping_sub(1 as u16);
            (CPUState { e: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'h' => {
            let answer: u16 = (cpu.h as u16).wrapping_sub(1 as u16);
            (CPUState { h: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        'l' => {
            let answer: u16 = (cpu.l as u16).wrapping_sub(1 as u16);
            (CPUState { l: answer.to_be_bytes()[0], ..cpu }, answer)
        }
        _ => (cpu, 0),
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

pub fn inx(cpu: CPUState, rp: (char, char)) -> CPUState {
    match rp {
        ('b', 'c') => {
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
        ('d', 'e') => {
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
        ('h', 'l') => {
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
        ('s', 'p') => CPUState {
            sp: cpu.sp.wrapping_add(1 as u16),
            cycles: 1,
            pc: cpu.pc + 1,
            ..cpu
        },
        _ => cpu,
    }
}

pub fn dcx(cpu: CPUState, rp: (char, char)) -> CPUState {
    match rp {
        ('b', 'c') => {
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
        ('d', 'e') => {
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
        ('h', 'l') => {
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
        ('s', 'p') => CPUState {
            sp: cpu.sp.wrapping_sub(1 as u16),
            pc: cpu.pc + 1,
            cycles: 1,
            ..cpu
        },
        _ => cpu,
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
