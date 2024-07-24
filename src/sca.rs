// https://stackoverflow.com/questions/54048500/convert-literal-to-associated-type-in-generic-struct-implementation
// https://stackoverflow.com/questions/54504026/how-do-i-provide-an-implementation-of-a-generic-struct-in-rust

use crate::{
    bounds_check::Unchecked,
    cyl::{Cyl, CylPair},
    refraction::Refraction,
    target::{Constant, Target},
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use thiserror::Error;

/// The error type for an invalid [`Sca`].
#[derive(Debug, Error, PartialEq)]
pub enum ScaBoundsError {
    #[error("cylinder must have both a power and an axis but the {0:?} was not supplied")]
    NoPair(CylPair),

    #[error("cylinder axis must be an integer value between 0° and 179° (supplied value: {0})")]
    Axis(i32),
}

/// A type that wraps a sphere and a cylinder.
pub trait Sca {
    /// Return the spherical value from a [`Sca`].
    fn sph(&self) -> f32;
    /// Return the [`Cyl`] from a [`Sca`].
    fn cyl(&self) -> Option<Cyl>;
}

/// A type that has mutable access to a wrapped sphere and cylinder.
pub trait ScaMut {
    /// Set the value of the wrapped sphere (or spherical equivalent).
    fn set_sph(self, sph: f32) -> Self;
    /// Set the value of the wrapped [`Cyl`].
    fn set_cyl(self, cyl: Option<Cyl>) -> Self;
}

/// A primitive type wrapping a sphere and a cylinder. Can be passed to [`Sca`] constructors that
/// apply bounds checking and return a more specific type.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct RawSca {
    pub sph: f32,
    pub cyl: Option<Cyl>,
}

impl Sca for RawSca {
    fn sph(&self) -> f32 {
        self.sph
    }

    fn cyl(&self) -> Option<Cyl> {
        self.cyl
    }
}

impl ScaMut for RawSca {
    fn set_sph(mut self, sph: f32) -> Self {
        self.sph = sph;
        self
    }

    fn set_cyl(mut self, cyl: Option<Cyl>) -> Self {
        self.cyl = cyl;
        self
    }
}

impl RawSca {
    /// Construct a new [`RawSca`], with bounds checking on the [`Axis`](crate::axis::Axis).
    pub fn new(sph: f32, power: Option<f32>, axis: Option<i32>) -> Result<Self, ScaBoundsError> {
        let cyl = match (power, axis) {
            (Some(power), Some(axis)) => Some(Cyl::new(power, axis)?),
            (None, None) => None,
            (_, None) => return Err(ScaBoundsError::NoPair(CylPair::Axis)),
            (None, _) => return Err(ScaBoundsError::NoPair(CylPair::Power)),
        };

        Ok(Self { sph, cyl })
    }

    /// Convert a [`RawSca`] into a [`Refraction`].
    pub fn into_refraction(&self) -> Refraction<Unchecked> {
        Refraction {
            sph: self.sph(),
            cyl: self.cyl(),
            bounds: PhantomData,
        }
    }

    /// Convert a [`RawSca`] into a [`Target`].
    pub fn into_target(&self, constant: Option<Constant>) -> Target<Unchecked> {
        Target {
            constant,
            se: self.sph(),
            cyl: self.cyl(),
            bounds: PhantomData,
        }
    }
}

mod tests {
    use super::*;

    // todo: unit test RawSca creation, and possibly other methods.
}
