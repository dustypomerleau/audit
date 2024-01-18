use crate::{
    cyl::Cyl,
    sca::{Sca, ScaBoundsError},
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TargetBoundsError {
    #[error("target cannot be created because the underlying Sca violated its invariants: {0:?}")]
    Sca(ScaBoundsError),

    #[error(
        "target spherical equivalent must be a value between -6 D and +2 D (supplied value: {0})"
    )]
    Se(f32),

    #[error("target cylinder power must be a value between 0 D and +6 D (supplied value: {0})")]
    Cyl(f32),
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
    sca: Sca,
}

impl Target {
    pub fn new(formula: Option<Formula>, sca: Sca) -> Result<Self, TargetBoundsError> {
        let Sca { sph, cyl } = sca;

        if (-6.0..=2.0).contains(&sph) {
            match cyl {
                Some(Cyl { power, axis: _ }) => {
                    if (0.0..=6.0).contains(&power) {
                        Ok(Self { formula, sca })
                    } else {
                        Err(TargetBoundsError::Cyl(power))
                    }
                }

                None => Ok(Self { formula, sca }),
            }
        } else {
            Err(TargetBoundsError::Se(sph))
        }
    }
}
