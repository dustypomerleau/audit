use crate::{
    check::{BoundsCheck, Checked, Unchecked},
    cyl::Cyl,
    sca::{Sca, ScaMut},
};
#[cfg(feature = "ssr")] use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use thiserror::Error;

/// The error type for an invalid [`Target`]
#[derive(Debug, Error, PartialEq)]
pub enum TargetBoundsError {
    #[error("the formula used for IOL calculation is not recognized (given value: {0})")]
    Formula(String),

    #[error(
        "target spherical equivalent must be a value between -6 D and +2 D (supplied value: {0})"
    )]
    Se(f32),

    #[error("target cylinder power must be a value between 0 D and +6 D (supplied value: {0})")]
    Cyl(f32),

    #[error("target constant requires both an IOL and a value, but the {0:?} was not supplied")]
    NoPair(ConstantPair),
}

/// Required values for the [`Iol`](crate::iol::Iol) constant to be associated with a specific
/// [`Target`]
#[derive(Debug, PartialEq)]
pub enum ConstantPair {
    Value,
    Formula,
}

/// A formula for calculating IOL power from biometry.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub enum Formula {
    Barrett,
    BarrettTrueK,
    Haigis,
    HofferQ,
    Holladay1,
    Holladay2,
    Kane,
    Olsen,
    SrkT,
}

/// The combination of formula and IOL constant used to calculate the [`Target`] for a
/// [`Case`](crate::case::Case). The default constant for the case's IOL/formula pair is used,
/// unless explicitly overridden by the surgeon.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct Constant {
    pub value: f32,
    pub formula: Formula,
}

/// The residual postop refraction for a case, assuming the provided formula and IOL constant.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Target<Bounds = Unchecked> {
    pub constant: Option<Constant>,
    pub se: f32,
    pub cyl: Option<Cyl>,
    pub bounds: PhantomData<Bounds>,
}

impl<Bounds> Sca for Target<Bounds> {
    fn sph(&self) -> f32 {
        self.se
    }

    fn cyl(&self) -> Option<Cyl> {
        self.cyl
    }
}

impl BoundsCheck for Target<Unchecked> {
    type CheckedOutput = Target<Checked>;
    type Error = TargetBoundsError;

    fn check(self) -> Result<Self::CheckedOutput, Self::Error> {
        let Self {
            constant, se, cyl, ..
        } = self;

        if (-6.0..=2.0).contains(&se) {
            let cyl = if let Some(Cyl { power, .. }) = cyl {
                if (0.0..=6.0).contains(&power) {
                    cyl
                } else {
                    return Err(TargetBoundsError::Cyl(power));
                }
            } else {
                None
            };

            Ok(Target::<Checked> {
                constant,
                se,
                cyl,
                bounds: PhantomData,
            })
        } else {
            Err(TargetBoundsError::Se(se))
        }
    }
}

impl ScaMut for Target<Unchecked> {
    fn set_sph(mut self, sph: f32) -> Self {
        self.se = sph;
        self
    }

    fn set_cyl(mut self, cyl: Option<Cyl>) -> Self {
        self.cyl = cyl;
        self
    }
}

impl Target<Unchecked> {
    /// Create a new [`Target`] without bounds checking.
    pub fn new(constant: Option<Constant>, se: f32, cyl: Option<Cyl>) -> Self {
        Self {
            constant,
            se,
            cyl,
            bounds: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // todo: replace with a randomized TargetFormula using Mock(all)
    fn iol_constant() -> Option<Constant> {
        Some(Constant {
            value: 119.36,
            formula: Formula::Thick(Thick::Kane),
        })
    }

    #[test]
    fn makes_new_formula() {
        let formula = Formula::new_from_str("Barrett True K").unwrap();
        assert_eq!(formula, Formula::Thick(Thick::BarrettTrueK))
    }

    #[test]
    fn unknown_formula_returns_err() {
        let formula = Formula::new_from_str("Awesome Formula");
        assert_eq!(
            formula,
            Err(TargetBoundsError::Formula("Awesome Formula".to_string()))
        )
    }

    #[test]
    fn makes_new_target() {
        let target = Target::new(
            Some(iol_constant()),
            -0.15,
            Some(Cyl {
                power: 0.22,
                axis: Axis(82),
            }),
        )
        .check();

        println!(std::any::type_name(target));
    }

    #[test]
    fn out_of_bounds_target_se_returns_err() {
        let constant = iol_constant();
        let se = -12.5;
        let cyl = Cyl::new(0.22, 82);
        let target = Target::new(constant, se, cyl);

        assert_eq!(target, Err(TargetBoundsError::Se(se)))
    }

    #[test]
    fn out_of_bounds_target_cyl_power_returns_err() {
        let constant = iol_constant();
        let se = -12.5;
        let cyl = Cyl::new(7.1, 82);
        let target = Target::new(constant, se, cyl);

        assert_eq!(target, Err(TargetBoundsError::Cyl(cyl)))
    }
}
