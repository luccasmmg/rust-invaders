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

fn inr_r(cpu: CPUState, r: char, cycles: u8) -> CPUState {
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
                cycles,
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
                cycles,
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
                cycles,
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
                cycles,
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
                cycles,
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
                cycles,
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
                cycles,
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

fn dcr_r(cpu: CPUState, r: char, cycles: u8) -> CPUState {
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
                cycles,
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
                cycles,
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
                cycles,
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
                cycles,
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
                cycles,
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
                cycles,
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
                cycles,
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
        _ => cpu
    }
}




// fn dcr(register: u8) -> u8 {
//     let answer: u16 = (register as u16).wrapping_sub(1 as u16);
//     self.arith_flags(answer, true);
//     answer as u8
// }

// fn inx(&self, rh: u8, rl: u8) -> (u8, u8) {
//     let result = (((rh as u16) << 8 | rl as u16).wrapping_add(1 as u16)).to_be_bytes();
//     (result[0], result[1])
// }

// fn dcx(&self, rh: u8, rl: u8) -> (u8, u8) {
//     let result = (((rh as u16) << 8 | rl as u16).wrapping_sub(1 as u16)).to_be_bytes();
//     (result[0], result[1])
// }

// fn dad(rh: u8, rl: u8) {
//     let hl = (self.h as u16) << 8 | self.l as u16;
//     let register_sum = (rh as u16) << 8 | rl as u16;
//     let answer = hl + register_sum;
//     self.h = answer.to_be_bytes()[0];
//     self.l = answer.to_be_bytes()[0];
//     self.cc.cy = if (answer & 0xff) != 0 { 1 } else { 0 };
// }

// fn and(value: u8) {
//     let answer = self.a & value;
//     self.arith_flags(answer as u16, false);
//     self.cc.cy = 0;
// }

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

//         // LXI OPS
//         0x01 => {
//             self.c = opcode[1];
//             self.b = opcode[2];
//             self.pc += 2
//         }
//         0x11 => {
//             self.d = opcode[1];
//             self.e = opcode[2];
//             self.pc += 2;
//         }
//         0x21 => {
//             self.h = opcode[1];
//             self.l = opcode[2];
//             self.pc += 2;
//         }
//         0x31 => {
//             let value: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
//             self.sp = value;
//             self.pc += 2;
//         }

//         // INX OPS
//         0x03 => { // INX B
//             let result = self.inx(self.b, self.c);
//             self.b = result.0;
//             self.c = result.1;
//         }
//         0x13 => { // INX D
//             let result = self.inx(self.d, self.e);
//             self.d = result.0;
//             self.e = result.1;
//         }
//         0x23 => { // INX H
//             let result = self.inx(self.h, self.l);
//             self.h = result.0;
//             self.l = result.1;
//         }
//         0x33 => { // INX SP
//             let divided_sp = self.pc.to_be_bytes();
//             let result = self.inx(divided_sp[0], divided_sp[1]);
//             self.pc = (result.0 as u16) << 8 | result.1 as u16;
//         }

//         // DCX OPS
//         0x0b => { // DCX B
//             let result = self.dcx(self.b, self.c);
//             self.b = result.0;
//             self.c = result.1;
//         }
//         0x1b => { // DCX D
//             let result = self.dcx(self.d, self.e);
//             self.d = result.0;
//             self.e = result.1;
//         }
//         0x2b => { // DCX H
//             let result = self.dcx(self.h, self.l);
//             self.h = result.0;
//             self.l = result.1;
//         }
//         0x3b => { // DCX SP
//             let divided_sp = self.pc.to_be_bytes();
//             let result = self.dcx(divided_sp[0], divided_sp[1]);
//             self.pc = (result.0 as u16) << 8 | result.1 as u16;
//         }

//         //DAD OPS
//         0x09 => {
//             self.dad(self.b, self.c);
//         }
//         0x19 => {
//             self.dad(self.d, self.e);
//         }
//         0x29 => {
//             self.dad(self.h, self.l);
//         }
//         0x39 => {
//             let divided_sp = self.sp.to_be_bytes();
//             self.dad(divided_sp[0], divided_sp[1]);
//         }

//         // INR OPS
//         0x04 => {
//             self.b = self.inr(self.b);
//         }
//         0x0c => {
//             self.c = self.inr(self.c);
//         }
//         0x14 => {
//             self.d = self.inr(self.d);
//         }
//         0x1c => {
//             self.e = self.inr(self.e);
//         }
//         0x24 => {
//             self.h = self.inr(self.h);
//         }
//         0x2c => {
//             self.l = self.inr(self.l);
//         }
//         0x34 => {
//             let subress: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[subress as usize] = self.inr(self.memory[address as usize]);
//         }

//         // DCR OPS
//         0x05 => {
//             self.b = self.dcr(self.b);
//         }
//         0x0d => {
//             self.c = self.dcr(self.c);
//         }
//         0x15 => {
//             self.d = self.dcr(self.d);
//         }
//         0x1d => {
//             self.e = self.dcr(self.e);
//         }
//         0x25 => {
//             self.h = self.dcr(self.h);
//         }
//         0x2d => {
//             self.l = self.dcr(self.l);
//         }
//         0x35 => {
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[address as usize] = self.dcr(self.memory[address as usize]);
//         }

//         // MVI
//         0x06 => {
//             self.b = opcode[1];
//             self.pc += 1;
//         }
//         0x0e => {
//             self.c = opcode[1];
//             self.pc += 1;
//         }
//         0x16 => {
//             self.d = opcode[1];
//             self.pc += 1;
//         }
//         0x1e => {
//             self.e = opcode[1];
//             self.pc += 1;
//         }
//         0x26 => {
//             self.h = opcode[1];
//             self.pc += 1;
//         }
//         0x2e => {
//             self.l = opcode[1];
//             self.pc += 1;
//         }
//         0x36 => {
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[address as usize] = opcode[1];
//             self.pc += 1;
//         }
//         0x3e => {
//             self.a = opcode[1];
//             self.pc += 1;
//         }

//         // STAX OPS
//         0x02 => {
//             let address: u16 = (self.b as u16) << 8 | self.c as u16;
//             self.memory[address as usize] = self.a;
//         }
//         0x12 => {
//             let address: u16 = (self.d as u16) << 8 | self.e as u16;
//             self.memory[address as usize] = self.a;
//         }

//         // LDAX
//         0x0a => {
//             let address: u16 = (self.b as u16) << 8 | self.c as u16;
//             self.a = self.memory[address as usize];
//         }
//         0x1a => {
//             let address: u16 = (self.d as u16) << 8 | self.e as u16;
//             self.a = self.memory[address as usize];
//         }

//         // STA
//         0x32 => {
//             let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
//             self.memory[address as usize] = self.a;
//         }

//         // LDA
//         0x3a => {
//             let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
//             self.a = self.memory[address as usize];
//         }

//         // SHLD
//         0x22 => {
//             let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
//             self.memory[address as usize] = self.l;
//             self.memory[(address + 1) as usize] = self.l;
//             self.pc += 2;
//         }

//         // LHLD
//         0x2a => {
//             let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
//             self.l = self.memory[address as usize];
//             self.h = self.memory[(address + 1) as usize];
//             self.pc += 2;
//         }

//         // MOV OPS
//         0x40 => (), // MOV B,B
//         0x41 => {
//             self.b = self.c; // MOV B,B
//         }
//         0x42 => {
//             self.b = self.d; // MOV B,D
//         }
//         0x43 => {
//             self.b = self.e; // MOV B,E
//         }
//         0x44 => {
//             self.b = self.h; // MOV B,H
//         }
//         0x45 => {
//             self.b = self.l; // MOV B,L
//         }
//         0x46 => { // MOV B,M
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.b = self.memory[address as usize];
//         }
//         0x47 => { // MOV B,A
//             self.b = self.a;
//         }
//         0x48 => { // MOV C B
//             self.c = self.b;
//         }
//         0x4a => { // MOV C D
//             self.c = self.d;
//         }
//         0x4b => { // MOV C E
//             self.c = self.e;
//         }
//         0x4c => { // MOV C H
//             self.c = self.h;
//         }
//         0x4d => { // MOV C L
//             self.c = self.l;
//         }
//         0x4e => { // MOV C M
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.c = self.memory[address as usize];
//         }
//         0x4f => { // MOV C A
//             self.c = self.a;
//         }
//         0x50 => { // MOV D B
//             self.d = self.b;
//         }
//         0x51 => { // MOV D C
//             self.d = self.c;
//         }
//         0x53 => { // MOV D E
//             self.d = self.e;
//         }
//         0x54 => { // MOV D H
//             self.d = self.h;
//         }
//         0x55 => { // MOV D L
//             self.d = self.l;
//         }
//         0x56 => { // MOV D M
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.d = self.memory[address as usize];
//         }
//         0x57 => { // MOV D A
//             self.d = self.a;
//         }
//         0x58 => { // MOV E B
//             self.e = self.b;
//         }
//         0x59 => { // MOV E C
//             self.e = self.c;
//         }
//         0x5a => { // MOV E D
//             self.e = self.d;
//         }
//         0x5c => { // MOV E H
//             self.e = self.h;
//         }
//         0x5d => { // MOV E L
//             self.e = self.l;
//         }
//         0x5e => { // MOV E M
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.e = self.memory[address as usize];
//         }
//         0x5f => { // MOV E A
//             self.e = self.a;
//         }
//         0x60 => { // MOV H B
//             self.h = self.b;
//         }
//         0x61 => { // MOV H C
//             self.h = self.c;
//         }
//         0x62 => { // MOV H D
//             self.h = self.d;
//         }
//         0x63 => { // MOV H E
//             self.h = self.e;
//         }
//         0x65 => { // MOV H L
//             self.h = self.l;
//         }
//         0x66 => { // MOV H M
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.h = self.memory[address as usize];
//         }
//         0x67 => { // MOV H A
//             self.h = self.a;
//         }
//         0x68 => { // MOV L B
//             self.l = self.b;
//         }
//         0x69 => { // MOV L C
//             self.l = self.c;
//         }
//         0x6a => { // MOV L D
//             self.l = self.d;
//         }
//         0x6b => { // MOV L E
//             self.l = self.e;
//         }
//         0x6c => { // MOV L H
//             self.l = self.h;
//         }
//         0x6e => { // MOV L M
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.l = self.memory[address as usize];
//         }
//         0x6f => { // MOV L A
//             self.l = self.a;
//         }
//         0x70 => { // MOV M B
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[address as usize] = self.b;
//         }
//         0x71 => { // MOV M C
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[address as usize] = self.c;
//         }
//         0x72 => { // MOV M D
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[address as usize] = self.d;
//         }
//         0x73 => { // MOV M E
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[address as usize] = self.e;
//         }
//         0x74 => { // MOV M H
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[address as usize] = self.h;
//         }
//         0x75 => { // MOV M L
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[address as usize] = self.l;
//         }
//         0x76 => (), //TODO
//         0x77 => { // MOV M A
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.memory[address as usize] = self.a;
//         }
//         0x78 => { // MOV A B
//             self.a = self.b;
//         }
//         0x79 => { // MOV A C
//             self.a = self.c;
//         }
//         0x7a => { // MOV A D
//             self.a = self.d;
//         }
//         0x7b => { // MOV A E
//             self.a = self.e;
//         }
//         0x7c => { // MOV A H
//             self.a = self.h;
//         }
//         0x7d => { // MOV A L
//             self.a = self.l;
//         }
//         0x7e => { // MOV A M
//             let address: u16 = (self.h as u16) << 8 | self.l as u16;
//             self.a = self.memory[address as usize];
//         }
//         0x7f => (),

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
            add(cpu, first_byte, 2)
        }
        0xce => {
            let first_byte: u8 = opcode[1];
            let cy: u8 = cpu.cc.cy;
            add(cpu, (first_byte).wrapping_add(cy), 2)
        }
        0xeb => {
            xchg(cpu)
        }

        // SUI OPS
        0xd6 => {
            let subtraend: u8 = opcode[1];
            sub(cpu, subtraend, 2)
        }
        0xde => {
            let cy = cpu.cc.cy;
            let subtraend: u8 = opcode[1].wrapping_sub(cy);
            sub(cpu, subtraend, 2)
        }
         _ => cpu,
    }
}
