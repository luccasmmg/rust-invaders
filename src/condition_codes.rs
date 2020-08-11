pub struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
    pub pad: u8,
}

impl ConditionCodes {
    pub fn new() -> ConditionCodes {
        ConditionCodes {
            z: false,
            s: false,
            p: false,
            cy: false,
            ac: false,
            pad: 3,
        }
    }
}
