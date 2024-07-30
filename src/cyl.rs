use crate::sca::ScaBoundsError;
use serde::{Deserialize, Serialize};

/// A missing cylinder component (for use by error types).
#[derive(Debug, PartialEq)]
pub enum CylPair {
    Power,
    Axis,
}

/// A generic cylinder type. The acceptable bounds of [`power`](Cyl::power) depend on the type of
/// power being represented. Since the acceptable values of [`axis`](Cyl::axis) are always the same,
/// we constrain the axis field when constructing a [`Cyl`]. The fields have been left public for
/// purposes of pattern-matching, but [`Cyl::new()`] should be used when instantiating a new
/// [`Cyl`], in order to take advantage of bounds checking on the [`axis`](Cyl::axis).
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Cyl {
    /// Cylinder power in `hm^-1`.
    pub power: i32,
    /// Axis in degrees.
    pub axis: u32,
}

impl Cyl {
    /// Create a new [`Cyl`], with bounds checking on the [`axis`](Cyl::axis). The
    /// [`power`](Cyl::power) is unconstrained until the [`Cyl`] is wrapped by a more specific
    /// type.
    pub fn new(power: i32, axis: u32) -> Result<Self, ScaBoundsError> {
        if axis < 180 {
            Ok(Self { power, axis })
        } else {
            Err(ScaBoundsError::Axis(axis))
        }
    }

    /// Update a [`Cyl`] with a new [`power`](Cyl::power).
    pub fn set_power(mut self, power: i32) -> Self {
        self.power = power;
        self
    }

    /// Update a [`Cyl`] with a new [`axis`](Cyl::axis).
    pub fn set_axis(mut self, axis: u32) -> Result<Self, ScaBoundsError> {
        if axis < 180 {
            self.axis = axis;
            Ok(self)
        } else {
            Err(ScaBoundsError::Axis(axis))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_new_cyl() {
        let cyl = Cyl::new(275, 30);

        assert_eq!(
            cyl,
            Ok(Cyl {
                power: 275,
                axis: 30
            })
        );
    }

    #[test]
    fn out_of_bounds_cyl_axis_returns_err() {
        let axis = 180;
        let cyl = Cyl::new(600, axis);

        assert_eq!(cyl, Err(ScaBoundsError::Axis(axis)));
    }
}
