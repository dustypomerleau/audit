// https://stackoverflow.com/questions/54048500/convert-literal-to-associated-type-in-generic-struct-implementation
// https://stackoverflow.com/questions/54504026/how-do-i-provide-an-implementation-of-a-generic-struct-in-rust

use crate::cyl::{Cyl, CylPair};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ScaBoundsError {
    #[error("cylinder must have both a power and an axis but the {0:?} was not supplied")]
    NoPair(CylPair),

    // note: this variant is needed, because it gets returned by Cyl::new - maybe we should
    // just make it an AxisBoundsError? CylBoundsError?
    #[error("cylinder axis must be an integer value between 0° and 179° (supplied value: {0})")]
    Axis(i32),
}

/// A [`Sca`] contains the sphere and cylinder values to be used in an [`Iol`](crate::iol::Iol),
/// [`Refraction`](crate::refraction::Refraction), or [`Target`](crate::target::Target),
/// but it is a more primitive type, without bounds checking for the sphere or cylinder powers.
/// [`Sca`] does have bounds checking for [`Axis`](crate::axis::Axis), because the bounds for
/// this value are always the same. Bounds checking of sphere and cylinder powers is performed
/// during conversion to the above types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sca {
    pub sph: f32,
    pub cyl: Option<Cyl>,
}

// The general approach is to build any Sca derivative using try_from() if there are no
// additional fields (like Iol), and a custom new() if there are additional fields (like
// Target).
impl Sca {
    pub fn new(sph: f32, cyl: Option<f32>, axis: Option<i32>) -> Result<Self, ScaBoundsError> {
        match (cyl, axis) {
            (None, None) => Ok(Self { sph, cyl: None }),

            (Some(cyl), Some(axis)) => {
                let cyl = Cyl::new(cyl, axis)?;

                Ok(Self {
                    sph,
                    cyl: Some(cyl),
                })
            }

            (Some(_cyl), _) => Err(ScaBoundsError::NoPair(CylPair::Axis)),

            (_, Some(_cyl)) => Err(ScaBoundsError::NoPair(CylPair::Power)),
        }
    }
}

mod tests {
    use super::*;
    use crate::axis::Axis;

    #[test]
    fn makes_a_sca() {
        let sca = Sca::new(20.0, Some(5.25), Some(20)).unwrap();

        assert_eq!(
            sca,
            Sca {
                sph: 20.0,
                cyl: Some(Cyl {
                    power: 5.25,
                    axis: Axis(20)
                })
            }
        )
    }

    #[test]
    fn missing_sca_cyl_power_returns_err() {
        let sca: Result<Sca, ScaBoundsError> = Sca::new(20.0, None, Some(20));
        assert_eq!(sca, Err(ScaBoundsError::NoPair(CylPair::Power)))
    }

    #[test]
    fn missing_sca_cyl_axis_returns_err() {
        let sca: Result<Sca, ScaBoundsError> = Sca::new(20.0, Some(5.25), None);
        assert_eq!(sca, Err(ScaBoundsError::NoPair(CylPair::Axis)))
    }

    #[test]
    fn out_of_bounds_sca_cyl_axis_returns_err() {
        let axis = 180i32;
        let sca: Result<Sca, ScaBoundsError> = Sca::new(20.0, Some(5.25), Some(axis));
        assert_eq!(sca, Err(ScaBoundsError::Axis(axis)))
    }
}
