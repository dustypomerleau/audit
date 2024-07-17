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

/// A wrapper type, to ensure that near visual acuities are only compared to other near acuities.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NearVa(pub Va);

/// A Snellen-style fractional visual acuity, with numerator and denominator. Units are not
/// specified, but both fields must be in the same unit.  
///
/// The type of vision chart is left to the surgeon's discretion, but is presumed to be a Snellen,
/// ETDRS, or similar chart that provides fractional equivalents.
///
/// The [`Va`] is assumed to be a distance visual acuity, unless wrapped by [`NearVa`].
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct Va {
    pub num: f32,
    pub den: f32,
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

    /// Creates a new [`Va`] from optional values, with bounds checking.
    pub fn try_new(num: Option<f32>, den: Option<f32>) -> Result<Option<Self>, VaBoundsError> {
        match (num, den) {
            (Some(num), Some(den)) => Some(Va::new(num, den)).transpose(),

            (None, None) => Ok(None),

            (Some(_num), _) => Err(VaBoundsError::NoPair(VaPair::Denominator).into()),

            (_, Some(_den)) => Err(VaBoundsError::NoPair(VaPair::Numerator).into()),
        }
    }
}

/// A collection of visual acuities from before surgery. We use separate structs for [`BeforeVaSet`]
/// and [`AfterVaSet`], because we enforce different mandatory fields for the two situations.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct BeforeVaSet {
    pub best_far: Va,
}

/// A collection of visual acuities from after surgery. We use separate structs for [`BeforeVaSet`]
/// and [`AfterVaSet`], because we enforce different mandatory fields for the two situations.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AfterVaSet {
    pub best_far: Option<Va>,
    pub raw_far: Va,
    pub raw_near: Option<NearVa>,
}

/// The visual acuity sets from before and after a particular [`Case`](crate::case::Case).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OpVa {
    pub before: BeforeVaSet,
    pub after: AfterVaSet,
}

#[cfg(test)]
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
