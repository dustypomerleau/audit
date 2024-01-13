use crate::{axis::Axis, sca::ScaBoundsError};

/// An agnostic cylinder type. The acceptable bounds of [`power`](Cyl::power) depend on the
/// type of power being represented. Since the acceptable values of [`axis`](Cyl::axis) are always
/// the same, we insist upon an [`Axis`](crate::axis::Axis) to constrain it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cyl {
    pub power: f32,
    pub axis: Axis,
}

// it makes sense to have the more primitive constructors take option, so we can construct
// them directly from the DB. However, if you're implementing this logic here, you don't need
// to repeat it, just call Cyl::new in Sca::new.
impl Cyl {
    pub fn new(power: Option<f32>, axis: Option<i32>) -> Result<Self, ScaBoundsError> {
        match (power, axis) {
            (Some(power), Some(axis)) => {
                if let Some(axis) = Axis::new(axis) {
                    Ok(Self { power, axis })
                } else {
                    Err(ScaBoundsError::Axis(axis))
                }
            }

            (Some(power), _) => Err(ScaBoundsError::NoPair(CylPair::Axis)),

            (..) => Err(ScaBoundsError::NoPair(CylPair::Power)),
        }
    }
}

/// Representation of missing cylinder components for use by error types.
#[derive(Debug, PartialEq)]
pub enum CylPair {
    Power,
    Axis,
}
