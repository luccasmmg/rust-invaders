#![allow(dead_code)]
use crate::condition_codes::ConditionCodes;
use crate::op_arithmetic::*;
use crate::op_data_transfer::*;
use crate::helpers::get_value_memory;
use std::fmt;
use crate::op_logical::*;
//use crate::op_special_io::*;

pub const MEMORY_SIZE: usize = 0x4000;

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
    pub memory: [u8; MEMORY_SIZE],
    pub cc: ConditionCodes,
    pub int_enable: u8,
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

impl fmt::Display for CPUState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Registers -> A: {}, B: {}, C: {}, D: {}, E: {}, H: {}, L: {} \n Flags -> Z: {} S: {} P: {} CY: {} AC: {}",
                self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.cc.z, self.cc.s, self.cc.p, self.cc.cy, self.cc.ac)
        }
}

fn emulate_8080_op(cpu: CPUState) -> CPUState {
    let pc: usize = cpu.pc as usize;
    let opcode = &cpu.memory[pc..];
    match opcode[0] {
        //         0x00 => (),
        // LXI OPS
        0x01 => lxi(cpu, ('b', 'c')),
        0x11 => lxi(cpu, ('d', 'e')),
        0x21 => lxi(cpu, ('h', 'l')),
        0x31 => lxi(cpu, ('s', 'p')),

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
        0x04 => inr_r(cpu, 'b'),
        0x0c => inr_r(cpu, 'c'),
        0x14 => inr_r(cpu, 'd'),
        0x1c => inr_r(cpu, 'e'),
        0x24 => inr_r(cpu, 'h'),
        0x2c => inr_r(cpu, 'l'),
        0x34 => inr_m(cpu),

        // DCR OPS
        0x05 => dcr_r(cpu, 'b'),
        0x0d => dcr_r(cpu, 'c'),
        0x15 => dcr_r(cpu, 'd'),
        0x1d => dcr_r(cpu, 'e'),
        0x25 => dcr_r(cpu, 'h'),
        0x2d => dcr_r(cpu, 'l'),
        0x35 => dcr_m(cpu),

        // MVI
        0x06 => mvi_r(cpu, 'b'),
        0x0e => mvi_r(cpu, 'c'),
        0x16 => mvi_r(cpu, 'd'),
        0x1e => mvi_r(cpu, 'e'),
        0x26 => mvi_r(cpu, 'h'),
        0x2e => mvi_r(cpu, 'l'),
        0x36 => mvi_m(cpu),
        0x3e => mvi_r(cpu, 'a'),

        // STAX OPS
        0x02 => stax(cpu, ('b', 'c')),
        0x12 => stax(cpu, ('d', 'e')),

        // LDAX
        0x0a => ldax(cpu, ('b', 'c')),
        0x1a => ldax(cpu, ('d', 'e')),

        // STA
        0x32 => sta(cpu),

        // LDA
        0x3a => lda(cpu),

        // SHLD
        0x22 => shld(cpu),

        // LHLD
        0x2a => lhld(cpu),

        // MOV OPS
        0x41 => mov_r_r('b', cpu.c, cpu),
        0x42 => mov_r_r('b', cpu.d, cpu),
        0x43 => mov_r_r('b', cpu.e, cpu),
        0x44 => mov_r_r('b', cpu.h, cpu),
        0x45 => mov_r_r('b', cpu.l, cpu),
        0x46 => mov_r_m(cpu, 'b'),
        0x47 => mov_r_r('a', cpu.a, cpu),
        0x48 => mov_r_r('c', cpu.b, cpu),
        0x4a => mov_r_r('c', cpu.d, cpu),
        0x4b => mov_r_r('c', cpu.e, cpu),
        0x4c => mov_r_r('c', cpu.h, cpu),
        0x4d => mov_r_r('c', cpu.l, cpu),
        0x4e => mov_r_m(cpu, 'c'),
        0x4f => mov_r_r('c', cpu.a, cpu),
        0x50 => mov_r_r('d', cpu.b, cpu),
        0x51 => mov_r_r('d', cpu.c, cpu),
        0x53 => mov_r_r('d', cpu.e, cpu),
        0x54 => mov_r_r('d', cpu.h, cpu),
        0x55 => mov_r_r('d', cpu.l, cpu),
        0x56 => mov_r_m(cpu, 'd'),
        0x57 => mov_r_r('d', cpu.a, cpu),
        0x58 => mov_r_r('e', cpu.b, cpu),
        0x59 => mov_r_r('e', cpu.c, cpu),
        0x5a => mov_r_r('e', cpu.d, cpu),
        0x5c => mov_r_r('e', cpu.h, cpu),
        0x5d => mov_r_r('e', cpu.l, cpu),
        0x5e => mov_r_m(cpu, 'e'),
        0x5f => mov_r_r('e', cpu.a, cpu),
        0x60 => mov_r_r('h', cpu.b, cpu),
        0x61 => mov_r_r('h', cpu.c, cpu),
        0x62 => mov_r_r('h', cpu.d, cpu),
        0x63 => mov_r_r('h', cpu.e, cpu),
        0x65 => mov_r_r('h', cpu.l, cpu),
        0x66 => mov_r_m(cpu, 'h'),
        0x67 => mov_r_r('h', cpu.a, cpu),
        0x68 => mov_r_r('l', cpu.b, cpu),
        0x69 => mov_r_r('l', cpu.c, cpu),
        0x6a => mov_r_r('l', cpu.d, cpu),
        0x6b => mov_r_r('l', cpu.e, cpu),
        0x6c => mov_r_r('l', cpu.h, cpu),
        0x6e => mov_r_m(cpu, 'l'),
        0x6f => mov_r_r('l', cpu.a, cpu),
        0x70 => mov_m_r(cpu, 'b'),
        0x71 => mov_m_r(cpu, 'c'),
        0x72 => mov_m_r(cpu, 'd'),
        0x73 => mov_m_r(cpu, 'e'),
        0x74 => mov_m_r(cpu, 'h'),
        0x75 => mov_m_r(cpu, 'l'),
        0x76 => cpu, //TODO
        0x77 => mov_m_r(cpu, 'a'),
        0x78 => mov_r_r('a', cpu.b, cpu),
        0x79 => mov_r_r('a', cpu.c, cpu),
        0x7a => mov_r_r('a', cpu.d, cpu),
        0x7b => mov_r_r('a', cpu.e, cpu),
        0x7c => mov_r_r('a', cpu.h, cpu),
        0x7d => mov_r_r('a', cpu.l, cpu),
        0x7e => mov_r_m(cpu, 'a'),

        // ADD OPS
        0x80 => add(cpu.b, 1, cpu),
        0x81 => add(cpu.c, 1, cpu),
        0x82 => add(cpu.d, 1, cpu),
        0x83 => add(cpu.e, 1, cpu),
        0x84 => add(cpu.h, 1, cpu),
        0x85 => add(cpu.l, 1, cpu),
        0x86 => add(get_value_memory(cpu.memory, cpu.h, cpu.l), 2, cpu),
        0x87 => add(cpu.a, 1, cpu),
        // ADC OPS
        0x88 => add((cpu.b).wrapping_add(cpu.cc.cy), 2, cpu),
        0x89 => add((cpu.c).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8a => add((cpu.d).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8b => add((cpu.e).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8c => add((cpu.h).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8d => add((cpu.l).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8e => add(get_value_memory(cpu.memory, cpu.h, cpu.l).wrapping_add(cpu.cc.cy), 2, cpu),
        0x8f => add((cpu.a).wrapping_add(cpu.cc.cy), 2, cpu),

        // SUB OPS
        0x90 => sub(cpu.b, 2, cpu),
        0x91 => sub(cpu.c, 2, cpu),
        0x92 => sub(cpu.d, 2, cpu),
        0x93 => sub(cpu.e, 2, cpu),
        0x94 => sub(cpu.h, 2, cpu),
        0x95 => sub(cpu.l, 2, cpu),
        0x96 => sub(get_value_memory(cpu.memory, cpu.h, cpu.l), 2, cpu),
        0x97 => sub(cpu.a, 2, cpu),

        // SUBB OPS
        0x98 => sub((cpu.b).wrapping_sub(cpu.cc.cy), 1, cpu),
        0x99 => sub((cpu.c).wrapping_sub(cpu.cc.cy), 1, cpu),
        0x9a => sub((cpu.d).wrapping_sub(cpu.cc.cy), 1, cpu),
        0x9b => sub((cpu.e).wrapping_sub(cpu.cc.cy), 1, cpu),
        0x9c => sub((cpu.h).wrapping_sub(cpu.cc.cy), 1, cpu),
        0x9d => sub((cpu.l).wrapping_sub(cpu.cc.cy), 1, cpu),
        0x9e => sub(get_value_memory(cpu.memory, cpu.h, cpu.l).wrapping_sub(cpu.cc.cy), 1, cpu),
        0x9f => sub((cpu.a).wrapping_sub(cpu.cc.cy), 1, cpu),

        // ADI OPS
        0xc6 => adi(opcode[1], 2, cpu),
        0xce => adi(opcode[1].wrapping_add(cpu.cc.cy), 2, cpu),
        0xeb => xchg(cpu),

        // SUI OPS
        0xd6 => sui(opcode[1], 2, cpu),
        0xde => sui(opcode[1].wrapping_sub(cpu.cc.cy), 2, cpu),

        // ANA OPS
        0xa0 => ana(cpu.b, 1, cpu),
        0xa1 => ana(cpu.c, 1, cpu),
        0xa2 => ana(cpu.d, 1, cpu),
        0xa3 => ana(cpu.e, 1, cpu),
        0xa4 => ana(cpu.h, 1, cpu),
        0xa5 => ana(cpu.l, 1, cpu),
        0xa6 => ana(get_value_memory(cpu.memory, cpu.h, cpu.l), 2, cpu),
        0xa7 => ana(cpu.b, 1, cpu),

        // XRA OPS
        0xa8 => xra(cpu.b, 1, cpu),
        0xa9 => xra(cpu.c, 1, cpu),
        0xaa => xra(cpu.d, 1, cpu),
        0xab => xra(cpu.e, 1, cpu),
        0xac => xra(cpu.h, 1, cpu),
        0xad => xra(cpu.l, 1, cpu),
        0xae => xra(get_value_memory(cpu.memory, cpu.h, cpu.l), 2, cpu),
        0xaf => xra(cpu.b, 1, cpu),

        // ORA OPS
        0xb0 => ora(cpu.b, 1, cpu),
        0xb1 => ora(cpu.c, 1, cpu),
        0xb2 => ora(cpu.d, 1, cpu),
        0xb3 => ora(cpu.e, 1, cpu),
        0xb4 => ora(cpu.h, 1, cpu),
        0xb5 => ora(cpu.l, 1, cpu),
        0xb6 => ora(get_value_memory(cpu.memory, cpu.h, cpu.l), 2, cpu),
        0xb7 => ora(cpu.b, 1, cpu),

        // CMP OPS
        0xb8 => cmp(cpu.b, 1, cpu),
        0xb9 => cmp(cpu.c, 1, cpu),
        0xba => cmp(cpu.d, 1, cpu),
        0xbb => cmp(cpu.e, 1, cpu),
        0xbc => cmp(cpu.h, 1, cpu),
        0xbd => cmp(cpu.l, 1, cpu),
        0xbe => cmp(get_value_memory(cpu.memory, cpu.h, cpu.l), 2, cpu),
        0xbf => cmp(cpu.b, 1, cpu),

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
        _ => cpu,

    }
}
