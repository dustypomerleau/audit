use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaBoundsError {
    #[error("Va numerator must be between 0.1 and 20.0. {0} was supplied")]
    Num(f32),
    #[error("Va denominator must be > 0. {0} was supplied")]
    Den(f32),
}

/// A Snellen-style fractional visual acuity, with numerator and denominator. Units are not
/// specified, but both fields must be in the same unit.  
///
/// The type of vision chart is left to the surgeon's discretion, but is presumed to be a Snellen,
/// ETDRS, or similar chart that provides fractional equivalents.
#[derive(Debug, PartialEq)]
pub struct Va {
    num: f32,
    den: f32,
}

impl Va {
    pub fn new(num: f32, den: f32) -> Result<Self, VaBoundsError> {
        if (0.1..=20.0).contains(&num) {
            if den > 0.0 {
                Ok(Self { num, den })
            } else {
                Err(VaBoundsError::Den(den))
            }
        } else {
            Err(VaBoundsError::Num(num))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Distance(Va);

#[derive(Debug, PartialEq)]
pub struct DistanceSet {
    before: Distance,
    after: Distance,
}

#[derive(Debug, PartialEq)]
pub struct Near(Va);

#[derive(Debug, PartialEq)]
pub struct NearSet {
    before: Near,
    after: Near,
}

/// A collection of preoperative and postoperative visual acuity measurements for a given case.
/// The `VaSet` for best-corrected distance is mandatory. Other fields are optional.
// todo: perhaps we want a new() function that ensures the correct enum variants for each VaSet
// in OpVa
#[derive(Debug, PartialEq)]
pub struct OpVa {
    best_distance: DistanceSet,
    best_near: Option<NearSet>,
    raw_distance: Option<DistanceSet>,
    raw_near: Option<NearSet>,
}
