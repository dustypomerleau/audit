use crate::{
    cyl::Cyl,
    sca::{Sca, ScaBoundsError},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TargetBoundsError {
    #[error("the formula used for IOL calculation is not recognized (given value: {0})")]
    Formula(String),

    #[error("target cannot be created because the underlying Sca violated its invariants: {0:?}")]
    Sca(ScaBoundsError),

    #[error(
        "target spherical equivalent must be a value between -6 D and +2 D (supplied value: {0})"
    )]
    Se(f32),

    #[error("target cylinder power must be a value between 0 D and +6 D (supplied value: {0})")]
    Cyl(f32),
}

// todo: add all common variants
/// A representation of thick-lens IOL formulas.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Thick {
    Barrett,
    BarrettTrueK,
    Holladay2,
    Kane,
    Olsen,
}

// todo: add all common variants
/// A representation of thin-lens IOL formulas.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Thin {
    Haigis,
    HofferQ,
    Holladay1,
    Srkt,
}

// Limited to common thick-lens formulas to start.
// Eventually we will add all the formulas commonly in use.
/// A formula for calculating IOL power from biometry.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Formula {
    Thick(Thick),
    Thin(Thin),
}

impl Formula {
    /// Create a new [`Formula`] from an input string. Primarily useful for converting formula
    /// names pulled from the database.
    pub fn new_from_str(input: &str) -> Result<Formula, TargetBoundsError> {
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
    pub fn formula_to_string(f: Formula) -> String {
        match f {
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
/// [`Case`](crate::case::Case).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TargetFormula {
    formula: Formula,
    constant: f32,
}

/// The residual postop refraction for a case, assuming the provided formula and IOL constant.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Target {
    pub target_formula: Option<TargetFormula>,
    pub sca: Sca,
}

impl Target {
    /// Create a new [`Target`] with bounds checking on the [`Sca`].
    pub fn new(target_formula: Option<TargetFormula>, sca: Sca) -> Result<Self, TargetBoundsError> {
        let Sca { sph, cyl } = sca;

        if (-6.0..=2.0).contains(&sph) {
            let sca = match cyl {
                Some(Cyl { power, .. }) => {
                    if (0.0..=6.0).contains(&power) {
                        sca
                    } else {
                        return Err(TargetBoundsError::Cyl(power));
                    }
                }

                None => sca,
            };

            Ok(Self {
                target_formula,
                sca,
            })
        } else {
            Err(TargetBoundsError::Se(sph))
        }
    }
}

mod tests {
    use super::*;

    // todo: replace with a randomized TargetFormula using Mock
    fn target_formula() -> Option<TargetFormula> {
        Some(TargetFormula {
            formula: Formula::Thick(Thick::Kane),
            constant: 119.36,
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
        let target_formula = target_formula();
        let sca = Sca::new(-0.15, Some(0.22), Some(82)).unwrap();
        let target = Target::new(target_formula, sca).unwrap();

        assert_eq!(
            target,
            Target {
                target_formula,
                sca
            }
        )
    }

    #[test]
    fn out_of_bounds_target_se_returns_err() {
        let se = -12.5;
        let target_formula = target_formula();
        let sca = Sca::new(se, Some(0.22), Some(82)).unwrap();
        let target = Target::new(target_formula, sca);

        assert_eq!(target, Err(TargetBoundsError::Se(se)))
    }

    #[test]
    fn out_of_bounds_target_cyl_power_returns_err() {
        let cyl = 7.1;
        let target_formula = target_formula();
        let sca = Sca::new(-0.24, Some(cyl), Some(82)).unwrap();
        let target = Target::new(target_formula, sca);

        assert_eq!(target, Err(TargetBoundsError::Cyl(cyl)))
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
}
