/// A Snellen-style fractional visual acuity, with numerator and denominator. Units are not
/// specified, but both fields must be in the same unit.  
///
/// The type of vision chart is left to the surgeon's discretion, but is presumed to be a Snellen,
/// ETDRS, or similar chart that provides fractional equivalents.
#[derive(Debug, PartialEq)]
pub enum Va {
    OutOfBounds(BadVa),
    Distance { num: f32, den: f32 },
    Near { num: f32, den: f32 },
}

/// A helper enum for specifying the kind of `Va` you want to generate with `Va::new()`.
pub enum VaKind {
    Distance,
    Near,
}

/// A helper enum for specifying the out of bounds value when `Va::new()` returns
/// `Va::OutOfBounds`.
pub enum BadVa {
    Num,
    Den,
}

impl Va {
    pub fn new(kind: VaKind, num: f32, den: f32) -> Self {
        if (0.1..=20.0).contains(&num) {
            if den > 0.0 {
                match kind {
                    VaKind::Distance => Self::Distance { num, den },
                    VaKind::Near => Self::Near { num, den },
                }
            } else {
                Self::OutOfBounds(BadVa::Den)
            }
        } else {
            Self::OutOfBounds(BadVa::Num)
        }
    }
}

#[derive(Debug, PartialEq)]
struct VaSet {
    before: Va,
    after: Va,
}

/// A collection of preoperative and postoperative visual acuity measurements for a given case.
/// The `VaSet` for best-corrected distance is mandatory. Other fields are optional.
// todo: perhaps we want a new() function that ensures the correct enum variants for each VaSet
// in OpVa
#[derive(Debug, PartialEq)]
pub struct OpVa {
    best_distance: VaSet,
    best_near: Option<VaSet>,
    raw_distance: Option<VaSet>,
    raw_near: Option<VaSet>,
}
