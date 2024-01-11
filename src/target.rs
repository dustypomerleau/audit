use crate::{
    axis::Axis,
    cyl::{Cyl, CylPair},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TargetBoundsError {
    #[error("target must always have a spherical equivalent, but `None` was supplied")]
    NoSph,

    #[error("target cylinder must have both a power and an axis but the {0:?} was not supplied")]
    NoPair(CylPair),

    #[error(
        "target spherical equivalent must be a value between -6 D and +2 D (supplied value: {0})"
    )]
    Se(f32),

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

/// The residual postop refraction predicted by your formula of choice.
// At the start, allow only one formula/target.
#[derive(Debug, PartialEq)]
pub struct Target {
    formula: Option<Formula>,
    se: f32,
    cyl: Option<Cyl>,
}

impl Target {
    pub fn new(
        formula: Option<Formula>,
        se: f32,
        cyl: Option<f32>,
        axis: Option<i32>,
    ) -> Result<Self, TargetBoundsError> {
        if (-6.0..=2.0).contains(&se) {
            match (cyl, axis) {
                (Some(cyl), Some(axis)) => {
                    if (0.0..=6.0).contains(&cyl) {
                        if let Some(axis) = Axis::new(axis) {
                            Ok(Self {
                                formula,
                                se,
                                cyl: Some(Cyl { power: cyl, axis }),
                            })
                        } else {
                            Err(TargetBoundsError::Axis(axis))
                        }
                    } else {
                        Err(TargetBoundsError::Cyl(cyl))
                    }
                }

                (Some(_cyl), _) => Err(TargetBoundsError::NoPair(CylPair::Axis)),

                (_, Some(_axis)) => Err(TargetBoundsError::NoPair(CylPair::Power)),

                (_, _) => Ok(Self {
                    formula,
                    se,
                    cyl: None,
                }),
            }
        } else {
            Err(TargetBoundsError::Se(se))
        }
    }
}
