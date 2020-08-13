pub fn is_even(byte: u16) -> u8 {
    let mut y = byte ^ (byte>>1);
    y = byte ^ (y>>2);
    y = byte ^ (y>>4);
    y = byte ^ (y>>8);
    if y & 1 != 1 {
       return 1
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
