// https://stackoverflow.com/questions/54048500/convert-literal-to-associated-type-in-generic-struct-implementation
// https://stackoverflow.com/questions/54504026/how-do-i-provide-an-implementation-of-a-generic-struct-in-rust

use crate::{
    axis::Axis,
    cyl::{Cyl, CylPair},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PowerBoundsError {
    #[error("power must always have a spherical component, but `None` was supplied")]
    NoSph,

    #[error("power cylinder must have both a power and an axis but the {0:?} was not supplied")]
    NoPair(CylPair),

    #[error("cylinder axis must be an integer value between 0° and 179° (supplied value: {0})")]
    Axis(i32),
}

/// A [`Power`] contains the same sphere and cylinder fields as an [`Iol`](crate::iol::Iol),
/// [`Refraction`](crate::refraction::Refraction), or [`Target`](crate::target::Target),
/// but it is a more primitive type, without bounds checking for the sphere or cylinder powers.
/// [`Power`] does have bounds checking for [`Axis`](crate::axis::Axis), because the bounds for
/// this value are always the same.
/// It is primarily useful for conversions into the above types, which succeeds via `try_into()` if the bounds for that type are met.
#[derive(Debug, PartialEq)]
pub struct Power {
    pub sph: f32,
    pub cyl: Option<Cyl>,
}

// the approach should be to implement anything fancy on Power and then all the other types can
// leverage that with try_into
impl Power {
    pub fn new(
        sph: Option<f32>,
        cyl: Option<f32>,
        axis: Option<i32>,
    ) -> Result<Self, PowerBoundsError> {
        match (sph, cyl, axis) {
            (Some(sph), Some(cyl), Some(axis)) => {
                if let Some(axis) = Axis::new(axis) {
                    Ok(Self {
                        sph,
                        cyl: Some(Cyl { power: cyl, axis }),
                    })
                } else {
                    Err(PowerBoundsError::Axis(axis))
                }
            }

            (Some(sph), Some(cyl), _) => Err(PowerBoundsError::NoPair(CylPair::Axis)),

            (Some(sph), _, Some(axis)) => Err(PowerBoundsError::NoPair(CylPair::Power)),

            (Some(sph), _, _) => Ok(Self { sph, cyl: None }),

            (_, _, _) => Err(PowerBoundsError::NoSph),
        }
    }
}
