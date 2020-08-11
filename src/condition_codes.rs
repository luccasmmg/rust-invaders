pub struct ConditionCodes {
    pub z: u8,
    pub s: u8,
    pub p: u8,
    pub cy: u8,
    pub ac: u8,
    pub pad: u8,
}

impl ConditionCodes {
    pub fn new() -> ConditionCodes {
        ConditionCodes {
            z: 0,
            s: 0,
            p: 0,
            cy: 0,
            ac: 0,
            pad: 3,
        }
    }
}
