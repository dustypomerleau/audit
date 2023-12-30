use crate::{axis::Axis, sca::BadSca};

/// A formula for calculating IOL power from biometry.
// Limited to common thick-lens formulas to start.
// Eventually we will add all the formulas commonly in use.
#[derive(Debug, PartialEq)]
pub enum Formula {
    Barrett,
    Kane,
}

/// A newtype to hold powers compatible with a target cylinder value.
#[derive(Debug, PartialEq)]
pub struct TargetCylPower(f32);

impl TargetCylPower {
    /// Creates a new [`TargetCylPower`] of up to 6 diopters, returning `None` if the value is out
    /// of bounds.
    pub fn new(power: f32) -> Option<Self> {
        if (0.0..=6.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

/// A [`Target`] cylinder value, including power and axis.
#[derive(Debug, PartialEq)]
pub enum TargetCyl {
    OutOfBounds(BadSca),
    Value { power: TargetCylPower, axis: Axis },
}

impl TargetCyl {
    /// Returns a new [`TargetCyl::Value`] if both the [`TargetCylPower`] and the [`Axis`] are within
    /// bounds. Returns [`TargetCyl::OutOfBounds`] with the offending field if either is out of
    /// bounds.
    fn new(power: f32, axis: f32) -> Self {
        if let Some(power) = TargetCylPower::new(power) {
            if let Some(axis) = Axis::new(axis) {
                Self::Value { power, axis }
            } else {
                Self::OutOfBounds(BadSca::Axis)
            }
        } else {
            Self::OutOfBounds(BadSca::Cyl)
        }
    }
}

/// The residual postop refraction predicted by your formula of choice.
// At the start, allow only one formula/target.
#[derive(Debug, PartialEq)]
pub enum Target {
    OutOfBounds(BadSca),
    Value {
        formula: Option<Formula>,
        se: f32,
        cyl: Option<TargetCyl>,
    },
}

impl Target {
    /// Returns a new [`Target`], giving the [`Target::Value`] variant if the spherical equivalent is within bounds,
    /// and [`Target::OutOfBounds`] if the `se` is out of bounds. The `cyl` field only contains
    /// `Some(TargetCyl)` if both `cyl` and `axis` are within bounds.
    pub fn new(formula: Option<Formula>, se: f32, cyl: Option<f32>, axis: Option<i32>) -> Self {
        if (-6.0..=2.0).contains(&se) {
            match (cyl, axis) {
                (Some(cyl), Some(axis)) => {
                    let cyl = TargetCyl::new(cyl, axis);
                    let cyl = match cyl {
                        TargetCyl::Value => Some(cyl),
                        // todo: log bad cyl, throw an error explaining that cyl is out of bounds
                        TargetCyl::OutOfBounds(BadSca::Cyl) => None,
                        // todo: log bad axis, throw an error explaining that axis is out of bounds
                        TargetCyl::OutOfBounds(BadSca::Axis) => None,
                    };

                    Self::Value { formula, se, cyl }
                }

                _ => Self::Value {
                    formula,
                    se,
                    cyl: None,
                },
            }
        } else {
            Self::OutOfBounds(BadSca::Sph)
        }
    }
}
