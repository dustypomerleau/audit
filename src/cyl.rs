use crate::{axis::Axis, sca::ScaBoundsError};

/// An agnostic cylinder type. The acceptable bounds of [`power`](Cyl::power) depend on the
/// type of power being represented. Since the acceptable values of [`axis`](Cyl::axis) are always
/// the same, we insist upon an [`Axis`](crate::axis::Axis) to constrain it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cyl {
    pub power: f32,
    pub axis: Axis,
}

impl Cyl {
    pub fn new(power: f32, axis: i32) -> Result<Self, ScaBoundsError> {
        if let Some(axis) = Axis::new(axis) {
            Ok(Self { power, axis })
        } else {
            Err(ScaBoundsError::Axis(axis))
        }
    }
}

/// A representation of missing cylinder components for use by error types.
#[derive(Debug, PartialEq)]
pub enum CylPair {
    Power,
    Axis,
}

mod tests {
    use super::*;

    #[test]
    fn makes_new_cyl() {
        let cyl = Cyl::new(2.75, 30);
        assert_eq!(
            cyl,
            Ok(Cyl {
                power: 2.75,
                axis: Axis(30)
            })
        )
    }

    #[test]
    fn out_of_bounds_cyl_axis_returns_err() {
        let cyl = Cyl::new(6.0, 180);
        assert_eq!(cyl, Err(ScaBoundsError::Axis(180)))
    }
}
