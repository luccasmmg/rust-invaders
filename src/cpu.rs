#![allow(dead_code)]
use crate::condition_codes::ConditionCodes;
use crate::helpers::is_even;
use crate::op_data_transfer::*;
use crate::op_arithmetic::*;
use crate::op_logical::*;

const MEMORY_SIZE: usize = 0x4000;

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

// This is dumb, i should use a HashMap and remove the order problem
fn arith_flags(answer: u16) -> (u8, u8, u8, u8) {
    let z = if answer & 0xff == 0 { 1 } else { 0 };
    let s = if answer & 0x80 != 0 { 1 } else { 0 };
    let cy = if answer > 0xff { 1 } else { 0 };
    let p = is_even(answer & 0xff);
    (z, s, cy, p)
}

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
