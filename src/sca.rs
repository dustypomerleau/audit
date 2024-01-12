// https://stackoverflow.com/questions/54048500/convert-literal-to-associated-type-in-generic-struct-implementation
// https://stackoverflow.com/questions/54504026/how-do-i-provide-an-implementation-of-a-generic-struct-in-rust

use crate::{
    axis::Axis,
    cyl::{Cyl, CylPair},
    iol::Iol,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScaBoundsError {
    #[error("sphere, cylinder, axis trios must always have a spherical component, but `None` was supplied")]
    NoSph,

    #[error("cylinder must have both a power and an axis but the {0:?} was not supplied")]
    NoPair(CylPair),

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

// the approach should be to implement anything fancy on Sca and then all the other types can
// leverage that with try_into
// A simple approach to getting Option<Cyl> would be:
// let cyl = Cyl::new(cyl, axis).ok();
// but this deprives you of detailed errors if only one of the cylinder values is missing.
impl Sca {
    pub fn new(
        sph: Option<f32>,
        cyl: Option<f32>,
        axis: Option<i32>,
    ) -> Result<Self, ScaBoundsError> {
        match (sph, cyl, axis) {
            (Some(sph), None, None) => Ok(Self { sph, cyl: None }),

            (Some(sph), _, _) => Ok(Self {
                sph,
                cyl: Some(Cyl::new(cyl, axis)?),
            }),

            (_, _, _) => Err(ScaBoundsError::NoSph),
        }
    }
}
