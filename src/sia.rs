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

impl Sia {
    pub fn new(power: f32, axis: Axis) -> Result<Self, SiaBoundsError> {
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
        let sia = Sia::new(power, Axis(100));

        assert_eq!(sia, Err(SiaBoundsError::Sia(power)))
    }
}
