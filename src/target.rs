use crate::{
    bounds_check::{BoundsCheck, Checked, Unchecked},
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
/// unless explicitly overridden by the surgeon. As with other values, we store `value * 100` as an
/// integer, rather than a float.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct Constant {
    pub value: u32,
    pub formula: Formula,
}

/// The residual postop refraction for a case, assuming the provided formula and IOL constant.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Target<Bounds = Unchecked> {
    pub constant: Option<Constant>,
    pub se: i32,
    pub cyl: Option<Cyl>,
    pub bounds: PhantomData<Bounds>,
}

impl<Bounds> Sca for Target<Bounds> {
    fn sph(&self) -> i32 {
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

        if (-600..=200).contains(&se) {
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
    /// Create a new [`Target`] from a generic [`Sca`]. At initialization, the values are not yet
    /// bounds-checked. We allow [`ScaMut`] methods only on the [`Unchecked`] variant (meaning,
    /// before bounds-checking).
    pub fn new<T: Sca>(sca: T, constant: Option<Constant>) -> Self {
        Self {
            constant,
            se: sca.sph(),
            cyl: sca.cyl(),
            bounds: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sca::RawSca;

    // todo: replace with a randomized TargetFormula using Mock(all)
    fn iol_constant() -> Option<Constant> {
        Some(Constant {
            value: 119.36,
            formula: Formula::Kane,
        })
    }

    #[test]
    fn makes_new_target() {
        let sca = RawSca::new(-0.15, Some(0.22), Some(82)).unwrap();
        let _target = Target::new(sca, iol_constant()).check().unwrap();
    }

    #[test]
    fn out_of_bounds_target_se_fails_check() {
        let se = -12.5;
        let sca = RawSca::new(se, Some(0.22), Some(82)).unwrap();
        let target = Target::new(sca, iol_constant()).check();

        assert_eq!(target, Err(TargetBoundsError::Se(se)));
    }

    #[test]
    fn out_of_bounds_target_cyl_power_fails_check() {
        let power = 7.1;
        let sca = RawSca::new(-0.18, Some(power), Some(82)).unwrap();
        let target = Target::new(sca, iol_constant()).check();

        assert_eq!(target, Err(TargetBoundsError::Cyl(power)));
    }
}
