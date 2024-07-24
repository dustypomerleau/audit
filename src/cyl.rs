use crate::{axis::Axis, sca::ScaBoundsError};
use serde::{Deserialize, Serialize};

/// A missing cylinder component (for use by error types).
#[derive(Debug, PartialEq)]
pub enum CylPair {
    Power,
    Axis,
}

/// An agnostic cylinder type. The acceptable bounds of [`power`](Cyl::power) depend on the
/// type of power being represented. Since the acceptable values of [`axis`](Cyl::axis) are always
/// the same, we insist upon an [`Axis`](crate::axis::Axis) to constrain it.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Cyl {
    pub power: f32,
    pub axis: Axis,
}

// When we call `unwrap_or_default()`, we always want a value, so replace None with 0s.
impl Default for Cyl {
    fn default() -> Self {
        Cyl {
            power: 0.0,
            axis: Axis::new(0).expect("default axis to be created"),
        }
    }
}

impl Cyl {
    /// Create a new [`Cyl`], with bounds checking on the [`axis`](Cyl::axis). The
    /// [`power`](Cyl::power) is unconstrained until the [`Cyl`] is wrapped by a more specific
    /// type.
    pub fn new(power: f32, axis: i32) -> Result<Self, ScaBoundsError> {
        if let Some(axis) = Axis::new(axis) {
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
                axis: Axis::new(30).unwrap()
            })
        );
    }

    #[test]
    fn out_of_bounds_cyl_axis_returns_err() {
        let axis = 180i32;
        let cyl = Cyl::new(6.0, axis);

        assert_eq!(cyl, Err(ScaBoundsError::Axis(axis)));
    }
}
