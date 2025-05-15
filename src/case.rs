use crate::{
    biometry::{Acd, Al, Biometry, Cct, K, Kpower, Ks, Lt, Wtw},
    bounded::Bounded,
    cyl::{Axis, RawCyl},
    db::{DbError, db},
    iol::{Iol, IolSe, OpIol},
    refraction::OpRefraction,
    sca::{RawSca, into_refraction},
    sia::{Sia, SiaPower},
    surgeon::Site,
    target::{Formula, Target, TargetCyl, TargetCylPower, TargetSe},
    va::{AfterVa, BeforeVa, OpVa, Va, VaDen, VaNum},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, range::RangeBounds};
use thiserror::Error;

/// The required fields for each [`Case`]. Used by [`CaseError::MissingField`].
#[derive(Debug, PartialEq)]
pub enum Required {
    Date,
    Email,
    Iol,
    Refraction,
    Side,
    Target,
    Urn,
    Va,
}

/// A wrapper for any type of bounds error on numeric types.
#[derive(Debug, Error)]
#[error("the value is out of bounds {0:?}")]
pub struct BoundsError(pub String);

/// The error type for a [`Case`] with missing fields or out of bounds values.
#[derive(Debug, Error)]
pub enum CaseError {
    #[error("out of bounds value on a `Case`: {0:?}")]
    Bounds(BoundsError),

    #[error("unable to connect to the database")]
    Db(DbError),

    #[error("{0:?} is a required field on `Case`, but wasn't supplied")]
    MissingField(Required),
}

impl From<BoundsError> for CaseError {
    fn from(err: BoundsError) -> Self {
        Self::Bounds(BoundsError(format!("{err:?}")))
    }
}

/// The side of the patient's surgery.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Side {
    Right,
    Left,
}

// Implementing Display is necessary for enums to impl Into<gel_protocol::Value>
impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
        }
    }
}

/// An adverse intraoperative event. Classification is at the surgeon's discretion, and only one
/// option can be selected. For example, a wrap around split in the rhexis opens the PC, but in the
/// surgeon's view it may be essentially a rhexis complication. For our purposes, we aren't
/// particularly concerned with how the adverse event was handled (for example, whether a
/// vitrectomy was required). We are interested only in the relative outcomes of cases with adverse
/// events versus those without.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Adverse {
    Rhexis,
    Pc,
    Zonule,
    Other,
}

impl Display for Adverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rhexis => write!(f, "Rhexis"),
            Self::Pc => write!(f, "Pc"),
            Self::Zonule => write!(f, "Zonule"),
            Self::Other => write!(f, "Other"),
        }
    }
}

bounded!((Main, u32, 100..=600));

/// A single surgical case.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Case {
    pub side: Side,
    pub biometry: Biometry,
    pub target: Target,
    pub main: Main,
    pub sia: Sia,
    pub iol: OpIol,
    pub adverse: Option<Adverse>,
    pub va: OpVa,
    pub refraction: OpRefraction,
}

// todo: match required fields on the form to non-Option here
// todo: fix Ks in the form
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormCase {
    pub urn: String,
    pub date: NaiveDate,      // prefill today
    pub site: Option<String>, // prefill default
    pub side: String,
    pub al: f32,
    pub k1_power: f32,
    pub k1_axis: u32,
    pub k2_power: f32,
    pub k2_axis: u32,
    pub acd: f32,
    pub lt: f32,
    pub cct: Option<u32>,
    pub wtw: Option<u32>,
    pub formula: String, // prefill default
    pub custom_constant: Option<String>,
    pub target_se: f32,
    pub target_cyl_power: Option<f32>,
    pub target_cyl_axis: Option<u32>,
    pub main: f32,         // prefill default
    pub sia_power: f32,    // prefill default
    pub sia_axis: u32,     // prefill default for side (needs signal)
    pub iol_model: String, // prefill default
    pub iol_se: f32,
    pub iol_axis: Option<u32>,   // cyl power is supplied by the Iol
    pub adverse: String,         // prefill "None"
    pub va_best_before_num: u32, // prefill 6
    pub va_best_before_den: f32,
    pub va_raw_before_num: Option<u32>,
    pub va_raw_before_den: Option<f32>,
    pub va_best_after_num: Option<u32>,
    pub va_best_after_den: Option<f32>,
    pub va_raw_after_num: u32, // prefill 6
    pub va_raw_after_den: f32,
    pub ref_before_sph: f32,
    pub ref_before_cyl_power: Option<f32>,
    pub ref_before_cyl_axis: Option<u32>,
    pub ref_after_sph: f32,
    pub ref_after_cyl_power: Option<f32>,
    pub ref_after_cyl_axis: Option<u32>,
}

pub struct SurgeonCase {
    /// A unique value that allows (only) the surgeon to deanonymize the case.
    pub urn: String,
    pub date: NaiveDate,
    pub site: Option<Site>,
    pub case: Case,
}

#[cfg(feature = "ssr")]
impl FormCase {
    pub async fn into_surgeon_case(self) -> Result<SurgeonCase, CaseError> {
        let FormCase {
            urn,
            date,
            site,
            side,
            al,
            k1_power,
            k1_axis,
            k2_power,
            k2_axis,
            acd,
            lt,
            cct,
            wtw,
            formula,
            custom_constant,
            target_se,
            target_cyl_power,
            target_cyl_axis,
            main,
            sia_power,
            sia_axis,
            iol_model,
            iol_se,
            iol_axis,
            adverse,
            va_best_before_num,
            va_best_before_den,
            va_raw_before_num,
            va_raw_before_den,
            va_best_after_num,
            va_best_after_den,
            va_raw_after_num,
            va_raw_after_den,
            ref_before_sph,
            ref_before_cyl_power,
            ref_before_cyl_axis,
            ref_after_sph,
            ref_after_cyl_power,
            ref_after_cyl_axis,
        } = self;

        let site = site.and_then(|name| Some(Site { name }));

        let side = match side.as_str() {
            "right" => Side::Right,
            "left" => Side::Left,
            _ => unreachable!(),
        };

        // intentionally truncate
        let biometry = Biometry {
            al: Al::new((al * 100.0) as u32)?,
            ks: Ks::new(
                K::new(Kpower::new((k1_power * 100.0) as u32)?, Axis::new(k1_axis)?),
                K::new(Kpower::new((k2_power * 100.0) as u32)?, Axis::new(k2_axis)?),
            ),
            acd: Acd::new((acd * 100.0) as u32)?,
            lt: Lt::new((lt * 100.0) as u32)?,
            cct: cct.and_then(|cct| Cct::new(cct).ok()),
            wtw: wtw.and_then(|wtw| Wtw::new(wtw).ok()),
        };

        fn to_formula(formula: &str) -> Formula {
            match formula {
                "ascrskrs" => Formula::AscrsKrs,
                "barrett" | "barretttoric" => Formula::Barrett,
                "barretttruek" => Formula::BarrettTrueK,
                "evo" => Formula::Evo,
                "haigis" => Formula::Haigis,
                "haigisl" => Formula::HaigisL,
                "hillrbf" => Formula::HillRbf,
                "hofferq" => Formula::HofferQ,
                "holladay1" => Formula::Holladay1,
                "holladay2" => Formula::Holladay2,
                "kane" => Formula::Kane,
                "okulix" => Formula::Okulix,
                "olsen" => Formula::Olsen,
                "srkt" => Formula::SrkT,
                _ => Formula::Other,
            }
        }

        let formula = to_formula(formula.as_str());

        let target_cyl = match (target_cyl_power, target_cyl_axis) {
            (Some(power), Some(axis)) => Some(TargetCyl::new(
                TargetCylPower::new((power * 100.0) as u32)?,
                Axis::new(axis)?,
            )),

            _ => None,
        };

        let target = Target {
            formula: Some(formula),
            custom_constant: if custom_constant == Some("true".to_string()) {
                true
            } else {
                false
            },
            se: TargetSe::new((target_se * 100.0) as i32)?,
            cyl: target_cyl,
        };

        let main = Main::new((main * 100.0) as u32)?;

        let sia = Sia::new(
            SiaPower::new((sia_power * 100.0) as u32)?,
            Axis::new(sia_axis)?,
        );

        // note: For now we are assuming the IOL model is in the DB. To start, offer an option in
        // the datalist that the IOL is not listed, and have a DB option for that.
        let iol = if let Ok(Some(json)) = db()
            .await
            .map_err(|err| CaseError::Db(err))?
            .query_single_json(format!(r#"select Iol filter .model = "{iol_model}";"#), &())
            .await
        {
            let iol = serde_json::from_str::<Iol>(json.as_ref())
                .map_err(|err| BoundsError(format!("{err:?}")))?;

            OpIol {
                iol,
                se: IolSe::new((iol_se * 100.0) as i32)?,
                axis: iol_axis.and_then(|axis| Axis::new(axis).ok()),
            }
        } else {
            return Err(CaseError::MissingField(Required::Iol));
        };

        fn to_adverse(s: &str) -> Option<Adverse> {
            match s {
                "rhexis" => Some(Adverse::Rhexis),
                "pc" => Some(Adverse::Pc),
                "zonule" => Some(Adverse::Zonule),
                "other" => Some(Adverse::Other),
                _ => None,
            }
        }

        let adverse = to_adverse(adverse.as_str());

        let va = OpVa {
            before: BeforeVa {
                best: Va::new(
                    VaNum::new(va_best_before_num * 100)?,
                    VaDen::new((va_best_before_den * 100.0) as u32)?,
                ),

                raw: match (va_raw_before_num, va_raw_before_den) {
                    (Some(num), Some(den)) => Some(Va::new(
                        VaNum::new(num * 100)?,
                        VaDen::new((den * 100.0) as u32)?,
                    )),

                    _ => None,
                },
            },

            after: AfterVa {
                best: match (va_best_after_num, va_best_after_den) {
                    (Some(num), Some(den)) => Some(Va::new(
                        VaNum::new(num * 100)?,
                        VaDen::new((den * 100.0) as u32)?,
                    )),

                    _ => None,
                },

                raw: Va::new(
                    VaNum::new(va_raw_after_num * 100)?,
                    VaDen::new((va_raw_after_den * 100.0) as u32)?,
                ),
            },
        };

        let ref_before_raw_cyl = match (ref_before_cyl_power, ref_before_cyl_axis) {
            (Some(power), Some(axis)) => {
                Some(RawCyl::new((power * 100.0) as i32, Axis::new(axis)?))
            }

            _ => None,
        };

        let ref_after_raw_cyl = match (ref_after_cyl_power, ref_after_cyl_axis) {
            (Some(power), Some(axis)) => {
                Some(RawCyl::new((power * 100.0) as i32, Axis::new(axis)?))
            }

            _ => None,
        };

        let refraction = OpRefraction {
            before: {
                let sca = RawSca::new((ref_before_sph * 100.0) as i32, ref_before_raw_cyl);
                into_refraction(sca)?
            },
            after: {
                let sca = RawSca::new((ref_after_sph * 100.0) as i32, ref_after_raw_cyl);
                into_refraction(sca)?
            },
        };

        let case = Case {
            side,
            biometry,
            target,
            main,
            sia,
            iol,
            adverse,
            va,
            refraction,
        };

        Ok(SurgeonCase {
            urn,
            date,
            site,
            case,
        })
    }
}
