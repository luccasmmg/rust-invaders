#![allow(dead_code)]
use crate::cpu::CPUState;

pub fn jmp(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    CPUState {
        cycles: 3,
        pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
        ..cpu
    }
}

pub fn jnz(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    println!("JNZ");
    let value = if cpu.cc.z == 0 {
        (opcode_2 as u16) << 8 | opcode_1 as u16
    } else {
        cpu.pc + 3
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jz(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let value = if cpu.cc.z == 1 {
        (opcode_2 as u16) << 8 | opcode_1 as u16
    } else {
        cpu.pc + 3
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jnc(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.cy == 0 {
        (opcode_2 as u16) << 8 | opcode_1 as u16
    } else {
        cpu.pc + 3
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jc(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.cy == 1 {
        (opcode_2 as u16) << 8 | opcode_1 as u16
    } else {
        cpu.pc + 3
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jpo(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.p == 0 {
        (opcode_2 as u16) << 8 | opcode_1 as u16
    } else {
        cpu.pc + 3
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jpe(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.p == 1 {
        (opcode_2 as u16) << 8 | opcode_1 as u16
    } else {
        cpu.pc + 3
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jp(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.s == 0 {
        (opcode_2 as u16) << 8 | opcode_1 as u16
    } else {
        cpu.pc + 3
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jm(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.s == 1 {
        (opcode_2 as u16) << 8 | opcode_1 as u16
    } else {
        cpu.pc + 3
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn call(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let pc = (cpu.pc + 3).to_be_bytes();
    let mut memory = cpu.memory;
    memory[cpu.sp as usize - 1] = pc[0];
    memory[cpu.sp as usize - 2] = pc[1];
    CPUState {
        cycles: 5,
        pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
        sp: cpu.sp - 2,
        memory,
        ..cpu
    }
}

pub fn cc(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    match cpu.cc.cy {
        1 => {
            let pc = (cpu.pc + 3).to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
    }
}

pub fn cnc(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    match cpu.cc.cy {
        0 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = (cpu.pc + 3).to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
    }
}

pub fn cz(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    match cpu.cc.z {
        1 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = (cpu.pc + 3).to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
    }
}

pub fn cnz(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    match cpu.cc.z {
        0 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = (cpu.pc + 3).to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
    }
}

pub fn cp(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    match cpu.cc.s {
        0 => {
            let pc = (cpu.pc + 3).to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
    }
}

pub fn cm(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    match cpu.cc.s {
        1 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = (cpu.pc + 3).to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
    }
}

pub fn cpe(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    match cpu.cc.p {
        1 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = (cpu.pc + 3).to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
    }
}

pub fn cpo(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    match cpu.cc.p {
        0 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = (cpu.pc + 3).to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode_2 as u16) << 8 | opcode_1 as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
    }
}

pub fn ret(cpu: CPUState) -> CPUState {
    CPUState {
        cycles: 5,
        pc: cpu.memory[cpu.sp as usize] as u16 |( cpu.memory[cpu.sp as usize + 1] as u16) << 8,
        sp: cpu.sp + 2,
        ..cpu
    }
}

pub fn rc(cpu: CPUState) -> CPUState {
    match cpu.cc.cy {
        1 => CPUState {
            cycles: 5,
            pc: cpu.memory[cpu.sp as usize] as u16 | (cpu.memory[cpu.sp as usize + 1] as u16) << 8,
            ..cpu
        },
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 1,
            ..cpu
        },
    }
}

pub fn rnc(cpu: CPUState) -> CPUState {
    match cpu.cc.cy {
        0 => CPUState {
            cycles: 5,
            pc: cpu.memory[cpu.sp as usize] as u16 | (cpu.memory[cpu.sp as usize + 1] as u16) << 8,
            ..cpu
        },
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 1,
            ..cpu
        },
    }
}

pub fn rz(cpu: CPUState) -> CPUState {
    match cpu.cc.z {
        1 => CPUState {
            cycles: 5,
            pc: cpu.memory[cpu.sp as usize] as u16 | (cpu.memory[cpu.sp as usize + 1] as u16) << 8,
            ..cpu
        },
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 1,
            ..cpu
        },
    }
}

pub fn rnz(cpu: CPUState) -> CPUState {
    match cpu.cc.z {
        0 => CPUState {
            cycles: 5,
            pc: cpu.memory[cpu.sp as usize] as u16 | (cpu.memory[cpu.sp as usize + 1] as u16) << 8,
            ..cpu
        },
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 1,
            ..cpu
        },
    }
}

pub fn rp(cpu: CPUState) -> CPUState {
    match cpu.cc.s {
        1 => CPUState {
            cycles: 5,
            pc: cpu.memory[cpu.sp as usize] as u16 | (cpu.memory[cpu.sp as usize + 1] as u16) << 8,
            ..cpu
        },
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 1,
            ..cpu
        },
    }
}

pub fn rm(cpu: CPUState) -> CPUState {
    match cpu.cc.s {
        0 => CPUState {
            cycles: 5,
            pc: cpu.memory[cpu.sp as usize] as u16 | (cpu.memory[cpu.sp as usize + 1] as u16) << 8,
            ..cpu
        },
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 1,
            ..cpu
        },
    }
}

pub fn rpe(cpu: CPUState) -> CPUState {
    match cpu.cc.p {
        1 => CPUState {
            cycles: 5,
            pc: cpu.memory[cpu.sp as usize] as u16 | (cpu.memory[cpu.sp as usize + 1] as u16) << 8,
            ..cpu
        },
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 1,
            ..cpu
        },
    }
}

pub fn rpo(cpu: CPUState) -> CPUState {
    match cpu.cc.p {
        0 => CPUState {
            cycles: 5,
            pc: cpu.memory[cpu.sp as usize] as u16 | (cpu.memory[cpu.sp as usize + 1] as u16) << 8,
            ..cpu
        },
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 1,
            ..cpu
        },
    }
}

pub fn pchl(cpu: CPUState) -> CPUState {
    CPUState {
        cycles: 1,
        pc: (cpu.h as u16) << 8 | cpu.l as u16,
        ..cpu
    }
}

pub fn rst(cpu: CPUState, n: u8) -> CPUState {
    let pc = cpu.pc.to_be_bytes();
    let mut memory = cpu.memory;
    memory[cpu.sp as usize - 1] = pc[0];
    memory[cpu.sp as usize - 2] = pc[1];
    match n {
        0..=7 => CPUState {
            cycles: 3,
            pc: (8 * n) as u16,
            sp: cpu.sp - 2,
            memory,
            ..cpu
        },
        _ => cpu,
    }
}
