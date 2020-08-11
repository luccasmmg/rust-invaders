use crate::condition_codes::ConditionCodes;
use crate::helpers::is_even;
use std::convert::TryInto;

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

    fn arith_flags(&mut self, answer: u16) {
        self.cc.z = if (answer & 0xff) == 0 { 1 } else { 0 };
        self.cc.s = if ( answer & 0x80 != 0 ) { 1 } else { 0 };
        self.cc.cy = if ( answer > 0xff ) { 1 } else { 0 };
        self.cc.p = is_even(answer & 0xff);
    }

    fn add(&mut self, addendum: u8) {
        let answer: u16 = (self.a as u16).wrapping_add(addendum as u16);
        self.arith_flags(answer);
        self.a = answer.to_le_bytes()[0];
    }

    fn sub(&mut self, subtract: u8) {
        let answer: u16 = (self.a as u16).wrapping_sub(subtract as u16);
        self.arith_flags(answer);
        self.a = answer.to_le_bytes()[0];
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
            //TODO 0x02 - 0x05
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
                let value: u16 = (opcode[2] as u16) << 8 | opcode[1];
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
            0x40 => (),
            0x41 => {
                self.b = self.c;
            }
            0x42 => {
                self.b = self.d;
            }
            0x43 => {
                self.b = self.e;
            }
            0x44 => {
                self.b = self.h;
            }
            0x45 => {
                self.b = self.l;
            }
            0x46 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.b = self.memory[address as usize];
            }
            0x47 => {
                self.b = self.a;
            }
            0x48 => {
                self.c = self.b;
            }
            0x4a => {
                self.c = self.d;
            }
            0x4b => {
                self.c = self.e;
            }
            0x4c => {
                self.c = self.h;
            }
            0x4d => {
                self.c = self.l;
            }
            0x4e => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.c = self.memory[address as usize];
            }
            0x4f => {
                self.c = self.a;
            }
            0x50 => {
                self.d = self.b;
            }
            0x51 => {
                self.d = self.c;
            }
            0x53 => {
                self.d = self.e;
            }
            0x54 => {
                self.d = self.h;
            }
            0x55 => {
                self.d = self.l;
            }
            0x56 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.d = self.memory[address as usize];
            }
            0x57 => {
                self.d = self.a;
            }
            0x58 => {
                self.e = self.b;
            }
            0x59 => {
                self.e = self.c;
            }
            0x5a => {
                self.e = self.d;
            }
            0x5b => self.pc += 1,
            0x5c => {
                self.e = self.h;
            }
            0x5d => {
                self.e = self.l;
            }
            0x5e => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.e = self.memory[address as usize];
            }
            0x5f => {
                self.e = self.a;
            }
            0x60 => {
                self.h = self.b;
            }
            0x61 => {
                self.h = self.c;
            }
            0x62 => {
                self.h = self.d;
            }
            0x63 => {
                self.h = self.e;
            }
            0x65 => {
                self.h = self.l;
            }
            0x66 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.h = self.memory[address as usize];
            }
            0x67 => {
                self.h = self.a;
            }
            0x68 => {
                self.l = self.b;
            }
            0x69 => {
                self.l = self.c;
            }
            0x6a => {
                self.l = self.d;
            }
            0x6b => {
                self.l = self.e;
            }
            0x6c => {
                self.l = self.h;
            }
            0x6e => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.l = self.memory[address as usize];
            }
            0x6f => {
                self.l = self.a;
            }
            0x70 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.b;
            }
            0x71 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.c;
            }
            0x72 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.d;
            }
            0x73 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.e;
            }
            0x74 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.h;
            }
            0x75 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.l;
            }
            0x76 => (), //TODO
            0x77 => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.memory[address as usize] = self.a;
            }
            0x78 => {
                self.a = self.b;
            }
            0x79 => {
                self.a = self.c;
            }
            0x7a => {
                self.a = self.d;
            }
            0x7b => {
                self.a = self.e;
            }
            0x7c => {
                self.a = self.h;
            }
            0x7d => {
                self.a = self.l;
            }
            0x7e => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                self.a = self.memory[address as usize];
            }
            0x7f => (),
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
            0x8e => {
                let addendum = (self.l).wrapping_add(self.cc.cy);
                self.add(addendum);
            }
            0x8e => {
                let address: u16 = (self.h as u16) << 8 | self.l as u16;
                let addendum = (self.memory[address as usize]).wrapping_add(self.cc.cy);
                self.add(addendum);
            }
            0x8f => {
                let addendum = (self.b).wrapping_add(self.cc.cy);
                self.add(self.a);
            }
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
            0xeb => {
                let h = self.h;
                let l = self.l;
                self.h = self.d;
                self.l = self.e;
                self.d = h;
                self.e = l;
            }
            _ => (),
        }
    }
}
