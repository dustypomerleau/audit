use crate::cyl::Cyl;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The error type for an invalid [`Sia`].
#[derive(Debug, Error, PartialEq)]
pub enum SiaBoundsError {
    #[error("SIA must be a value between 0 D and 2 D (supplied value: {0})")]
    Sia(i32),
}

/// A surgically-induced astigmatism. The purist would prefer using
/// `meridian` rather than `axis` for [`Sia`] and biometric Ks, but on balance I've
/// decided that the cognitive overhead of using both terms in the code is higher than the cognitive
/// overhead of knowing when `axis` actually refers to a meridian.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Sia {
    pub power: i32,
    pub axis: i32,
}

impl TryFrom<Cyl> for Sia {
    type Error = SiaBoundsError;

    fn try_from(cyl: Cyl) -> Result<Self, Self::Error> {
        Self::new(cyl)
    }
}

impl Sia {
    /// Create a new bounds-checked [`Sia`] from a generic [`Cyl`].
    pub fn new(cyl: Cyl) -> Result<Self, SiaBoundsError> {
        let Cyl { power, axis } = cyl;

        if (0..=200).contains(&power) {
            Ok(Self { power, axis })
        } else {
            Err(SiaBoundsError::Sia(power))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_new_sia() {
        let cyl = Cyl::new(10, 100).unwrap();
        Sia::new(cyl).unwrap();
    }

    #[test]
    fn out_of_bounds_sia_power_returns_err() {
        let power = 210;
        let cyl = Cyl::new(power, 100).unwrap();
        let sia = Sia::new(cyl);

        assert_eq!(sia, Err(SiaBoundsError::Sia(power)));
    }
}
