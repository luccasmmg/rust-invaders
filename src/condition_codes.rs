use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ConditionCodes {
    pub z: u8,
    pub s: u8,
    pub p: u8,
    pub cy: u8,
    pub ac: u8,
}

impl ConditionCodes {
    pub fn new() -> ConditionCodes {
        ConditionCodes {
            z: 0,
            s: 0,
            p: 0,
            cy: 0,
            ac: 0,
        }
    }
}

impl fmt::Display for ConditionCodes {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}{}{}",
                self.z, self.s, self.p, self.cy)
        }
}
