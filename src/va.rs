use crate::distance::{Far, Near};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum VaPair {
    Numerator,
    Denominator,
}

#[derive(Debug, Error, PartialEq)]
pub enum VaBoundsError {
    #[error("Va numerator must be between 0.1 and 20.0. {0} was supplied")]
    Num(f32),
    #[error("Va denominator must be > 0. {0} was supplied")]
    Den(f32),
    #[error("visual acuity must have both a numerator and a denominator. {0:?} was not supplied.")]
    NoPair(VaPair),
}

/// A Snellen-style fractional visual acuity, with numerator and denominator. Units are not
/// specified, but both fields must be in the same unit.  
///
/// The type of vision chart is left to the surgeon's discretion, but is presumed to be a Snellen,
/// ETDRS, or similar chart that provides fractional equivalents.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Va {
    num: f32,
    den: f32,
}

impl Va {
    pub fn new(num: f32, den: f32) -> Result<Self, VaBoundsError> {
        if (0.0..=20.0).contains(&num) && num > 0.0 {
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FarVaSet {
    pub before: Far<Va>,
    pub after: Far<Va>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NearVaSet {
    pub before: Near<Va>,
    pub after: Near<Va>,
}

/// A collection of preoperative and postoperative visual acuity measurements for a given case.
/// The `VaSet` for best-corrected far acuity is mandatory. Other fields are optional.
// todo: perhaps we want a new() function that ensures the correct enum variants for each VaSet
// in OpVa
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OpVa {
    pub best_far: FarVaSet,
    pub best_near: Option<NearVaSet>,
    pub raw_far: Option<FarVaSet>,
    pub raw_near: Option<NearVaSet>,
}

mod tests {
    use super::*;

    #[test]
    fn makes_new_va() {
        let va = Va::new(6.0, 7.5).unwrap();
        assert_eq!(va, Va { num: 6.0, den: 7.5 })
    }

    #[test]
    fn out_of_bounds_va_numerator_returns_err() {
        let num = 21.2f32;
        let va = Va::new(num, 9.0);
        assert_eq!(va, Err(VaBoundsError::Num(num)))
    }

    #[test]
    fn out_of_bounds_va_denominator_returns_err() {
        let den = -1.2f32;
        let va = Va::new(6.0, den);
        assert_eq!(va, Err(VaBoundsError::Den(den)))
    }
}
