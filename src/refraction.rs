use crate::{axis::Axis, cyl::Cyl};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RefBoundsError {
    #[error("refraction must always have a spherical component, but `None` was supplied")]
    NoSph,

    #[error(
        "refraction cylinder must have both a power and an axis but the {0:?} was not supplied"
    )]
    NoPair(Cyl),

    #[error("refraction sphere must be a value between -20 and +20 (supplied value: {0})")]
    Sph(f32),

    #[error("refraction cylinder must be a value between -10 and +10 (supplied value: {0})")]
    Cyl(f32),

    #[error("refraction axis must be an integer between 0 and 179 (supplied value: {0})")]
    Axis(i32),
}

/// The spherical component of a subjective refraction.
#[derive(Debug, PartialEq)]
pub struct RefSphPower(f32);

impl RefSphPower {
    pub fn new(power: f32) -> Option<Self> {
        if (-20.0..=20.0).contains(&power) && power % 0.25 == 0.0 {
            Some(Self(power))
        } else {
            None
        }
    }
}

/// The cylindrical power component of a subjective refraction.
#[derive(Debug, PartialEq)]
pub struct RefCylPower(f32);

impl RefCylPower {
    pub fn new(power: f32) -> Option<Self> {
        if (-10.0..=10.0).contains(&power) && power % 0.25 == 0.0 {
            Some(Self(power))
        } else {
            None
        }
    }
}

/// The cylinder component of a subjective refraction, consisting of a cylindrical power in
/// diopters, and an axis in degrees.
#[derive(Debug, PartialEq)]
pub struct RefCyl {
    power: RefCylPower,
    axis: Axis,
}

// todo: this function should probably be generic, but we will need the associated type of the
// error
impl RefCyl {
    fn new(power: f32, axis: i32) -> Result<Self, RefBoundsError> {
        if let Some(power) = RefCylPower::new(power) {
            if let Some(axis) = Axis::new(axis) {
                Ok(Self { power, axis })
            } else {
                Err(RefBoundsError::Axis(axis))
            }
        } else {
            Err(RefBoundsError::Cyl(power))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Refraction {
    sph: RefSphPower,
    cyl: Option<RefCyl>,
}

impl Refraction {
    pub fn new(sph: f32, cyl: Option<f32>, axis: Option<i32>) -> Result<Self, RefBoundsError> {
        if let Some(sph) = RefSphPower::new(sph) {
            let cyl = match (cyl, axis) {
                (Some(cyl), Some(axis)) => Some(RefCyl::new(cyl, axis)?),

                (Some(_cyl), _) => return Err(RefBoundsError::NoPair(Cyl::Axis)),

                (_, Some(_axis)) => return Err(RefBoundsError::NoPair(Cyl::Power)),

                (_, _) => None,
            };

            Ok(Self { sph, cyl })
        } else {
            Err(RefBoundsError::Sph(sph))
        }
    }
}

// for now, limit this to distance refraction
// todo: consider how best to enforce this - it might complicate your life, but you could consider
// something like Refraction::Sph(Refr::Cyl { sph: RefSphPower, cyl: RefCyl })
#[derive(Debug, PartialEq)]
pub struct OpRefraction {
    before: Refraction,
    after: Refraction,
}
