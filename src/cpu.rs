#![allow(dead_code)]
use crate::condition_codes::ConditionCodes;
use crate::helpers::is_even;

const MEMORY_SIZE: usize = 0x4000;

pub struct CPUState {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    cycles: u8,
    memory: [u8; MEMORY_SIZE],
    cc: ConditionCodes,
    int_enable: u8,
}

impl CPUState {
    fn new() -> CPUState {
        CPUState {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            cycles: 0,
            memory: [0; MEMORY_SIZE],
            cc: ConditionCodes::new(),
            int_enable: 0,
        }
    }
}

// This is dumb, i should use a HashMap and remove the order problem
fn arith_flags(answer: u16) -> (u8, u8, u8, u8) {
    let z = if answer & 0xff == 0 { 1 } else { 0 };
    let s = if answer & 0x80 != 0 { 1 } else { 0 };
    let cy = if answer > 0xff { 1 } else { 0 };
    let p = is_even(answer & 0xff);
    (z, s, cy, p)
}

fn mov_r_r(cpu: CPUState, r: char, value: u8) -> CPUState {
    match r {
        'a' => CPUState {
            a: value,
            cycles: 1,
            pc: cpu.pc + 1,
            ..cpu
        },
        'b' => CPUState {
            b: value,
            cycles: 1,
            pc: cpu.pc + 1,
            ..cpu
        },
        'c' => CPUState {
            c: value,
            cycles: 1,
            pc: cpu.pc + 1,
            ..cpu
        },
        'd' => CPUState {
            d: value,
            cycles: 1,
            pc: cpu.pc + 1,
            ..cpu
        },
        'e' => CPUState {
            e: value,
            cycles: 1,
            pc: cpu.pc + 1,
            ..cpu
        },
        'h' => CPUState {
            h: value,
            cycles: 1,
            pc: cpu.pc + 1,
            ..cpu
        },
        'l' => CPUState {
            l: value,
            cycles: 1,
            pc: cpu.pc + 1,
            ..cpu
        },
        _ => cpu,
    }
}

fn mov_r_m(cpu: CPUState, r: char) -> CPUState {
    let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    let value = cpu.memory[address as usize];
    match r {
        'a' => CPUState {
            a: value,
            cycles: 2,
            pc: cpu.pc + 1,
            ..cpu
        },
        'b' => CPUState {
            b: value,
            cycles: 2,
            pc: cpu.pc + 1,
            ..cpu
        },
        'c' => CPUState {
            c: value,
            cycles: 2,
            pc: cpu.pc + 1,
            ..cpu
        },
        'd' => CPUState {
            d: value,
            cycles: 2,
            pc: cpu.pc + 1,
            ..cpu
        },
        'e' => CPUState {
            e: value,
            cycles: 2,
            pc: cpu.pc + 1,
            ..cpu
        },
        'h' => CPUState {
            h: value,
            cycles: 2,
            pc: cpu.pc + 1,
            ..cpu
        },
        'l' => CPUState {
            l: value,
            cycles: 2,
            pc: cpu.pc + 1,
            ..cpu
        },
        _ => cpu,
    }
}

fn mov_m_r(cpu: CPUState, r: char) -> CPUState {
    let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    let mut memory = cpu.memory;
    match r {
        'a' => memory[address as usize] = cpu.a,
        'b' => memory[address as usize] = cpu.b,
        'c' => memory[address as usize] = cpu.c,
        'd' => memory[address as usize] = cpu.d,
        'e' => memory[address as usize] = cpu.e,
        'h' => memory[address as usize] = cpu.h,
        'l' => memory[address as usize] = cpu.l,
        _ => memory[address as usize] = memory[address as usize],
    }
    CPUState {
        memory,
        cycles: 2,
        pc: cpu.pc + 1,
        ..cpu
    }
}

fn mvi_r(cpu: CPUState, r: char) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = opcode[1];
    match r {
        'a' => CPUState {
            a: value,
            cycles: 2,
            pc: cpu.pc + 2,
            ..cpu
        },
        'b' => CPUState {
            b: value,
            cycles: 2,
            pc: cpu.pc + 2,
            ..cpu
        },
        'c' => CPUState {
            c: value,
            cycles: 2,
            pc: cpu.pc + 2,
            ..cpu
        },
        'd' => CPUState {
            d: value,
            cycles: 2,
            pc: cpu.pc + 2,
            ..cpu
        },
        'e' => CPUState {
            e: value,
            cycles: 2,
            pc: cpu.pc + 2,
            ..cpu
        },
        'h' => CPUState {
            h: value,
            cycles: 2,
            pc: cpu.pc + 2,
            ..cpu
        },
        'l' => CPUState {
            l: value,
            cycles: 2,
            pc: cpu.pc + 2,
            ..cpu
        },
        _ => cpu,
    }
}

fn mvi_m(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = opcode[1];
    let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    let mut memory = cpu.memory;
    memory[address as usize] = value;
    CPUState {
        memory,
        cycles: 3,
        pc: cpu.pc + 2,
        ..cpu
    }
}

fn lxi(cpu: CPUState, rs: (char, char)) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    match rs {
        ('b', 'c') => CPUState {
            b: opcode[2],
            c: opcode[1],
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
        ('d', 'e') => CPUState {
            d: opcode[2],
            e: opcode[1],
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
        ('h', 'l') => CPUState {
            h: opcode[2],
            l: opcode[1],
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
        ('s', 'p') => CPUState {
            sp: ((opcode[2] as u16) << 8 | opcode[1] as u16),
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
        _ => cpu,
    }
}

fn lda(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
    CPUState {
        a: cpu.memory[address as usize],
        cycles: 4,
        pc: cpu.pc + 3,
        ..cpu
    }
}

fn sta(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
    let mut memory = cpu.memory;
    memory[address as usize] = cpu.a;
    CPUState {
        memory,
        cycles: 4,
        pc: cpu.pc + 3,
        ..cpu
    }
}

fn lhld(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let address_l: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
    let address_h: u16 = address_l + 1;
    CPUState {
        h: cpu.memory[address_h as usize],
        l: cpu.memory[address_l as usize],
        cycles: 5,
        pc: cpu.pc + 3,
        ..cpu
    }
}

fn shld(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let address_l: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
    let address_h: u16 = address_l + 1;
    let mut memory = cpu.memory;
    memory[address_l as usize] = cpu.l;
    memory[address_h as usize] = cpu.h;
    CPUState {
        memory,
        cycles: 5,
        pc: cpu.pc + 3,
        ..cpu
    }
}

fn ldax(cpu: CPUState, rs: (char, char)) -> CPUState {
    let value: u8;
    match rs {
        ('b', 'c') => {
            let address: u16 = (cpu.b as u16) << 8 | cpu.c as u16;
            value = cpu.memory[address as usize];
        }
        ('d', 'e') => {
            let address: u16 = (cpu.d as u16) << 8 | cpu.e as u16;
            value = cpu.memory[address as usize];
        }
        _ => value = cpu.a,
    }
    CPUState {
        a: value,
        cycles: 2,
        pc: cpu.pc + 1,
        ..cpu
    }
}

fn stax(cpu: CPUState, rs: (char, char)) -> CPUState {
    let mut memory = cpu.memory;
    match rs {
        ('b', 'c') => {
            let address: u16 = (cpu.b as u16) << 8 | cpu.c as u16;
            memory[address as usize] = cpu.a;
        }
        ('d', 'e') => {
            let address: u16 = (cpu.d as u16) << 8 | cpu.e as u16;
            memory[address as usize] = cpu.a;
        }
        _ => (),
    }
    CPUState {
        memory,
        cycles: 2,
        pc: cpu.pc + 1,
        ..cpu
    }
}

fn xchg(cpu: CPUState) -> CPUState {
    CPUState {
        h: cpu.d,
        l: cpu.e,
        d: cpu.h,
        e: cpu.l,
        cycles: 1,
        pc: cpu.pc + 1,
        ..cpu
    }
}

fn add(cpu: CPUState, addendum: u8, cycles: u8) -> CPUState {
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

fn adi(cpu: CPUState, addendum: u8, cycles: u8) -> CPUState {
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

fn sub(cpu: CPUState, subtraend: u8, cycles: u8) -> CPUState {
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

fn sui(cpu: CPUState, subtraend: u8, cycles: u8) -> CPUState {
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

fn inr_r(cpu: CPUState, r: char) -> CPUState {
    match r {
        'a' => {
            let answer: u16 = (cpu.a as u16).wrapping_add(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                a: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'b' => {
            let answer: u16 = (cpu.b as u16).wrapping_add(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                b: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'c' => {
            let answer: u16 = (cpu.c as u16).wrapping_add(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                c: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'd' => {
            let answer: u16 = (cpu.d as u16).wrapping_add(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                d: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'e' => {
            let answer: u16 = (cpu.e as u16).wrapping_add(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                e: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'h' => {
            let answer: u16 = (cpu.h as u16).wrapping_add(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                h: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'l' => {
            let answer: u16 = (cpu.l as u16).wrapping_add(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                l: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        }
        _ => cpu
    }
}
fn inr_m(cpu: CPUState) -> CPUState {
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
        cc:flags,
        pc: cpu.pc + 3,
        cycles: 3,
        ..cpu
    }
}

fn dcr_m(cpu: CPUState) -> CPUState {
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
        cc:flags,
        pc: cpu.pc + 3,
        cycles: 3,
        ..cpu
    }
}

fn dcr_r(cpu: CPUState, r: char) -> CPUState {
    match r {
        'a' => {
            let answer: u16 = (cpu.a as u16).wrapping_sub(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                a: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'b' => {
            let answer: u16 = (cpu.b as u16).wrapping_sub(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                b: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'c' => {
            let answer: u16 = (cpu.c as u16).wrapping_sub(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                c: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'd' => {
            let answer: u16 = (cpu.d as u16).wrapping_sub(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                d: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'e' => {
            let answer: u16 = (cpu.e as u16).wrapping_sub(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                e: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'h' => {
            let answer: u16 = (cpu.h as u16).wrapping_sub(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                h: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        },
        'l' => {
            let answer: u16 = (cpu.l as u16).wrapping_sub(1 as u16);
            let cc = arith_flags(answer);
            let flags = ConditionCodes {
                z: cc.0,
                s: cc.1,
                p: cc.3,
                ..cpu.cc
            };
            CPUState {
                l: answer.to_be_bytes()[0],
                cc:flags,
                pc: cpu.pc + 1,
                ..cpu
            }

        }
        _ => cpu
    }
}

fn inx(cpu: CPUState, rp: (char, char)) -> CPUState {
    match rp {
        ('b', 'c') => {
            let result = (((cpu.b as u16) << 8 | cpu.c as u16).wrapping_add(1 as u16)).to_be_bytes();
            CPUState {
                b: result[0],
                c: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        ('d', 'e') => {
            let result = (((cpu.d as u16) << 8 | cpu.e as u16).wrapping_add(1 as u16)).to_be_bytes();
            CPUState {
                d: result[0],
                e: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        ('h', 'l') => {
            let result = (((cpu.h as u16) << 8 | cpu.l as u16).wrapping_add(1 as u16)).to_be_bytes();
            CPUState {
                h: result[0],
                l: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        ('s', 'p') => {
            CPUState {
                sp: cpu.sp.wrapping_add(1 as u16),
                cycles: 1,
                pc: cpu.pc + 1,
                ..cpu
            }
        }
        _ => cpu
    }
}

fn dcx(cpu: CPUState, rp: (char, char)) -> CPUState {
    match rp {
        ('b', 'c') => {
            let result = (((cpu.b as u16) << 8 | cpu.c as u16).wrapping_sub(1 as u16)).to_be_bytes();
            CPUState {
                b: result[0],
                c: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        ('d', 'e') => {
            let result = (((cpu.d as u16) << 8 | cpu.e as u16).wrapping_sub(1 as u16)).to_be_bytes();
            CPUState {
                d: result[0],
                e: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        ('h', 'l') => {
            let result = (((cpu.h as u16) << 8 | cpu.l as u16).wrapping_sub(1 as u16)).to_be_bytes();
            CPUState {
                h: result[0],
                l: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        ('s', 'p') => {
            CPUState {
                sp: cpu.sp.wrapping_sub(1 as u16),
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        _ => cpu
    }
}

fn dad(cpu: CPUState, rp: (char, char)) -> CPUState {
    let rp_hl: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    match rp {
        ('b', 'c') => {
            let result = (((cpu.b as u16) << 8 | cpu.c as u16).wrapping_add(rp_hl)).to_be_bytes();
            CPUState {
                h: result[0],
                l: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        ('d', 'e') => {
            let result = (((cpu.d as u16) << 8 | cpu.e as u16).wrapping_add(rp_hl)).to_be_bytes();
            CPUState {
                h: result[0],
                l: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        ('h', 'l') => {
            let result = (((cpu.h as u16) << 8 | cpu.l as u16).wrapping_add(rp_hl)).to_be_bytes();
            CPUState {
                h: result[0],
                l: result[1],
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        ('s', 'p') => {
            CPUState {
                sp: cpu.sp.wrapping_add(rp_hl),
                pc: cpu.pc + 1,
                cycles: 1,
                ..cpu
            }
        },
        _ => cpu
    }
}

fn ana(cpu: CPUState, value: u8, cycles: u8) -> CPUState {
    let answer = (cpu.a & value) as u16;
    let flags_result = arith_flags(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };
   
    CPUState {
        a: answer.to_be_bytes()[0],
        cycles,
        pc: cpu.pc + 1,
        cc: flags,
        ..cpu
    }
}

fn ani(cpu: CPUState, value: u8, cycles: u8) -> CPUState {
    let answer = (cpu.a & value) as u16;
    let flags_result = arith_flags(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[0],
        cycles,
        pc: cpu.pc + 2,
        cc: flags,
        ..cpu
    }
}

fn xra(cpu: CPUState, value: u8, cycles: u8) -> CPUState {
    let answer = (cpu.a ^ value) as u16;
    let flags_result = arith_flags(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[0],
        cycles,
        pc: cpu.pc + 1,
        cc: flags,
        ..cpu
    }
}

fn xri(cpu: CPUState, value: u8, cycles: u8) -> CPUState {
    let answer = (cpu.a ^ value) as u16;
    let flags_result = arith_flags(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[0],
        cycles,
        pc: cpu.pc + 2,
        cc: flags,
        ..cpu
    }
}

fn ora(cpu: CPUState, value: u8, cycles: u8) -> CPUState {
    let answer = (cpu.a | value) as u16;
    let flags_result = arith_flags(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[0],
        cycles,
        pc: cpu.pc + 1,
        cc: flags,
        ..cpu
    }
}

fn ori(cpu: CPUState, value: u8, cycles: u8) -> CPUState {
    let answer = (cpu.a | value) as u16;
    let flags_result = arith_flags(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[0],
        cycles,
        pc: cpu.pc + 2,
        cc: flags,
        ..cpu
    }
}

fn cmp(cpu: CPUState, value: u8, cycles: u8) -> CPUState {
    let answer = (cpu.a | value) as u16;
    let flags_result = arith_flags(answer);
    let flags = ConditionCodes {
        z: if answer.to_be_bytes()[1] == cpu.a { 1 } else { cpu.cc.z },
        s: flags_result.1,
        cy: if answer.to_be_bytes()[1] > cpu.a { 1 } else { cpu.cc.z },
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        cycles,
        pc: cpu.pc + 1,
        cc: flags,
        ..cpu
    }
}

fn cpi(cpu: CPUState, value: u8, cycles: u8) -> CPUState {
    let answer = (cpu.a | value) as u16;
    let flags_result = arith_flags(answer);
    let flags = ConditionCodes {
        z: if answer.to_be_bytes()[1] == cpu.a { 1 } else { cpu.cc.z },
        s: flags_result.1,
        cy: if answer.to_be_bytes()[1] > cpu.a { 1 } else { cpu.cc.z },
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        cycles,
        pc: cpu.pc + 2,
        cc: flags,
        ..cpu
    }
}

fn rlc(cpu: CPUState) -> CPUState {
    let answer = ((cpu.a & 0x80) >> 7) | (cpu.a << 1);
    let cy = if 0x80 == answer & 0x80 { 1 } else { 0 };
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: answer,
        cc: ConditionCodes { cy, ..cpu.cc},
        ..cpu
    }
}

fn rrc(cpu: CPUState) -> CPUState {
    let answer = ((cpu.a & 1) << 7) | (cpu.a >> 1);
    let cy = if 1 == answer & 1 { 1 } else { 0 };
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: answer,
        cc: ConditionCodes { cy, ..cpu.cc},
        ..cpu
    }
}

fn rar(cpu: CPUState) -> CPUState {
    let answer = (cpu.cc.cy << 7) | (cpu.a >> 1);
    let cy = if 1 == answer & 1 { 1 } else { 0 };
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: answer,
        cc: ConditionCodes { cy, ..cpu.cc},
        ..cpu
    }
}

fn ral(cpu: CPUState) -> CPUState {
    let answer = cpu.cc.cy | cpu.a << 1;
    let cy = if 0x80 == (answer & 0x80) { 1 } else { 0 };
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: answer,
        cc: ConditionCodes { cy, ..cpu.cc},
        ..cpu
    }
}

fn cma(cpu: CPUState) -> CPUState {
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: !a,
        ..cpu
    }
}

fn cmc(cpu: CPUState) -> CPUState {
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        cc: ConditionCodes { cy: !cy, ..cpu.cc},
        ..cpu
    }
}

fn stc(cpu: CPUState) -> CPUState {
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        cc: ConditionCodes { cy: 1, ..cpu.cc},
        ..cpu
    }
}

// fn xor(value: u8) {
//     let answer = self.a ^ value;
//     self.arith_flags(answer as u16, false);
//     self.cc.cy = 0;
// }

// fn or(value: u8) {
//     let answer = self.a | value;
//     self.arith_flags(answer as u16, false);
//     self.cc.cy = 0;
// }

// fn cmp(value: u8) {
//     let answer = (self.a).wrapping_sub(value);
//     self.arith_flags(answer as u16, false);
//     self.cc.z = if answer == 0 { 1 } { self.cc.z };
//     self.cc.cy = if self.a < value { 1 } { self.cc.cy };
// }

// fn unimplemented_instruction(&self) {
//     panic!("Error: Unimplemented instruction\n")
// }

fn emulate_8080_op(cpu: CPUState) -> CPUState {
    let pc: usize = cpu.pc as usize;
    let opcode = &cpu.memory[pc..];
    match opcode[0] {
//         0x00 => (),

        // LXI OPS
        0x01 => {
            lxi(cpu, ('b', 'c'))
        }
        0x11 => {
            lxi(cpu, ('d', 'e'))
        }
        0x21 => {
            lxi(cpu, ('h', 'l'))
        }
        0x31 => {
            lxi(cpu, ('s', 'p'))
        }

        // INX OPS
        0x03 => { // INX B
            inx(cpu, ('b', 'c'))
        }
        0x13 => { // INX D
            inx(cpu, ('d', 'e'))
        }
        0x23 => { // INX H
            inx(cpu, ('h', 'l'))
        }
        0x33 => { // INX SP
            inx(cpu, ('s', 'p'))
        }

        // DCX OPS
        0x0b => { // DCX B
            dcx(cpu, ('b', 'c'))
        }
        0x1b => { // DCX D
            dcx(cpu, ('d', 'e'))
        }
        0x2b => { // DCX H
            dcx(cpu, ('h', 'l'))
        }
        0x3b => { // DCX SP
            dcx(cpu, ('s', 'p'))
        }

        //DAD OPS
        0x09 => {
            dad(cpu, ('b', 'c'))
        }
        0x19 => {
            dad(cpu, ('d', 'e'))
        }
        0x29 => {
            dad(cpu, ('h', 'l'))
        }
        0x39 => {
            dad(cpu, ('s', 'p'))
        }

        // INR OPS
        0x04 => {
            inr_r(cpu, 'b')
        }
        0x0c => {
            inr_r(cpu, 'c')
        }
        0x14 => {
            inr_r(cpu, 'd')
        }
        0x1c => {
            inr_r(cpu, 'e')
        }
        0x24 => {
            inr_r(cpu, 'h')
        }
        0x2c => {
            inr_r(cpu, 'l')
        }
        0x34 => {
            inr_m(cpu)
        }

        // DCR OPS
        0x05 => {
            dcr_r(cpu, 'b')
        }
        0x0d => {
            dcr_r(cpu, 'c')
        }
        0x15 => {
            dcr_r(cpu, 'd')
        }
        0x1d => {
            dcr_r(cpu, 'e')
        }
        0x25 => {
            dcr_r(cpu, 'h')
        }
        0x2d => {
            dcr_r(cpu, 'l')
        }
        0x35 => {
            dcr_m(cpu)
        }

        // MVI
        0x06 => {
            mvi_r(cpu, 'b')
        }
        0x0e => {
            mvi_r(cpu, 'c')
        }
        0x16 => {
            mvi_r(cpu, 'd')
        }
        0x1e => {
            mvi_r(cpu, 'e')
        }
        0x26 => {
            mvi_r(cpu, 'h')
        }
        0x2e => {
            mvi_r(cpu, 'l')
        }
        0x36 => {
            mvi_m(cpu)
        }
        0x3e => {
            mvi_r(cpu, 'a')
        }

        // STAX OPS
        0x02 => {
            stax(cpu, ('b', 'c'))
        }
        0x12 => {
            stax(cpu, ('d', 'e'))
        }

        // LDAX
        0x0a => {
            ldax(cpu, ('b', 'c'))
        }
        0x1a => {
            ldax(cpu, ('d', 'e'))
        }

        // STA
        0x32 => {
            sta(cpu)
        }

        // LDA
        0x3a => {
            lda(cpu)
        }

        // SHLD
        0x22 => {
            shld(cpu)
        }

        // LHLD
        0x2a => {
            lhld(cpu)
        }

        // MOV OPS
        0x41 => {
            let value = cpu.c;
            mov_r_r(cpu, 'b', value)
        }
        0x42 => {
            let value = cpu.d;
            mov_r_r(cpu, 'b', value)
        }
        0x43 => {
            let value = cpu.e;
            mov_r_r(cpu, 'b', value)
        }
        0x44 => {
            let value = cpu.h;
            mov_r_r(cpu, 'b', value)
        }
        0x45 => {
            let value = cpu.l;
            mov_r_r(cpu, 'b', value)
        }
        0x46 => {
            mov_r_m(cpu, 'b')
        }
        0x47 => {
            let value = cpu.a;
            mov_r_r(cpu, 'a', value)
        }
        0x48 => {
            let value = cpu.b;
            mov_r_r(cpu, 'c', value)
        }
        0x4a => {
            let value = cpu.d;
            mov_r_r(cpu, 'c', value)
        }
        0x4b => {
            let value = cpu.e;
            mov_r_r(cpu, 'c', value)
        }
        0x4c => {
            let value = cpu.h;
            mov_r_r(cpu, 'c', value)
        }
        0x4d => {
            let value = cpu.l;
            mov_r_r(cpu, 'c', value)
        }
        0x4e => {
            mov_r_m(cpu, 'c')
        }
        0x4f => {
            let value = cpu.a;
            mov_r_r(cpu, 'c', value)
        }
        0x50 => {
            let value = cpu.b;
            mov_r_r(cpu, 'd', value)
        }
        0x51 => {
            let value = cpu.c;
            mov_r_r(cpu, 'd', value)
        }
        0x53 => {
            let value = cpu.e;
            mov_r_r(cpu, 'd', value)
        }
        0x54 => {
            let value = cpu.h;
            mov_r_r(cpu, 'd', value)
        }
        0x55 => {
            let value = cpu.l;
            mov_r_r(cpu, 'd', value)
        }
        0x56 => {
            mov_r_m(cpu, 'd')
        }
        0x57 => {
            let value = cpu.a;
            mov_r_r(cpu, 'd', value)
        }
        0x58 => {
            let value = cpu.b;
            mov_r_r(cpu, 'e', value)
        }
        0x59 => {
            let value = cpu.c;
            mov_r_r(cpu, 'e', value)
        }
        0x5a => {
            let value = cpu.d;
            mov_r_r(cpu, 'e', value)
        }
        0x5c => {
            let value = cpu.h;
            mov_r_r(cpu, 'e', value)
        }
        0x5d => {
            let value = cpu.l;
            mov_r_r(cpu, 'e', value)
        }
        0x5e => {
            mov_r_m(cpu, 'e')
        }
        0x5f => {
            let value = cpu.a;
            mov_r_r(cpu, 'e', value)
        }
        0x60 => {
            let value = cpu.b;
            mov_r_r(cpu, 'h', value)
        }
        0x61 => {
            let value = cpu.c;
            mov_r_r(cpu, 'h', value)
        }
        0x62 => {
            let value = cpu.d;
            mov_r_r(cpu, 'h', value)
        }
        0x63 => {
            let value = cpu.e;
            mov_r_r(cpu, 'h', value)
        }
        0x65 => {
            let value = cpu.l;
            mov_r_r(cpu, 'h', value)
        }
        0x66 => {
            mov_r_m(cpu, 'h')
        }
        0x67 => {
            let value = cpu.a;
            mov_r_r(cpu, 'h', value)
         }
        0x68 => {
            let value = cpu.b;
            mov_r_r(cpu, 'l', value)
        }
        0x69 => {
            let value = cpu.c;
            mov_r_r(cpu, 'l', value)
        }
        0x6a => {
            let value = cpu.d;
            mov_r_r(cpu, 'l', value)
        }
        0x6b => {
            let value = cpu.e;
            mov_r_r(cpu, 'l', value)
        }
        0x6c => {
            let value = cpu.h;
            mov_r_r(cpu, 'l', value)
        }
        0x6e => {
            mov_r_m(cpu, 'l')
        }
        0x6f => {
            let value = cpu.a;
            mov_r_r(cpu, 'l', value)
        }

        0x70 => {
            mov_m_r(cpu, 'b')
        }
        0x71 => {
            mov_m_r(cpu, 'c')
        }
        0x72 => {
            mov_m_r(cpu, 'd')
        }
        0x73 => {
            mov_m_r(cpu, 'e')
        }
        0x74 => {
            mov_m_r(cpu, 'h')
        }
        0x75 => {
            mov_m_r(cpu, 'l')
        }
        0x76 => cpu,//TODO
        0x77 => {
            mov_m_r(cpu, 'a')
        }

        0x78 => {
            let value = cpu.b;
            mov_r_r(cpu, 'a', value)
        }
        0x79 => {
            let value = cpu.c;
            mov_r_r(cpu, 'a', value)
        }
        0x7a => {
            let value = cpu.d;
            mov_r_r(cpu, 'a', value)
        }
        0x7b => {
            let value = cpu.e;
            mov_r_r(cpu, 'a', value)
        }
        0x7c => {
            let value = cpu.h;
            mov_r_r(cpu, 'a', value)
        }
        0x7d => {
            let value = cpu.l;
            mov_r_r(cpu, 'a', value)
        }
        0x7e => {
            mov_r_m(cpu, 'a')
        }
        // ADD OPS
        0x80 => {
            let addendum = cpu.b;
            add(cpu, addendum, 1)
        }
        0x81 => {
            let addendum = cpu.c;
            add(cpu, addendum, 1)
        }
        0x82 => {
            let addendum = cpu.d;
            add(cpu, addendum, 1)
        }
        0x83 => {
            let addendum = cpu.e;
            add(cpu, addendum, 1)
        }
        0x84 => {
            let addendum = cpu.h;
            add(cpu, addendum, 1)
        }
        0x85 => {
            let addendum = cpu.l;
            add(cpu, addendum, 1)
        }
        0x86 => {
            let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
            let addendum = cpu.memory[address as usize];
            add(cpu, addendum, 2)
        }
        0x87 => {
            let addendum = cpu.a;
            add(cpu, addendum, 1)
        }

        // ADC OPS
        0x88 => {
            let addendum = (cpu.b).wrapping_add(cpu.cc.cy);
            add(cpu, addendum, 2)
        }
        0x89 => {
            let addendum = (cpu.c).wrapping_add(cpu.cc.cy);
            add(cpu, addendum, 2)
        }
        0x8a => {
            let addendum = (cpu.d).wrapping_add(cpu.cc.cy);
            add(cpu, addendum, 2)
        }
        0x8b => {
            let addendum = (cpu.e).wrapping_add(cpu.cc.cy);
            add(cpu, addendum, 2)
        }
        0x8c => {
            let addendum = (cpu.h).wrapping_add(cpu.cc.cy);
            add(cpu, addendum, 2)
        }
        0x8d => {
            let addendum = (cpu.l).wrapping_add(cpu.cc.cy);
            add(cpu, addendum, 2)
        }
        0x8e => {
            let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
            let addendum = (cpu.memory[address as usize]).wrapping_add(cpu.cc.cy);
            add(cpu, addendum, 2)
        }
        0x8f => {
            let addendum = (cpu.a).wrapping_add(cpu.cc.cy);
            add(cpu, addendum, 2)
        }

        // SUB OPS
        0x90 => {
            let subtraend = cpu.b;
            sub(cpu, subtraend, 2)
        }
        0x91 => {
            let subtraend = cpu.c;
            sub(cpu, subtraend, 2)
        }
        0x92 => {
            let subtraend = cpu.d;
            sub(cpu, subtraend, 2)
        }
        0x93 => {
            let subtraend = cpu.e;
            sub(cpu, subtraend, 2)
        }
        0x94 => {
            let subtraend = cpu.h;
            sub(cpu, subtraend, 2)
        }
        0x95 => {
            let subtraend = cpu.l;
            sub(cpu, subtraend, 2)
        }
        0x96 => {
            let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
            let subtraend = cpu.memory[address as usize];
            sub(cpu, subtraend, 2)
        }
        0x97 => {
            let subtraend = cpu.a;
            sub(cpu, subtraend, 2)
        }

        // SUBB OPS
        0x98 => {
            let subtraend = (cpu.b).wrapping_sub(cpu.cc.cy);
            sub(cpu, subtraend, 1)
        }
        0x99 => {
            let subtraend = (cpu.c).wrapping_sub(cpu.cc.cy);
            sub(cpu, subtraend, 1)
        }
        0x9a => {
            let subtraend = (cpu.d).wrapping_sub(cpu.cc.cy);
            sub(cpu, subtraend, 1)
        }
        0x9b => {
            let subtraend = (cpu.e).wrapping_sub(cpu.cc.cy);
            sub(cpu, subtraend, 1)
        }
        0x9c => {
            let subtraend = (cpu.h).wrapping_sub(cpu.cc.cy);
            sub(cpu, subtraend, 1)
        }
        0x9d => {
            let subtraend = (cpu.l).wrapping_sub(cpu.cc.cy);
            sub(cpu, subtraend, 1)
        }
        0x9e => {
            let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
            let subtraend = (cpu.memory[address as usize]).wrapping_sub(cpu.cc.cy);
            sub(cpu, subtraend, 1)
        }
        0x9f => {
            let subtraend = (cpu.a).wrapping_sub(cpu.cc.cy);
            sub(cpu, subtraend, 1)
        }

        // ADI OPS
        0xc6 => {
            let first_byte: u8 = opcode[1];
            adi(cpu, first_byte, 2)
        }
        0xce => {
            let first_byte: u8 = opcode[1];
            let cy: u8 = cpu.cc.cy;
            adi(cpu, (first_byte).wrapping_add(cy), 2)
        }
        0xeb => {
            xchg(cpu)
        }

        // SUI OPS
        0xd6 => {
            let subtraend: u8 = opcode[1];
            sui(cpu, subtraend, 2)
        }
        0xde => {
            let cy = cpu.cc.cy;
            let subtraend: u8 = opcode[1].wrapping_sub(cy);
            sui(cpu, subtraend, 2)
        }
         _ => cpu,
    }
}
