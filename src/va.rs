use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Required values in a fractional [`Va`]
#[derive(Debug, PartialEq)]
pub enum VaPair {
    Numerator,
    Denominator,
}

/// The error type for an invalid [`Va`]
#[derive(Debug, Error, PartialEq)]
pub enum VaBoundsError {
    #[error("Va numerator must be between 100 and 2000. {0} was supplied")]
    Num(u32),

    #[error("Va denominator must be > 0. {0} was supplied")]
    Den(u32),

    #[error("visual acuity must have both a numerator and a denominator. {0:?} was not supplied.")]
    NoPair(VaPair),
}

/// A Snellen-style fractional visual acuity, with numerator and denominator. Units are not
/// specified, but both fields must be in the same unit.  
///
/// The type of vision chart is left to the surgeon's discretion, but is presumed to be a Snellen,
/// ETDRS, or similar chart that provides fractional equivalents.
///
/// Values are represented as `((entered float) * 100) as u32`, and stored in the DB as `i32` with
/// constraints. This makes the representation consistent with [`Cyl`](crate::cyl::Cyl),
/// [`Iol`](crate::iol::Iol), [`Refraction`](crate::refraction::Refraction), and
/// [`Target`](crate::target::Target).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Va {
    pub num: u32,
    pub den: u32,
}

impl Default for Va {
    fn default() -> Self {
        Self { num: 600, den: 600 }
    }
}

impl Va {
    /// Creates a new [`Va`] with bounds checking.
    pub fn new(num: u32, den: u32) -> Result<Self, VaBoundsError> {
        if num < 2000 {
            if den > 0 {
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BeforeVa {
    pub best: Va,
    pub raw: Option<Va>,
}

/// A collection of visual acuities from after surgery. We use separate structs for [`BeforeVa`]
/// and [`AfterVa`], because we enforce different mandatory fields for the two situations.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AfterVa {
    pub best: Option<Va>,
    pub raw: Va,
}

/// The visual acuity sets from before and after a particular [`Case`](crate::case::Case).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OpVa {
    pub before: BeforeVa,
    pub after: AfterVa,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_new_va() {
        let va = Va::new(600, 750).unwrap();

        assert_eq!(va, Va { num: 600, den: 750 });
    }

    #[test]
    fn out_of_bounds_va_numerator_returns_err() {
        let num = 2120;
        let va = Va::new(num, 900);

        assert_eq!(va, Err(VaBoundsError::Num(num)));
    }

    #[test]
    fn zero_va_denominator_returns_err() {
        let den = 0u32;
        let va = Va::new(600, den);

        assert_eq!(va, Err(VaBoundsError::Den(den)));
    }
}
