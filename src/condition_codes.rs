use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Flags {
    pub z: u8,
    pub s: u8,
    pub p: u8,
    pub cy: u8,
    pub ac: u8,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            z: 0,
            s: 0,
            p: 0,
            cy: 0,
            ac: 0,
        }
    }
}

impl fmt::Display for Flags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}{}{}",
                self.z, self.s, self.p, self.cy)
        }
}
