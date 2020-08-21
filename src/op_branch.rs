#![allow(dead_code)]
use crate::cpu::CPUState;

pub fn jmp(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    CPUState {
        cycles: 3,
        pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
        ..cpu
    }
}

pub fn jnz(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.z == 0 {
        (opcode[2] as u16) << 8 | opcode[1] as u16
    } else {
        cpu.pc + 2
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jz(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.z == 1 {
        (opcode[2] as u16) << 8 | opcode[1] as u16
    } else {
        cpu.pc + 2
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jnc(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.cy == 0 {
        (opcode[2] as u16) << 8 | opcode[1] as u16
    } else {
        cpu.pc + 2
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jc(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.cy == 1 {
        (opcode[2] as u16) << 8 | opcode[1] as u16
    } else {
        cpu.pc + 2
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jpo(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.p == 0 {
        (opcode[2] as u16) << 8 | opcode[1] as u16
    } else {
        cpu.pc + 2
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jpe(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.p == 1 {
        (opcode[2] as u16) << 8 | opcode[1] as u16
    } else {
        cpu.pc + 2
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jp(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.s == 0 {
        (opcode[2] as u16) << 8 | opcode[1] as u16
    } else {
        cpu.pc + 2
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn jm(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let value = if cpu.cc.s == 1 {
        (opcode[2] as u16) << 8 | opcode[1] as u16
    } else {
        cpu.pc + 2
    };
    CPUState {
        cycles: 3,
        pc: value,
        ..cpu
    }
}

pub fn call(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let pc = cpu.pc.to_be_bytes();
    let mut memory = cpu.memory;
    memory[cpu.sp as usize - 1] = pc[0];
    memory[cpu.sp as usize - 2] = pc[1];
    CPUState {
        cycles: 5,
        pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
        sp: cpu.sp - 2,
        memory,
        ..cpu
    }
}

pub fn cc(cpu: CPUState) -> CPUState {
    match cpu.cc.cy {
        1 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = cpu.pc.to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 2,
            ..cpu
        },
    }
}

pub fn cnc(cpu: CPUState) -> CPUState {
    match cpu.cc.cy {
        0 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = cpu.pc.to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 2,
            ..cpu
        },
    }
}

pub fn cz(cpu: CPUState) -> CPUState {
    match cpu.cc.z {
        1 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = cpu.pc.to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 2,
            ..cpu
        },
    }
}

pub fn cnz(cpu: CPUState) -> CPUState {
    match cpu.cc.z {
        0 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = cpu.pc.to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 2,
            ..cpu
        },
    }
}

pub fn cp(cpu: CPUState) -> CPUState {
    match cpu.cc.s {
        1 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = cpu.pc.to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 2,
            ..cpu
        },
    }
}

pub fn cm(cpu: CPUState) -> CPUState {
    match cpu.cc.s {
        0 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = cpu.pc.to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 2,
            ..cpu
        },
    }
}

pub fn cpe(cpu: CPUState) -> CPUState {
    match cpu.cc.p {
        1 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = cpu.pc.to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 2,
            ..cpu
        },
    }
}

pub fn cpo(cpu: CPUState) -> CPUState {
    match cpu.cc.p {
        0 => {
            let opcode = &cpu.memory[cpu.pc as usize..];
            let pc = cpu.pc.to_be_bytes();
            let mut memory = cpu.memory;
            memory[cpu.sp as usize - 1] = pc[0];
            memory[cpu.sp as usize - 2] = pc[1];
            CPUState {
                cycles: 5,
                pc: (opcode[2] as u16) << 8 | opcode[1] as u16,
                sp: cpu.sp - 2,
                memory: memory,
                ..cpu
            }
        }
        _ => CPUState {
            cycles: 3,
            pc: cpu.pc + 2,
            ..cpu
        },
    }
}

pub fn ret(cpu: CPUState) -> CPUState {
    let pcl = cpu.memory[cpu.sp as usize];
    let pch = cpu.memory[cpu.sp as usize + 1];
    CPUState {
        cycles: 5,
        pc: (pch as u16) << 8 | pcl as u16,
        sp: cpu.sp + 2,
        ..cpu
    }
}

pub fn rc(cpu: CPUState) -> CPUState {
    let pcl = cpu.memory[cpu.sp as usize];
    let pch = cpu.memory[cpu.sp as usize + 1];
    match cpu.cc.cy {
        1 => CPUState {
            cycles: 5,
            pc: (pch as u16) << 8 | pcl as u16,
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
    let pcl = cpu.memory[cpu.sp as usize];
    let pch = cpu.memory[cpu.sp as usize + 1];
    match cpu.cc.cy {
        0 => CPUState {
            cycles: 5,
            pc: (pch as u16) << 8 | pcl as u16,
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
    let pcl = cpu.memory[cpu.sp as usize];
    let pch = cpu.memory[cpu.sp as usize + 1];
    match cpu.cc.z {
        1 => CPUState {
            cycles: 5,
            pc: (pch as u16) << 8 | pcl as u16,
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
    let pcl = cpu.memory[cpu.sp as usize];
    let pch = cpu.memory[cpu.sp as usize + 1];
    match cpu.cc.z {
        0 => CPUState {
            cycles: 5,
            pc: (pch as u16) << 8 | pcl as u16,
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
    let pcl = cpu.memory[cpu.sp as usize];
    let pch = cpu.memory[cpu.sp as usize + 1];
    match cpu.cc.s {
        1 => CPUState {
            cycles: 5,
            pc: (pch as u16) << 8 | pcl as u16,
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
    let pcl = cpu.memory[cpu.sp as usize];
    let pch = cpu.memory[cpu.sp as usize + 1];
    match cpu.cc.s {
        0 => CPUState {
            cycles: 5,
            pc: (pch as u16) << 8 | pcl as u16,
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
    let pcl = cpu.memory[cpu.sp as usize];
    let pch = cpu.memory[cpu.sp as usize + 1];
    match cpu.cc.p {
        1 => CPUState {
            cycles: 5,
            pc: (pch as u16) << 8 | pcl as u16,
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
    let pcl = cpu.memory[cpu.sp as usize];
    let pch = cpu.memory[cpu.sp as usize + 1];
    match cpu.cc.p {
        0 => CPUState {
            cycles: 5,
            pc: (pch as u16) << 8 | pcl as u16,
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
