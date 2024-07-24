use crate::{axis::Axis, cyl::Cyl};
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
pub struct Sia {
    power: f32,
    axis: Axis,
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

        if (0.0..=2.0).contains(&power) {
            Ok(Self { power, axis })
        } else {
            Err(SiaBoundsError::Sia(power))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axis::Axis;

    #[test]
    fn out_of_bounds_sia_power_returns_err() {
        let power = 2.1;
        let cyl = Cyl::new(power, 100).unwrap();
        let sia = Sia::new(cyl);

        assert_eq!(sia, Err(SiaBoundsError::Sia(power)));
    }
}
