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
            memory: [0; MEMORY_SIZE],
            cc: ConditionCodes::new(),
            int_enable: 0,
        }
    }

    fn arith_flags(&mut self, answer: u16, cy_affected: bool) {
        self.cc.z = if answer & 0xff == 0 { 1 } else { 0 };
        self.cc.s = if answer & 0x80 != 0 { 1 } else { 0 };
        if cy_affected {
            self.cc.cy = if answer > 0xff { 1 } else { 0 };
        }
        self.cc.p = is_even(answer & 0xff);
    }

    fn add(&mut self, addendum: u8) {
        let answer: u16 = (self.a as u16).wrapping_add(addendum as u16);
        self.arith_flags(answer, true);
        self.a = answer.to_le_bytes()[0];
    }

    fn inr(&mut self, register: u8) -> u8 {
        let answer: u16 = (register as u16).wrapping_add(1 as u16);
        self.arith_flags(answer, true);
        answer as u8
    }

    fn sub(&mut self, subtract: u8) {
        let answer: u16 = (self.a as u16).wrapping_sub(subtract as u16);
        self.arith_flags(answer, true);
        self.a = answer.to_le_bytes()[0];
    }

    fn dcr(&mut self, register: u8) -> u8 {
        let answer: u16 = (register as u16).wrapping_sub(1 as u16);
        self.arith_flags(answer, true);
        answer as u8
    }

    fn inx(&self, rh: u8, rl: u8) -> (u8, u8) {
        let result = (((rh as u16) << 8 | rl as u16).wrapping_add(1 as u16)).to_be_bytes();
        (result[0], result[1])
    }

    fn dcx(&self, rh: u8, rl: u8) -> (u8, u8) {
        let result = (((rh as u16) << 8 | rl as u16).wrapping_sub(1 as u16)).to_be_bytes();
        (result[0], result[1])
    }

    fn dad(&mut self, rh: u8, rl: u8) {
        let hl = (self.h as u16) << 8 | self.l as u16;
        let register_sum = (rh as u16) << 8 | rl as u16;
        let answer = hl + register_sum;
        self.h = answer.to_be_bytes()[0];
        self.l = answer.to_be_bytes()[0];
        self.cc.cy = if (answer & 0xff) != 0 { 1 } else { 0 };
    }

    fn unimplemented_instruction(&self) {
        panic!("Error: Unimplemented instruction\n")
    }

    fn emulate_8080_op(&mut self) {
        let pc: usize = self.pc as usize;
        let opcode = &self.memory[pc..];
        self.pc += 1;
        match opcode[0] {
            0x00 => (),
            0x01 => {
                self.c = opcode[1];
                self.b = opcode[2];
                self.pc += 2
            }
            0x02 => {
                let address: u16 = (self.b as u16) << 8 | self.c as u16;
                self.memory[address as usize] = self.a;
            }

            // INX OPS
            0x03 => { // INX B
                let result = self.inx(self.b, self.c);
                self.b = result.0;
                self.c = result.1;
            }
            0x13 => { // INX D
                let result = self.inx(self.d, self.e);
                self.d = result.0;
                self.e = result.1;
            }
            0x23 => { // INX H
                let result = self.inx(self.h, self.l);
                self.h = result.0;
                self.l = result.1;
            }
            0x33 => { // INX SP
                let divided_sp = self.pc.to_be_bytes();
                let result = self.inx(divided_sp[0], divided_sp[1]);
                self.pc = (result.0 as u16) << 8 | result.1 as u16;
            }

            // DCX OPS
            0x0b => { // DCX B
                let result = self.dcx(self.b, self.c);
                self.b = result.0;
                self.c = result.1;
            }
            0x1b => { // DCX D
                let result = self.dcx(self.d, self.e);
                self.d = result.0;
                self.e = result.1;
            }
            0x2b => { // DCX H
                let result = self.dcx(self.h, self.l);
                self.h = result.0;
                self.l = result.1;
            }
            0x3b => { // DCX SP
                let divided_sp = self.pc.to_be_bytes();
                let result = self.dcx(divided_sp[0], divided_sp[1]);
                self.pc = (result.0 as u16) << 8 | result.1 as u16;
            }

            //DAD OPS
            0x09 => {
                self.dad(self.b, self.c);
            }
            0x19 => {
                self.dad(self.d, self.e);
            }
            0x29 => {
                self.dad(self.h, self.l);
            }
            0x39 => {
                let divided_sp = self.sp.to_be_bytes();
                self.dad(divided_sp[0], divided_sp[1]);
            }

            // INR OPS
            0x04 => {
                self.b = self.inr(self.b);
            }
            0x0c => {
                self.c = self.inr(self.c);
            }
            0x14 => {
                self.d = self.inr(self.d);
            }
            0x1c => {
                self.e = self.inr(self.e);
            }
            0x24 => {
                self.h = self.inr(self.h);
            }
            0x2c => {
                self.l = self.inr(self.l);
            }
            0x34 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.inr(self.memory[address as usize]);
            }

            // DCR OPS
            0x05 => {
                self.b = self.dcr(self.b);
            }
            0x0d => {
                self.c = self.dcr(self.c);
            }
            0x15 => {
                self.d = self.dcr(self.d);
            }
            0x1d => {
                self.e = self.dcr(self.e);
            }
            0x25 => {
                self.h = self.dcr(self.h);
            }
            0x2d => {
                self.l = self.dcr(self.l);
            }
            0x35 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.dcr(self.memory[address as usize]);
            }

            0x06 => {
                self.b = opcode[1];
                self.pc += 1;
            }
            0x0a => {
                let address: u16 = (self.b as u16) << 8 | self.c as u16;
                self.a = self.memory[address as usize];
            }
            0x0e => {
                self.c = opcode[1];
                self.pc += 1;
            }
            0x11 => {
                self.d = opcode[1];
                self.e = opcode[2];
                self.pc += 2;
            }
            0x12 => {
                let address: u16 = (self.d as u16) << 8 | self.e as u16;
                self.memory[address as usize] = self.a;
            }
            0x16 => {
                self.d = opcode[1];
                self.pc += 1;
            }
            0x1a => {
                let address: u16 = (self.d as u16) << 8 | self.e as u16;
                self.a = self.memory[address as usize];
            }
            0x1e => {
                self.e = opcode[1];
                self.pc += 1;
            }
            0x21 => {
                self.h = opcode[1];
                self.l = opcode[2];
                self.pc += 2;
            }
            0x22 => {
                let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
                self.memory[address as usize] = self.l;
                self.memory[(address + 1) as usize] = self.l;
                self.pc += 2;
            }
            0x26 => {
                self.h = opcode[1];
                self.pc += 1;
            }
            0x2a => {
                let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
                self.l = self.memory[address as usize];
                self.h = self.memory[(address + 1) as usize];
                self.pc += 2;
            }
            0x2e => {
                self.l = opcode[1];
                self.pc += 1;
            }
            0x36 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = opcode[1];
                self.pc += 1;
            }
            0x31 => {
                let value: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
                self.sp = value;
                self.pc += 2;
            }
            0x32 => {
                let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
                self.memory[address as usize] = self.a;
            }
            0x3a => {
                let address: u16 = (opcode[2] as u16) << 8 | opcode[1] as u16;
                self.a = self.memory[address as usize];
            }
            0x3e => {
                self.a = opcode[1];
                self.pc += 1;
            }

            // MOV OPS
            0x40 => (), // MOV B,B
            0x41 => {
                self.b = self.c; // MOV B,B
            }
            0x42 => {
                self.b = self.d; // MOV B,D
            }
            0x43 => {
                self.b = self.e; // MOV B,E
            }
            0x44 => {
                self.b = self.h; // MOV B,H
            }
            0x45 => {
                self.b = self.l; // MOV B,L
            }
            0x46 => { // MOV B,M
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.b = self.memory[address as usize];
            }
            0x47 => { // MOV B,A
                self.b = self.a;
            }
            0x48 => { // MOV C B
                self.c = self.b;
            }
            0x4a => { // MOV C D
                self.c = self.d;
            }
            0x4b => { // MOV C E
                self.c = self.e;
            }
            0x4c => { // MOV C H
                self.c = self.h;
            }
            0x4d => { // MOV C L
                self.c = self.l;
            }
            0x4e => { // MOV C M
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.c = self.memory[address as usize];
            }
            0x4f => { // MOV C A
                self.c = self.a;
            }
            0x50 => { // MOV D B
                self.d = self.b;
            }
            0x51 => { // MOV D C
                self.d = self.c;
            }
            0x53 => { // MOV D E
                self.d = self.e;
            }
            0x54 => { // MOV D H
                self.d = self.h;
            }
            0x55 => { // MOV D L
                self.d = self.l;
            }
            0x56 => { // MOV D M
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.d = self.memory[address as usize];
            }
            0x57 => { // MOV D A
                self.d = self.a;
            }
            0x58 => { // MOV E B
                self.e = self.b;
            }
            0x59 => { // MOV E C
                self.e = self.c;
            }
            0x5a => { // MOV E D
                self.e = self.d;
            }
            0x5c => { // MOV E H
                self.e = self.h;
            }
            0x5d => { // MOV E L
                self.e = self.l;
            }
            0x5e => { // MOV E M
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.e = self.memory[address as usize];
            }
            0x5f => { // MOV E A
                self.e = self.a;
            }
            0x60 => { // MOV H B
                self.h = self.b;
            }
            0x61 => { // MOV H C
                self.h = self.c;
            }
            0x62 => { // MOV H D
                self.h = self.d;
            }
            0x63 => { // MOV H E
                self.h = self.e;
            }
            0x65 => { // MOV H L
                self.h = self.l;
            }
            0x66 => { // MOV H M
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.h = self.memory[address as usize];
            }
            0x67 => { // MOV H A
                self.h = self.a;
            }
            0x68 => { // MOV L B
                self.l = self.b;
            }
            0x69 => { // MOV L C
                self.l = self.c;
            }
            0x6a => { // MOV L D
                self.l = self.d;
            }
            0x6b => { // MOV L E
                self.l = self.e;
            }
            0x6c => { // MOV L H
                self.l = self.h;
            }
            0x6e => { // MOV L M
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.l = self.memory[address as usize];
            }
            0x6f => { // MOV L A
                self.l = self.a;
            }
            0x70 => { // MOV M B
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.b;
            }
            0x71 => { // MOV M C
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.c;
            }
            0x72 => { // MOV M D
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.d;
            }
            0x73 => { // MOV M E
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.e;
            }
            0x74 => { // MOV M H
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.h;
            }
            0x75 => { // MOV M L
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.l;
            }
            0x76 => (), //TODO
            0x77 => { // MOV M A
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.a;
            }
            0x78 => { // MOV A B
                self.a = self.b;
            }
            0x79 => { // MOV A C
                self.a = self.c;
            }
            0x7a => { // MOV A D
                self.a = self.d;
            }
            0x7b => { // MOV A E
                self.a = self.e;
            }
            0x7c => { // MOV A H
                self.a = self.h;
            }
            0x7d => { // MOV A L
                self.a = self.l;
            }
            0x7e => { // MOV A M
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.a = self.memory[address as usize];
            }
            0x7f => (),

            // ADD OPS
            0x80 => {
                self.add(self.b);
            }
            0x81 => {
                self.add(self.c);
            }
            0x82 => {
                self.add(self.d);
            }
            0x83 => {
                self.add(self.e);
            }
            0x84 => {
                self.add(self.h);
            }
            0x85 => {
                self.add(self.l);
            }
            0x86 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.add(self.memory[address as usize]);
            }
            0x87 => {
                self.add(self.a);
            }

            // ADC OPS
            0x88 => {
                let addendum = (self.b).wrapping_add(self.cc.cy);
                self.add(addendum);
            }
            0x89 => {
                let addendum = (self.c).wrapping_add(self.cc.cy);
                self.add(addendum);
            }
            0x8a => {
                let addendum = (self.d).wrapping_add(self.cc.cy);
                self.add(addendum);
            }
            0x8b => {
                let addendum = (self.e).wrapping_add(self.cc.cy);
                self.add(addendum);
            }
            0x8c => {
                let addendum = (self.h).wrapping_add(self.cc.cy);
                self.add(addendum);
            }
            0x8d => {
                let addendum = (self.l).wrapping_add(self.cc.cy);
                self.add(addendum);
            }
            0x8e => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                let addendum = (self.memory[address as usize]).wrapping_add(self.cc.cy);
                self.add(addendum);
            }
            0x8f => {
                let addendum = (self.a).wrapping_add(self.cc.cy);
                self.add(addendum);
            }

            // SUB OPS
            0x90 => {
                self.sub(self.b);
            }
            0x91 => {
                self.sub(self.c);
            }
            0x92 => {
                self.sub(self.d);
            }
            0x93 => {
                self.sub(self.e);
            }
            0x94 => {
                self.sub(self.h);
            }
            0x95 => {
                self.sub(self.l);
            }
            0x96 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.sub(self.memory[address as usize]);
            }
            0x97 => {
                self.sub(self.a);
            }

            // SUBB OPS
            0x98 => {
                let subtraend = (self.b).wrapping_sub(self.cc.cy);
                self.sub(subtraend);
            }
            0x99 => {
                let subtraend = (self.c).wrapping_sub(self.cc.cy);
                self.sub(subtraend);
            }
            0x9a => {
                let subtraend = (self.d).wrapping_sub(self.cc.cy);
                self.sub(subtraend);
            }
            0x9b => {
                let subtraend = (self.e).wrapping_sub(self.cc.cy);
                self.sub(subtraend);
            }
            0x9c => {
                let subtraend = (self.h).wrapping_sub(self.cc.cy);
                self.sub(subtraend);
            }
            0x9d => {
                let subtraend = (self.l).wrapping_sub(self.cc.cy);
                self.sub(subtraend);
            }
            0x9e => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                let subtraend = (self.memory[address as usize]).wrapping_sub(self.cc.cy);
                self.sub(subtraend);
            }
            0x9f => {
                let subtraend = (self.a).wrapping_sub(self.cc.cy);
                self.sub(subtraend);
            }


            // ADI OPS
            0xc6 => {
                let first_byte: u8 = opcode[1];
                self.add(first_byte);
                self.pc += 1;
            }
            0xce => {
                let first_byte: u8 = opcode[1];
                self.add((first_byte).wrapping_add(self.cc.cy));
                self.pc += 1;
            }
            0xeb => {
                let h = self.h;
                let l = self.l;
                self.h = self.d;
                self.l = self.e;
                self.d = h;
                self.e = l;
            }

            // SUI OPS
            0xd6 => {
                let first_byte: u8 = opcode[1];
                self.sub(first_byte);
                self.pc += 1;
            }
            0xde => {
                let first_byte: u8 = opcode[1];
                self.sub((first_byte).wrapping_sub(self.cc.cy));
                self.pc += 1;
            }
            _ => (),
        }
    }
}
