// https://stackoverflow.com/questions/54048500/convert-literal-to-associated-type-in-generic-struct-implementation
// https://stackoverflow.com/questions/54504026/how-do-i-provide-an-implementation-of-a-generic-struct-in-rust

use crate::cyl::{Cyl, CylPair};
use thiserror::Error;

/// The error type for an invalid [`Sca`].
#[derive(Debug, Error, PartialEq)]
pub enum ScaBoundsError {
    #[error("cylinder must have both a power and an axis but the {0:?} was not supplied")]
    NoPair(CylPair),

    #[error("cylinder axis must be an integer value between 0° and 179° (supplied value: {0})")]
    Axis(i32),
}

pub trait Sca {
    fn sph(&self) -> f32;
    fn cyl(&self) -> Option<Cyl>;
}

pub trait ScaMut {
    fn set_sph(self, sph: f32) -> Self;
    fn set_cyl(self, cyl: Option<Cyl>) -> Self;
}

pub struct RawSca {
    pub sph: f32,
    pub cyl: Option<Cyl>,
}

impl RawSca {
    pub fn new(sph: f32, power: Option<f32>, axis: Option<i32>) -> Result<Self, ScaBoundsError> {
        let cyl = match (power, axis) {
            (Some(power), Some(axis)) => Some(Cyl::new(power, axis)?),
            (None, None) => None,
            (Some(_cyl), _) => return Err(ScaBoundsError::NoPair(CylPair::Axis)),
            (_, Some(_axis)) => return Err(ScaBoundsError::NoPair(CylPair::Power)),
        };

        Ok(Self { sph, cyl })
    }
}
