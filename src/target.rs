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

mod tests {
    use super::*;
    use crate::axis::Axis;

    #[test]
    fn makes_new_target() {
        let sca = Sca::new(-0.15, Some(0.22), Some(82)).unwrap();
        let target = Target::new(Some(Formula::Kane), sca).unwrap();

        assert_eq!(
            target,
            Target {
                formula: Some(Formula::Kane),
                sca: Sca {
                    sph: -0.15,
                    cyl: Some(Cyl {
                        power: 0.22,
                        axis: Axis(82)
                    })
                }
            }
        )
    }

    #[test]
    fn out_of_bounds_target_se_returns_err() {
        let se = -12.5f32;
        let sca = Sca::new(se, Some(0.22), Some(82)).unwrap();
        let target = Target::new(Some(Formula::Kane), sca);

        assert_eq!(target, Err(TargetBoundsError::Se(se)))
    }

    #[test]
    fn out_of_bounds_target_cyl_power_returns_err() {
        let cyl = 7.1f32;
        let sca = Sca::new(-0.24, Some(cyl), Some(82)).unwrap();
        let target = Target::new(Some(Formula::Kane), sca);

        assert_eq!(target, Err(TargetBoundsError::Cyl(cyl)))
    }
}
