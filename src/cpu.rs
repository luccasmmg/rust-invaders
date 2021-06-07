#![allow(dead_code)]
use crate::condition_codes::ConditionCodes;
use crate::op_arithmetic::*;
use crate::op_data_transfer::*;
use crate::helpers::get_value_memory;
use std::fmt;
use crate::op_logical::*;
use crate::op_branch::*;
use crate::op_stack::*;
use crate::op_special_io::*;

pub const MEMORY_SIZE: usize = 0xffff;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StackPairs {
    BC,
    DE,
    HL
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WithSPPairs {
    BC,
    DE,
    HL,
    SP
}

pub enum Registers {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, PartialEq)]
pub struct CPUState {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub cycles: u8,
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: bool,
}

impl CPUState {
    pub fn new() -> CPUState {
        CPUState {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 61440,
            pc: 0,
            cycles: 0,
            memory: vec![0; MEMORY_SIZE],
            cc: ConditionCodes::new(),
            int_enable: false,
        }
    }

    pub fn load_memory(&mut self, rom: &Vec<u8>, size: usize) {
        self.memory[..size].copy_from_slice(rom);
    }
}

impl fmt::Display for CPUState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Registers -> AF: {:02x}00, BC: {:02x}{:02x}, DE: {:02x}{:02x}, HL: {:02x}{:02x} \n
Flags -> Z: {:02x} S: {:02x} P: {:02x} CY: {:02x}\n
PC/SP -> PC: {:04x}, SP: {:04x}\n -----------------------------------------------------------------------------------",
                self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.cc.z, self.cc.s, self.cc.p, self.cc.cy, self.pc, self.sp)
        }
}

pub fn emulate_8080_op(cpu: CPUState, opcode: &[u8]) -> CPUState {
    match opcode[0] {
        0x00 => nop(cpu),
        // LXI OPS
        0x01 => lxi(cpu, ('b', 'c'), opcode[1], opcode[2]),
        0x11 => lxi(cpu, ('d', 'e'), opcode[1], opcode[2]),
        0x21 => lxi(cpu, ('h', 'l'), opcode[1], opcode[2]),
        0x31 => lxi(cpu, ('s', 'p'), opcode[1], opcode[2]),

        // INX OPS
        0x03 => inx(cpu, WithSPPairs::BC),
        0x13 => inx(cpu, WithSPPairs::DE),
        0x23 => inx(cpu, WithSPPairs::HL),
        0x33 => inx(cpu, WithSPPairs::SP),

        // DCX OPS
        0x0b => dcx(cpu, WithSPPairs::BC),
        0x1b => dcx(cpu, WithSPPairs::DE),
        0x2b => dcx(cpu, WithSPPairs::HL),
        0x3b => dcx(cpu, WithSPPairs::SP),

        //DAD OPS
        0x09 => dad(cpu, ('b', 'c')),
        0x19 => dad(cpu, ('d', 'e')),
        0x29 => dad(cpu, ('h', 'l')),
        0x39 => dad(cpu, ('s', 'p')),

        // INR OPS
        0x3c => inr_r(cpu, 'a'),
        0x04 => inr_r(cpu, 'b'),
        0x0c => inr_r(cpu, 'c'),
        0x14 => inr_r(cpu, 'd'),
        0x1c => inr_r(cpu, 'e'),
        0x24 => inr_r(cpu, 'h'),
        0x2c => inr_r(cpu, 'l'),
        0x34 => inr_m(cpu),

        // DCR OPS
        0x3d => dcr_r(cpu, 'a'),
        0x05 => dcr_r(cpu, 'b'),
        0x0d => dcr_r(cpu, 'c'),
        0x15 => dcr_r(cpu, 'd'),
        0x1d => dcr_r(cpu, 'e'),
        0x25 => dcr_r(cpu, 'h'),
        0x2d => dcr_r(cpu, 'l'),
        0x35 => dcr_m(cpu),

        // MVI
        0x06 => mvi_r(cpu, 'b', opcode[1]),
        0x0e => mvi_r(cpu, 'c', opcode[1]),
        0x16 => mvi_r(cpu, 'd', opcode[1]),
        0x1e => mvi_r(cpu, 'e', opcode[1]),
        0x26 => mvi_r(cpu, 'h', opcode[1]),
        0x2e => mvi_r(cpu, 'l', opcode[1]),
        0x36 => mvi_m(cpu, opcode[1]),
        0x3e => mvi_r(cpu, 'a', opcode[1]),

        // STAX OPS
        0x02 => stax(cpu, ('b', 'c')),
        0x12 => stax(cpu, ('d', 'e')),

        // LDAX
        0x0a => ldax(cpu, ('b', 'c')),
        0x1a => ldax(cpu, ('d', 'e')),

        // STA
        0x32 => sta(cpu),

        // LDA
        0x3a => lda(cpu, opcode[1], opcode[2]),

        // SHLD
        0x22 => shld(cpu),

        // LHLD
        0x2a => lhld(cpu),

        // MOV OPS
        0x41 => mov_r_r(Registers::B, cpu.c, cpu),
        0x42 => mov_r_r(Registers::B, cpu.d, cpu),
        0x43 => mov_r_r(Registers::B, cpu.e, cpu),
        0x44 => mov_r_r(Registers::B, cpu.h, cpu),
        0x45 => mov_r_r(Registers::B, cpu.l, cpu),
        0x46 => mov_r_m(cpu, Registers::B),
        0x47 => mov_r_r(Registers::B, cpu.a, cpu),
        0x48 => mov_r_r(Registers::C, cpu.b, cpu),
        0x4a => mov_r_r(Registers::C, cpu.d, cpu),
        0x4b => mov_r_r(Registers::C, cpu.e, cpu),
        0x4c => mov_r_r(Registers::C, cpu.h, cpu),
        0x4d => mov_r_r(Registers::C, cpu.l, cpu),
        0x4e => mov_r_m(cpu, Registers::C),
        0x4f => mov_r_r(Registers::C, cpu.a, cpu),
        0x50 => mov_r_r(Registers::D, cpu.b, cpu),
        0x51 => mov_r_r(Registers::D, cpu.c, cpu),
        0x53 => mov_r_r(Registers::D, cpu.e, cpu),
        0x54 => mov_r_r(Registers::D, cpu.h, cpu),
        0x55 => mov_r_r(Registers::D, cpu.l, cpu),
        0x56 => mov_r_m(cpu, Registers::D),
        0x57 => mov_r_r(Registers::D, cpu.a, cpu),
        0x58 => mov_r_r(Registers::E, cpu.b, cpu),
        0x59 => mov_r_r(Registers::E, cpu.c, cpu),
        0x5a => mov_r_r(Registers::E, cpu.d, cpu),
        0x5c => mov_r_r(Registers::E, cpu.h, cpu),
        0x5d => mov_r_r(Registers::E, cpu.l, cpu),
        0x5e => mov_r_m(cpu, Registers::E),
        0x5f => mov_r_r(Registers::E, cpu.a, cpu),
        0x60 => mov_r_r(Registers::H, cpu.b, cpu),
        0x61 => mov_r_r(Registers::H, cpu.c, cpu),
        0x62 => mov_r_r(Registers::H, cpu.d, cpu),
        0x63 => mov_r_r(Registers::H, cpu.e, cpu),
        0x65 => mov_r_r(Registers::H, cpu.l, cpu),
        0x66 => mov_r_m(cpu, Registers::H),
        0x67 => mov_r_r(Registers::H, cpu.a, cpu),
        0x68 => mov_r_r(Registers::L, cpu.b, cpu),
        0x69 => mov_r_r(Registers::L, cpu.c, cpu),
        0x6a => mov_r_r(Registers::L, cpu.d, cpu),
        0x6b => mov_r_r(Registers::L, cpu.e, cpu),
        0x6c => mov_r_r(Registers::L, cpu.h, cpu),
        0x6e => mov_r_m(cpu, Registers::L),
        0x6f => mov_r_r(Registers::L, cpu.a, cpu),
        0x70 => mov_m_r(cpu, Registers::B),
        0x71 => mov_m_r(cpu, Registers::C),
        0x72 => mov_m_r(cpu, Registers::D),
        0x73 => mov_m_r(cpu, Registers::E),
        0x74 => mov_m_r(cpu, Registers::H),
        0x75 => mov_m_r(cpu, Registers::L),
        0x77 => mov_m_r(cpu, Registers::A),
        0x78 => mov_r_r(Registers::A, cpu.b, cpu),
        0x79 => mov_r_r(Registers::A, cpu.c, cpu),
        0x7a => mov_r_r(Registers::A, cpu.d, cpu),
        0x7b => mov_r_r(Registers::A, cpu.e, cpu),
        0x7c => mov_r_r(Registers::A, cpu.h, cpu),
        0x7d => mov_r_r(Registers::A, cpu.l, cpu),
        0x7e => mov_r_m(cpu, Registers::A),

        //PUSH
        0xc5 => push(cpu, StackPairs::BC),
        0xd5 => push(cpu, StackPairs::DE),
        0xe5 => push(cpu, StackPairs::HL),
        0xf5 => push_psw(cpu),

        //POP
        0xc1 => pop(cpu, StackPairs::BC),
        0xd1 => pop(cpu, StackPairs::DE),
        0xe1 => pop(cpu, StackPairs::HL),
        0xf1 => pop_psw(cpu),

        //XTHL
        0xe3 => xthl(cpu),
        //SPHL
        0xf9 => sphl(cpu),

        // ADD OPS
        0x80 => add(cpu.b, 1, cpu),
        0x81 => add(cpu.c, 1, cpu),
        0x82 => add(cpu.d, 1, cpu),
        0x83 => add(cpu.e, 1, cpu),
        0x84 => add(cpu.h, 1, cpu),
        0x85 => add(cpu.l, 1, cpu),
        0x86 => add(get_value_memory(&cpu.memory, cpu.h, cpu.l), 2, cpu),
        0x87 => add(cpu.a, 1, cpu),
        // ADC OPS
        0x88 => add((cpu.b).wrapping_add(cpu.cc.cy), 2, cpu),
        0x89 => add((cpu.c).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8a => add((cpu.d).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8b => add((cpu.e).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8c => add((cpu.h).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8d => add((cpu.l).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8e => add(get_value_memory(&cpu.memory, cpu.h, cpu.l).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8f => add((cpu.a).wrapping_add(cpu.cc.cy), 2, cpu),

        // SUB OPS
        0x90 => sub(cpu.b, 2, cpu),
        0x91 => sub(cpu.c, 2, cpu),
        0x92 => sub(cpu.d, 2, cpu),
        0x93 => sub(cpu.e, 2, cpu),
        0x94 => sub(cpu.h, 2, cpu),
        0x95 => sub(cpu.l, 2, cpu),
        0x96 => sub(get_value_memory(&cpu.memory, cpu.h, cpu.l), 2, cpu),
        0x97 => sub(cpu.a, 2, cpu),

        // SUBB OPS
        0x98 => sub((cpu.b).wrapping_add(cpu.cc.cy), 1, cpu),
        0x99 => sub((cpu.c).wrapping_add(cpu.cc.cy), 1, cpu),
        0x9a => sub((cpu.d).wrapping_add(cpu.cc.cy), 1, cpu),
        0x9b => sub((cpu.e).wrapping_add(cpu.cc.cy), 1, cpu),
        0x9c => sub((cpu.h).wrapping_add(cpu.cc.cy), 1, cpu),
        0x9d => sub((cpu.l).wrapping_add(cpu.cc.cy), 1, cpu),
        0x9e => sub(get_value_memory(&cpu.memory, cpu.h, cpu.l).wrapping_add(cpu.cc.cy), 1, cpu),
        0x9f => sub((cpu.a).wrapping_add(cpu.cc.cy), 1, cpu),

        // ADI OPS
        0xc6 => adi(opcode[1], 2, cpu),
        0xce => adi(opcode[1].wrapping_add(cpu.cc.cy), 2, cpu),
        0xeb => xchg(cpu),

        // SUI OPS
        0xd6 => sui(opcode[1], 2, cpu),
        0xde => sui(opcode[1].wrapping_add(cpu.cc.cy), 2, cpu),

        //JMPS
        0xc3 => jmp(cpu, opcode[1], opcode[2]),
        0xc2 => jnz(cpu, opcode[1], opcode[2]),
        0xca => jz(cpu, opcode[1], opcode[2]),
        0xe2 => jpo(cpu, opcode[1], opcode[2]),
        0xea => jpe(cpu, opcode[1], opcode[2]),
        0xf2 => jp(cpu, opcode[1], opcode[2]),
        0xfa => jm(cpu, opcode[1], opcode[2]),
        0xd2 => jnc(cpu, opcode[1], opcode[2]),
        0xda => jc(cpu, opcode[1], opcode[2]),

        //CALLS
        0xcd => call(cpu, opcode[1], opcode[2]),
        0xdc => cc(cpu, opcode[1], opcode[2]),
        0xd4 => cnc(cpu, opcode[1], opcode[2]),
        0xcc => cz(cpu, opcode[1], opcode[2]),
        0xc4 => cnz(cpu, opcode[1], opcode[2]),
        0xf4 => cp(cpu, opcode[1], opcode[2]),
        0xfc => cm(cpu, opcode[1], opcode[2]),
        0xec => cpe(cpu, opcode[1], opcode[2]),
        0xe4 => cpo(cpu, opcode[1], opcode[2]),

        //Rs
        0xc9 => ret(cpu),
        0xd8 => rc(cpu),
        0xd0 => rnc(cpu),
        0xc8 => rz(cpu),
        0xc0 => rnz(cpu),
        0xf0 => rp(cpu),
        0xf8 => rm(cpu),
        0xe8 => rpe(cpu),
        0xe0 => rpo(cpu),

        //PHCL
        0xe9 => pchl(cpu),

        //RSTs
        0xc7 => rst(cpu, 0),
        0xcf => rst(cpu, 1),
        0xd7 => rst(cpu, 2),
        0xdf => rst(cpu, 3),
        0xe7 => rst(cpu, 4),
        0xef => rst(cpu, 5),
        0xf7 => rst(cpu, 6),
        0xff => rst(cpu, 7),

        // ANA OPS
        0xa0 => ana(cpu.b, 1, cpu),
        0xa1 => ana(cpu.c, 1, cpu),
        0xa2 => ana(cpu.d, 1, cpu),
        0xa3 => ana(cpu.e, 1, cpu),
        0xa4 => ana(cpu.h, 1, cpu),
        0xa5 => ana(cpu.l, 1, cpu),
        0xa6 => ana(get_value_memory(&cpu.memory, cpu.h, cpu.l), 2, cpu),
        0xa7 => ana(cpu.a, 1, cpu),

        // XRA OPS
        0xa8 => xra(cpu.b, 1, cpu),
        0xa9 => xra(cpu.c, 1, cpu),
        0xaa => xra(cpu.d, 1, cpu),
        0xab => xra(cpu.e, 1, cpu),
        0xac => xra(cpu.h, 1, cpu),
        0xad => xra(cpu.l, 1, cpu),
        0xae => xra(get_value_memory(&cpu.memory, cpu.h, cpu.l), 2, cpu),
        0xaf => xra(cpu.a, 1, cpu),

        // ORA OPS
        0xb0 => ora(cpu.b, 1, cpu),
        0xb1 => ora(cpu.c, 1, cpu),
        0xb2 => ora(cpu.d, 1, cpu),
        0xb3 => ora(cpu.e, 1, cpu),
        0xb4 => ora(cpu.h, 1, cpu),
        0xb5 => ora(cpu.l, 1, cpu),
        0xb6 => ora(get_value_memory(&cpu.memory, cpu.h, cpu.l), 2, cpu),
        0xb7 => ora(cpu.a, 1, cpu),

        // CMP OPS
        0xb8 => cmp(cpu.b, 1, cpu),
        0xb9 => cmp(cpu.c, 1, cpu),
        0xba => cmp(cpu.d, 1, cpu),
        0xbb => cmp(cpu.e, 1, cpu),
        0xbc => cmp(cpu.h, 1, cpu),
        0xbd => cmp(cpu.l, 1, cpu),
        0xbe => cmp(get_value_memory(&cpu.memory, cpu.h, cpu.l), 2, cpu),
        0xbf => cmp(cpu.a, 1, cpu),

        //ANI
        0xe6 => ani(opcode[1], cpu),
        //XRI
        0xee => xri(opcode[1], cpu),
        //ORI
        0xf6 => ori(opcode[1], cpu),
        //CPI
        0xfe => cpi(opcode[1], cpu),
        //RLC
        0x07 => rlc(cpu),
        //RRC
        0x0f => rrc(cpu),
        //RAL
        0x17 => ral(cpu),
        //RAR
        0x1f => rar(cpu),
        //CMA
        0x2f => cma(cpu),
        //CMC
        0x3f => cmc(cpu),
        //STC
        0x37 => stc(cpu),

        //IO SPECIAL
        0xfb => ei(cpu),
        0xf3 => di(cpu),
        0x76 => panic!(),
        _ => cpu,

    }
}
