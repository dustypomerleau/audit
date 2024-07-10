use crate::{
    check::{BoundsCheck, Checked, Unchecked},
    cyl::Cyl,
    sca::{Sca, ScaMut},
};
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

// todo: add all common variants
/// A thick-lens IOL formula.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Thick {
    Barrett,
    BarrettTrueK,
    Holladay2,
    Kane,
    Olsen,
}

// todo: add all common variants
/// A thin-lens IOL formula.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Thin {
    Haigis,
    HofferQ,
    Holladay1,
    Srkt,
}

/// A formula for calculating IOL power from biometry.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Formula {
    Thick(Thick),
    Thin(Thin),
}

impl Formula {
    /// Create a new [`Formula`] from an input string. Primarily useful for converting formula
    /// names pulled from the database.
    pub fn new_from_str(input: &str) -> Result<Self, TargetBoundsError> {
        let mut f = input.to_string().to_lowercase();
        // trying to avoid pulling in regex here. '/' avoids SRK/T, which surely someone will try
        f.retain(|c| c != ' ' && c != '/');

        let formula = match f.as_str() {
            "barrett" => Formula::Thick(Thick::Barrett),
            "barretttruek" => Formula::Thick(Thick::BarrettTrueK),
            "haigis" => Formula::Thin(Thin::Haigis),
            "hofferq" => Formula::Thin(Thin::HofferQ),
            "holladay1" => Formula::Thin(Thin::Holladay1),
            "holladay2" => Formula::Thick(Thick::Holladay2),
            "kane" => Formula::Thick(Thick::Kane),
            "olsen" => Formula::Thick(Thick::Olsen),
            "srkt" => Formula::Thin(Thin::Srkt),
            _ => return Err(TargetBoundsError::Formula(input.to_string())),
        };

        Ok(formula)
    }

    /// Convert to a [`String`] representation of the [`Formula`] (typically for database
    /// insertion).
    pub fn to_string(self) -> String {
        match self {
            Formula::Thick(thick) => match thick {
                Thick::Barrett => "Barrett".to_string(),
                Thick::BarrettTrueK => "Barrett True K".to_string(),
                Thick::Holladay2 => "Holladay 2".to_string(),
                Thick::Kane => "Kane".to_string(),
                Thick::Olsen => "Olsen".to_string(),
            },

            Formula::Thin(thin) => match thin {
                Thin::Haigis => "Haigis".to_string(),
                Thin::HofferQ => "Hoffer Q".to_string(),
                Thin::Holladay1 => "Holladay 1".to_string(),
                Thin::Srkt => "SRK/T".to_string(),
            },
        }
    }
}

/// The combination of formula and IOL constant used to calculate the [`Target`] for a
/// [`Case`](crate::case::Case). The default constant for the case's IOL/formula pair is used,
/// unless explicitly overridden by the surgeon.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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
    type Error = TargetBoundsError;
    type Output = Target<Checked>;

    fn check(self) -> Result<Self::Output, Self::Error> {
        let Target {
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
