use crate::cpu::CPUState;

pub fn mov_r_r(r: char, value: u8, cpu: CPUState) -> CPUState {
    let inter_cpu = match r {
        'a' => CPUState { a: value, ..cpu },
        'b' => CPUState { b: value, ..cpu },
        'c' => CPUState { c: value, ..cpu },
        'd' => CPUState { d: value, ..cpu },
        'e' => CPUState { e: value, ..cpu },
        'h' => CPUState { h: value, ..cpu },
        'l' => CPUState { l: value, ..cpu },
        _ => cpu,
    };
    CPUState {
        cycles: 1,
        pc: inter_cpu.pc + 1,
        ..inter_cpu
    }
}

pub fn mov_r_m(cpu: CPUState, r: char) -> CPUState {
    println!("Mov R M ");
    let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    let value = cpu.memory[address as usize];
    let inter_cpu = match r {
        'a' => CPUState { a: value, ..cpu },
        'b' => CPUState { b: value, ..cpu },
        'c' => CPUState { c: value, ..cpu },
        'd' => CPUState { d: value, ..cpu },
        'e' => CPUState { e: value, ..cpu },
        'h' => CPUState { h: value, ..cpu },
        'l' => CPUState { l: value, ..cpu },
        _ => cpu,
    };
    CPUState {
        cycles: 2,
        pc: inter_cpu.pc + 1,
        ..inter_cpu
    }
}

pub fn mov_m_r(cpu: CPUState, r: char) -> CPUState {
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

pub fn mvi_r(cpu: CPUState, r: char, value: u8) -> CPUState {
    println!("{}", value);
    let inter_cpu = match r {
        'a' => CPUState { a: value, ..cpu },
        'b' => CPUState { b: value, ..cpu },
        'c' => CPUState { c: value, ..cpu },
        'd' => CPUState { d: value, ..cpu },
        'e' => CPUState { e: value, ..cpu },
        'h' => CPUState { h: value, ..cpu },
        'l' => CPUState { l: value, ..cpu },
        _ => cpu,
    };
    CPUState {
        cycles: 2,
        pc: inter_cpu.pc + 2,
        ..inter_cpu
    }
}

pub fn mvi_m(cpu: CPUState, value: u8) -> CPUState {
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

pub fn lxi(cpu: CPUState, rs: (char, char), opcode_1: u8, opcode_2: u8) -> CPUState {
    match rs {
        ('b', 'c') => CPUState {
            b: opcode_2,
            c: opcode_1,
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
        ('d', 'e') => CPUState {
            d: opcode_2,
            e: opcode_1,
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
        ('h', 'l') => CPUState {
            h: opcode_2,
            l: opcode_1,
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
        ('s', 'p') => CPUState {
            sp: ((opcode_2 as u16) << 8 | opcode_1 as u16),
            cycles: 3,
            pc: cpu.pc + 3,
            ..cpu
        },
        _ => cpu,
    }
}

pub fn lda(cpu: CPUState, opcode_1: u8, opcode_2: u8) -> CPUState {
    let address: u16 = (opcode_2 as u16) << 8 | opcode_1 as u16;
    CPUState {
        a: cpu.memory[address as usize],
        cycles: 4,
        pc: cpu.pc + 3,
        ..cpu
    }
}

pub fn sta(cpu: CPUState) -> CPUState {
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

pub fn lhld(cpu: CPUState) -> CPUState {
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

pub fn shld(cpu: CPUState) -> CPUState {
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

pub fn ldax(cpu: CPUState, rs: (char, char)) -> CPUState {
    let value: u8 = match rs {
        ('b', 'c') => {
            let address: u16 = (cpu.b as u16) << 8 | cpu.c as u16;
            cpu.memory[address as usize]
        }
        ('d', 'e') => {
            let address: u16 = (cpu.d as u16) << 8 | cpu.e as u16;
            cpu.memory[address as usize]
        }
        _ => cpu.a,
    };
    CPUState {
        a: value,
        cycles: 2,
        pc: cpu.pc + 1,
        ..cpu
    }
}

pub fn stax(cpu: CPUState, rs: (char, char)) -> CPUState {
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

pub fn xchg(cpu: CPUState) -> CPUState {
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
