use crate::cyl::Cyl;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The error type for an invalid [`Sia`].
#[derive(Debug, Error, PartialEq)]
pub enum SiaBoundsError {
    #[error("SIA must be a value between 0 D and 2 D (supplied value: {0})")]
    Sia(f32),
}

/// A surgically-induced astigmatism.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Sia(pub Cyl);

impl TryFrom<Cyl> for Sia {
    type Error = SiaBoundsError;

    fn try_from(cyl: Cyl) -> Result<Self, Self::Error> {
        if (0.0..=2.0).contains(&cyl.power) {
            Ok(Self(cyl))
        } else {
            Err(SiaBoundsError::Sia(cyl.power))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axis::Axis;

    #[test]
    fn sia_implements_try_from_cyl() {
        let sia: Sia = Cyl::new(0.1, 100).unwrap().try_into().unwrap();

        assert_eq!(
            sia,
            Sia(Cyl {
                power: 0.1,
                axis: Axis(100)
            })
        )
    }

    #[test]
    fn out_of_bounds_sia_power_returns_err() {
        let power = 2.1;
        let sia: Result<Sia, SiaBoundsError> = Cyl::new(power, 100).unwrap().try_into();

        assert_eq!(sia, Err(SiaBoundsError::Sia(power)))
    }
}
