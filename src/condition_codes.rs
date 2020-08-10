pub struct ConditionCodes {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
    pad: u8,
}

impl ConditionCodes {
    pub fn new() -> ConditionCodes {
        ConditionCodes {
            z: 1,
            s: 1,
            p: 1,
            cy: 1,
            ac: 1,
            pad: 3,
        }
    }
}
