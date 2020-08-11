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
