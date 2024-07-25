use crate::sca::ScaBoundsError;
use serde::{Deserialize, Serialize};

/// A missing cylinder component (for use by error types).
#[derive(Debug, PartialEq)]
pub enum CylPair {
    Power,
    Axis,
}

/// A generic cylinder type. The acceptable bounds of [`power`](Cyl::power) depend on the
/// type of power being represented. Since the acceptable values of [`axis`](Cyl::axis) are always
/// the same, we constrain the axis field when constructing a [`Cyl`]. The
/// [`power`](Cyl::power) and [`axis`](Cyl::axis) fields have been left public for purposes of
/// pattern-matching, but [`Cyl::new()`] should be used when instantiating a new [`Cyl`], in
/// order to take advantage of bounds checking on the [`axis`](Cyl::axis).
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Cyl {
    pub power: f32,
    pub axis: u32,
}

// When we call `unwrap_or_default()`, we always want a value, so replace None with 0s.
impl Default for Cyl {
    fn default() -> Self {
        Cyl {
            power: 0.0,
            axis: 0,
        }
    }
}

impl Cyl {
    /// Create a new [`Cyl`], with bounds checking on the [`axis`](Cyl::axis). The
    /// [`power`](Cyl::power) is unconstrained until the [`Cyl`] is wrapped by a more specific
    /// type.
    pub fn new(power: f32, axis: u32) -> Result<Self, ScaBoundsError> {
        if axis < 180 {
            Ok(Self { power, axis })
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
        let cyl = Cyl::new(2.75, 30);

        assert_eq!(
            cyl,
            Ok(Cyl {
                power: 2.75,
                axis: 30
            })
        );
    }

    #[test]
    fn out_of_bounds_cyl_axis_returns_err() {
        let axis = 180;
        let cyl = Cyl::new(6.0, axis);

        assert_eq!(cyl, Err(ScaBoundsError::Axis(axis)));
    }
}
