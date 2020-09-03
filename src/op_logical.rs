#![allow(dead_code)]
use crate::condition_codes::ConditionCodes;
use crate::cpu::CPUState;
use crate::helpers::arith_flags_logs;

pub fn ana(value: u8, cycles: u8, cpu: CPUState) -> CPUState {
    let answer = (cpu.a & value) as u16;
    let flags_result = arith_flags_logs(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[1],
        cycles,
        pc: cpu.pc + 1,
        cc: flags,
        ..cpu
    }
}

pub fn ani(value: u8, cpu: CPUState) -> CPUState {
    let answer = (cpu.a & value) as u16;
    let flags_result = arith_flags_logs(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ac: 0,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[1],
        cycles: 2,
        pc: cpu.pc + 2,
        cc: flags,
        ..cpu
    }
}

pub fn xra(value: u8, cycles: u8, cpu: CPUState) -> CPUState {
    let answer = (cpu.a ^ value) as u16;
    let flags_result = arith_flags_logs(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[1],
        cycles,
        pc: cpu.pc + 1,
        cc: flags,
        ..cpu
    }
}

pub fn xri(value: u8, cpu: CPUState) -> CPUState {
    let answer = (cpu.a ^ value) as u16;
    let flags_result = arith_flags_logs(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[1],
        cycles: 2,
        pc: cpu.pc + 2,
        cc: flags,
        ..cpu
    }
}

pub fn ora(value: u8, cycles: u8, cpu: CPUState) -> CPUState {
    let answer = (cpu.a | value) as u16;
    let flags_result = arith_flags_logs(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[1],
        cycles,
        pc: cpu.pc + 1,
        cc: flags,
        ..cpu
    }
}

pub fn ori(value: u8, cpu: CPUState) -> CPUState {
    let answer = (cpu.a | value) as u16;
    let flags_result = arith_flags_logs(answer);
    let flags = ConditionCodes {
        z: flags_result.0,
        s: flags_result.1,
        cy: 0,
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        a: answer.to_be_bytes()[1],
        cycles: 2,
        pc: cpu.pc + 2,
        cc: flags,
        ..cpu
    }
}

pub fn cmp(value: u8, cycles: u8, cpu: CPUState) -> CPUState {
    let answer = ((cpu.a).wrapping_sub(value)) as u16;
    let flags_result = arith_flags_logs(answer);
    let flags = ConditionCodes {
        z: if answer.to_be_bytes()[1] == cpu.a {
            1
        } else {
            cpu.cc.z
        },
        s: flags_result.1,
        cy: if cpu.a < value {
            1
        } else {
            cpu.cc.z
        },
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

pub fn cpi(value: u8, cpu: CPUState) -> CPUState {
    let answer = (cpu.a as u16).wrapping_sub(value as u16);
    let flags_result = arith_flags_logs(answer);
    let flags = ConditionCodes {
        z: if value == cpu.a {
            1
        } else {
            0
        },
        s: flags_result.1,
        cy: if cpu.a < value {
            1
        } else {
            0
        },
        p: flags_result.3,
        ..cpu.cc
    };

    CPUState {
        cycles: 2,
        pc: cpu.pc + 2,
        cc: flags,
        ..cpu
    }
}

pub fn rlc(cpu: CPUState) -> CPUState {
    let bit7: u8 = cpu.a & (1 << 7);
    let mut answer: u8 = cpu.a << 1;
    answer |= bit7 >> 7;
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: answer,
        cc: ConditionCodes { cy: bit7, ..cpu.cc },
        ..cpu
    }
}

pub fn rrc(cpu: CPUState) -> CPUState {
    let bit0: u8 = cpu.a & 1;
    let mut answer = cpu.a >> 1;
    answer |= bit0 << 7;
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: answer,
        cc: ConditionCodes { cy: bit0, ..cpu.cc },
        ..cpu
    }
}

pub fn rar(cpu: CPUState) -> CPUState {
    let bit0: u8 = cpu.a & 1;
    let mut answer = cpu.a >> 1;
    if cpu.cc.cy == 1 { answer |= 1 << 7; }
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: answer,
        cc: ConditionCodes { cy: bit0, ..cpu.cc },
        ..cpu
    }
}

pub fn ral(cpu: CPUState) -> CPUState {
    let bit7: u8 = cpu.a & (1 << 7);
    let mut answer = cpu.a << 1;
    answer |= cpu.cc.cy;
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: answer,
        cc: ConditionCodes { cy: bit7, ..cpu.cc },
        ..cpu
    }
}

pub fn cma(cpu: CPUState) -> CPUState {
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        a: !cpu.a,
        ..cpu
    }
}

pub fn cmc(cpu: CPUState) -> CPUState {
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        cc: ConditionCodes {
            cy: !cpu.cc.cy,
            ..cpu.cc
        },
        ..cpu
    }
}

pub fn stc(cpu: CPUState) -> CPUState {
    CPUState {
        cycles: 1,
        pc: cpu.pc + 1,
        cc: ConditionCodes { cy: 1, ..cpu.cc },
        ..cpu
    }
}
