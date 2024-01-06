use crate::{axis::Axis, cyl::Cyl};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TargetBoundsError {
    #[error("target must always have a spherical equivalent, but `None` was supplied")]
    NoSph,

    #[error("target cylinder must have both a power and an axis but the {0:?} was not supplied")]
    NoPair(Cyl),

    #[error(
        "target spherical equivalent must be a value between -6 D and +2 D (supplied value: {0})"
    )]
    Sph(f32),

    #[error("target cylinder power must be a value between 0 D and +6 D (supplied value: {0})")]
    Cyl(f32),

    #[error("target axis must be an integer value between 0° and 179° (supplied value: {0})")]
    Axis(i32),
}

/// A formula for calculating IOL power from biometry.
// Limited to common thick-lens formulas to start.
// Eventually we will add all the formulas commonly in use.
#[derive(Debug, PartialEq)]
pub enum Formula {
    Barrett,
    Kane,
}

#[derive(Debug, PartialEq)]
pub struct TargetSePower(f32);

impl TargetSePower {
    pub fn new(power: f32) -> Option<Self> {
        if (-6.0..=2.0).contains(&power) {
            Some(Self(power))
        } else {
            None
        }
    }
}

/// A newtype to hold powers compatible with a target cylinder value.
#[derive(Debug, PartialEq)]
pub struct TargetCylPower(f32);

impl TargetCylPower {
    /// Creates a new [`TargetCylPower`] of up to 6 diopters, returning `None` if the value is out
    /// of bounds.
    pub fn new(power: f32) -> Option<Self> {
        if (0.0..=6.0).contains(&power) {
            Some(Self(power))
        } else {
            None
        }
    }
}

/// A [`Target`] cylinder value, including power and axis.
#[derive(Debug, PartialEq)]
pub struct TargetCyl {
    power: TargetCylPower,
    axis: Axis,
}

impl TargetCyl {
    fn new(power: f32, axis: i32) -> Result<Self, TargetBoundsError> {
        if let Some(power) = TargetCylPower::new(power) {
            if let Some(axis) = Axis::new(axis) {
                Ok(Self { power, axis })
            } else {
                Err(TargetBoundsError::Axis(axis))
            }
        } else {
            Err(TargetBoundsError::Cyl(power))
        }
    }
}

/// The residual postop refraction predicted by your formula of choice.
// At the start, allow only one formula/target.
#[derive(Debug, PartialEq)]
pub struct Target {
    formula: Option<Formula>,
    se: TargetSePower,
    cyl: Option<TargetCyl>,
}

impl Target {
    pub fn new(
        formula: Option<Formula>,
        se: f32,
        cyl: Option<f32>,
        axis: Option<i32>,
    ) -> Result<Self, TargetBoundsError> {
        if let Some(se) = TargetSePower::new(se) {
            let cyl = match (cyl, axis) {
                (Some(cyl), Some(axis)) => Some(TargetCyl::new(cyl, axis)?),

                (Some(_cyl), _) => return Err(TargetBoundsError::NoPair(Cyl::Axis)),

                (_, Some(_axis)) => return Err(TargetBoundsError::NoPair(Cyl::Power)),

                (_, _) => None,
            };

            Ok(Self { formula, se, cyl })
        } else {
            Err(TargetBoundsError::Sph(se))
        }
    }
}
