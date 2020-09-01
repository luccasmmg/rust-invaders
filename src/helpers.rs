pub fn parity(byte: u16) -> u16 {
    let mut y = byte;
    y ^= y >> 4;
    y ^= y >> 2;
    y ^= y >> 1;
    (!y) & 1
}

// This is dumb, i should use a HashMap and remove the order problem
pub fn arith_flags(answer: u16) -> (u8, u8, u8, u8) {
    let cy = if answer > 0xff { 1 } else { 0 };
    let z = if answer & 0xff == 0 { 1 } else { 0 };
    let s = if answer & 0x80 == 0x80 { 1 } else { 0 };
    let p = if parity(answer & 0xff) == 1 { 1 } else { 0 };
    (z, s, cy, p)
}

pub fn arith_flags_logs(answer: u16) -> (u8, u8, u8, u8) {
    let cy = if answer > 0xff { 1 } else { 0 };
    let z = if answer == 0 { 1 } else { 0 };
    let s = if answer & (1 << 7) != 0 { 1 } else { 0 };
    let p = if parity(answer & 0xff) == 1 { 1 } else { 0 };
    (z, s, cy, p)
}

pub fn get_value_memory(memory: &Vec<u8>, hr: u8, lr: u8) -> u8 {
    let address: u16 = (hr as u16) << 8 | lr as u16;
    memory[address as usize]
}
