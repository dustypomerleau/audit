#[cfg(feature = "ssr")] use edgedb_derive::Queryable;
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
///
/// Values are represented as `((entered float) * 100) as u32`, and stored in the DB as `i32` with
/// constraints. This makes the representation consistent with [`Cyl`](crate::cyl::Cyl),
/// [`Iol`](crate::iol::Iol), [`Refraction`](crate::refraction::Refraction), and
/// [`Target`](crate::target::Target).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct Va {
    pub num: u32,
    pub den: u32,
}

impl Va {
    /// Creates a new [`Va`] with bounds checking.
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
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct BeforeVa {
    pub best: Va,
    pub raw: Option<Va>,
}

/// A collection of visual acuities from after surgery. We use separate structs for [`BeforeVa`]
/// and [`AfterVa`], because we enforce different mandatory fields for the two situations.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct AfterVa {
    pub best: Option<Va>,
    pub raw: Va,
}

/// The visual acuity sets from before and after a particular [`Case`](crate::case::Case).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct OpVa {
    pub before: BeforeVa,
    pub after: AfterVa,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_new_va() {
        let va = Va::new(6.0, 7.5).unwrap();

        assert_eq!(va, Va { num: 6.0, den: 7.5 });
    }

    #[test]
    fn out_of_bounds_va_numerator_returns_err() {
        let num = 21.2;
        let va = Va::new(num, 9.0);

        assert_eq!(va, Err(VaBoundsError::Num(num)));
    }

    #[test]
    fn out_of_bounds_va_denominator_returns_err() {
        let den = -1.2;
        let va = Va::new(6.0, den);

        assert_eq!(va, Err(VaBoundsError::Den(den)));
    }
}
