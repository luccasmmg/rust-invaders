use crate::cpu::CPUState;
use crate::cpu::Registers;
use crate::helpers::write_memory;

pub fn mov_r_r(r: Registers, value: u8, cpu: CPUState) -> CPUState {
    let inter_cpu = match r {
        Registers::A => CPUState { a: value, ..cpu },
        Registers::B => CPUState { b: value, ..cpu },
        Registers::C => CPUState { c: value, ..cpu },
        Registers::D => CPUState { d: value, ..cpu },
        Registers::E => CPUState { e: value, ..cpu },
        Registers::H => CPUState { h: value, ..cpu },
        Registers::L => CPUState { l: value, ..cpu },
    };
    CPUState {
        cycles: 1,
        pc: inter_cpu.pc.wrapping_add(1),
        ..inter_cpu
    }
}

pub fn mov_r_m(cpu: CPUState, r: Registers) -> CPUState {
    let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    let value = cpu.memory[address as usize];
    let inter_cpu = match r {
        Registers::A => CPUState { a: value, ..cpu },
        Registers::B => CPUState { b: value, ..cpu },
        Registers::C => CPUState { c: value, ..cpu },
        Registers::D => CPUState { d: value, ..cpu },
        Registers::E => CPUState { e: value, ..cpu },
        Registers::H => CPUState { h: value, ..cpu },
        Registers::L => CPUState { l: value, ..cpu },
    };
    CPUState {
        cycles: 2,
        pc: inter_cpu.pc.wrapping_add(1),
        ..inter_cpu
    }
}

pub fn mov_m_r(cpu: CPUState, r: Registers) -> CPUState {
    let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    let memory = match r {
        Registers::A => write_memory(cpu.memory, address, cpu.a),
        Registers::B => write_memory(cpu.memory, address, cpu.b),
        Registers::C => write_memory(cpu.memory, address, cpu.c),
        Registers::D => write_memory(cpu.memory, address, cpu.d),
        Registers::E => write_memory(cpu.memory, address, cpu.e),
        Registers::H => write_memory(cpu.memory, address, cpu.h),
        Registers::L => write_memory(cpu.memory, address, cpu.l),
    };
    CPUState {
        memory,
        cycles: 2,
        pc: cpu.pc.wrapping_add(1),
        ..cpu
    }
}

pub fn mvi_r(cpu: CPUState, r: char, value: u8) -> CPUState {
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
        pc: inter_cpu.pc.wrapping_add(2),
        ..inter_cpu
    }
}

pub fn mvi_m(cpu: CPUState, value: u8) -> CPUState {
    let address: u16 = (cpu.h as u16) << 8 | cpu.l as u16;
    CPUState {
        memory: write_memory(cpu.memory, address, value),
        cycles: 3,
        pc: cpu.pc.wrapping_add(2),
        ..cpu
    }
}

pub fn lxi(cpu: CPUState, rs: (char, char), opcode_1: u8, opcode_2: u8) -> CPUState {
    match rs {
        ('b', 'c') => CPUState {
            b: opcode_2,
            c: opcode_1,
            cycles: 3,
            pc: cpu.pc.wrapping_add(3),
            ..cpu
        },
        ('d', 'e') => CPUState {
            d: opcode_2,
            e: opcode_1,
            cycles: 3,
            pc: cpu.pc.wrapping_add(3),
            ..cpu
        },
        ('h', 'l') => CPUState {
            h: opcode_2,
            l: opcode_1,
            cycles: 3,
            pc: cpu.pc.wrapping_add(3),
            ..cpu
        },
        ('s', 'p') => CPUState {
            sp: ((opcode_2 as u16) << 8 | opcode_1 as u16),
            cycles: 3,
            pc: cpu.pc.wrapping_add(3),
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
        pc: cpu.pc.wrapping_add(3),
        ..cpu
    }
}

pub fn sta(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
    let memory = write_memory(cpu.memory, address, cpu.a);
    CPUState {
        memory,
        cycles: 4,
        pc: cpu.pc.wrapping_add(3),
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
        pc: cpu.pc.wrapping_add(3),
        ..cpu
    }
}

pub fn shld(cpu: CPUState) -> CPUState {
    let opcode = &cpu.memory[cpu.pc as usize..];
    let address_l: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
    let address_h: u16 = address_l + 1;
    let memory_l = write_memory(cpu.memory, address_l, cpu.l);
    let memory_h = write_memory(memory_l, address_h, cpu.h);
    CPUState {
        memory: memory_h,
        cycles: 5,
        pc: cpu.pc.wrapping_add(3),
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
        pc: cpu.pc.wrapping_add(1),
        ..cpu
    }
}

pub fn stax(cpu: CPUState, rs: (char, char)) -> CPUState {
    let memory = match rs {
        ('b', 'c') => {
            let address: u16 = (cpu.b as u16) << 8 | cpu.c as u16;
            write_memory(cpu.memory, address, cpu.a)
        }
        ('d', 'e') => {
            let address: u16 = (cpu.d as u16) << 8 | cpu.e as u16;
            write_memory(cpu.memory, address, cpu.a)
        }
        _ => cpu.memory
    };
    CPUState {
        memory,
        cycles: 2,
        pc: cpu.pc.wrapping_add(1),
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
        pc: cpu.pc.wrapping_add(1),
        ..cpu
    }
}
