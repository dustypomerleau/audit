use crate::powers::{IOL_CYL_POWERS, IOL_SE_POWERS};

pub struct IolSe(f32);

impl IolSe {
    pub fn new(se: f32) -> Option<Self> {
        if IOL_SE_POWERS.contains(&se) {
            Some(Self(se))
        } else {
            None
        }
    }
}

pub struct IolCyl(f32);

impl IolCyl {
    pub fn new(cyl: f32) -> Option<Self> {
        if IOL_CYL_POWERS.contains(&cyl) {
            Some(Self(cyl))
        } else {
            None
        }
    }
}

pub struct Iol {
    se: IolSe,
    cyl: Option<IolCyl>,
}
