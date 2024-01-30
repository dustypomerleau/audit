use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A representation of a missing member of a fractional visual acuity, for use by error types.
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

/// A wrapper type, to ensure that far visual acuities are only compared to other far acuities.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Far(Va);

/// A wrapper type, to ensure that near visual acuities are only compared to other near acuities.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Near(Va);

/// A Snellen-style fractional visual acuity, with numerator and denominator. Units are not
/// specified, but both fields must be in the same unit.  
///
/// The type of vision chart is left to the surgeon's discretion, but is presumed to be a Snellen,
/// ETDRS, or similar chart that provides fractional equivalents.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Va {
    pub num: f32,
    pub den: f32,
}

impl Va {
    /// Creates a new visual acuity with bounds checking.
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

/// A collection of visual acuities from before surgery. We use separate structs for [`BeforeVa`]
/// and [`AfterVa`], because we enforce different mandatory fields for the two situations.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BeforeVa {
    best_far: Far<Va>,
    best_near: Option<Near<Va>>,
    raw_far: Option<Far<Va>>,
    raw_near: Option<Near<Va>>,
}

/// A collection of visual acuities from after surgery. We use separate structs for [`BeforeVa`]
/// and [`AfterVa`], because we enforce different mandatory fields for the two situations.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AfterVa {
    best_far: Option<Far<Va>>,
    best_near: Option<Near<Va>>,
    raw_far: Far<Va>,
    raw_near: Option<Near<Va>>,
}

/// The visual acuity sets from before and after a particular [`Case`](crate::case::Case).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OpVa {
    before: BeforeVa,
    after: AfterVa,
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
        let num = 21.2;
        let va = Va::new(num, 9.0);

        assert_eq!(va, Err(VaBoundsError::Num(num)))
    }

    #[test]
    fn out_of_bounds_va_denominator_returns_err() {
        let den = -1.2;
        let va = Va::new(6.0, den);

        assert_eq!(va, Err(VaBoundsError::Den(den)))
    }
}
