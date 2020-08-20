use crate::cpu::MEMORY_SIZE;

pub fn is_even(byte: u16) -> u8 {
    let mut y = byte ^ (byte >> 1);
    y = byte ^ (y >> 2);
    y = byte ^ (y >> 4);
    y = byte ^ (y >> 8);
    if y & 1 != 1 {
        return 1;
    }
    0
}

// This is dumb, i should use a HashMap and remove the order problem
pub fn arith_flags(answer: u16) -> (u8, u8, u8, u8) {
    let z = if answer & 0xff == 0 { 1 } else { 0 };
    let s = if answer & 0x80 != 0 { 1 } else { 0 };
    let cy = if answer > 0xff { 1 } else { 0 };
    let p = is_even(answer & 0xff);
    (z, s, cy, p)
}

pub fn get_value_memory(memory: [u8; MEMORY_SIZE], hr: u8, lr: u8) -> u8 {
    let address: u16 = (hr as u16) << 8 | lr as u16;
    memory[address as usize]
}

pub fn change_value(memory: [u8; MEMORY_SIZE], hr: u8, lr: u8, new_value: u8) -> [u8; MEMORY_SIZE] {
    let address: u16 = (hr as u16) << 8 | lr as u16;
    let mut new_memory = memory;
    memory[address as usize] = new_value;
    memory
}
