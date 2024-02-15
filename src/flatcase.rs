use crate::{
    case::{Adverse, Case, Side},
    csv::WriteString,
    cyl::Cyl,
    iol::{Focus, Iol, OpIol},
    refraction::{OpRefraction, Refraction},
    sca::Sca,
    sia::Sia,
    surgeon::{Surgeon, SurgeonSia},
    target::{Constant, Target},
    va::{AfterVaSet, BeforeVaSet, FarVa, NearVa, OpVa, Va},
};
use chrono::NaiveDate;
use edgedb_derive::Queryable;
use polars::prelude::PolarsError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FcError {
    #[error("Polars error: {0:?}")]
    Polars(PolarsError),
    #[error("Serde error: {0:?}")]
    Serde(serde_json::error::Error),
}

impl From<PolarsError> for FcError {
    fn from(error: PolarsError) -> Self { Self::Polars(error) }
}

impl From<serde_json::error::Error> for FcError {
    fn from(error: serde_json::error::Error) -> Self { Self::Serde(error) }
}

/// A flattened version of the [`Case`](crate::case::Case) struct for use in database queries and
/// the initial ingestion of CSV data.
// todo: this likely needs to be flattened _completely_, which means bringing target_formula in
// line with the DB by matching on a String value, rather than expecting an enum (Case can keep an
// enum)
#[derive(Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct FlatCase {
    // todo: should Surgeon and Iol details be removed from FlatCase?
    pub surgeon_email: Option<String>,
    pub surgeon_first_name: Option<String>,
    pub surgeon_last_name: Option<String>,
    pub surgeon_site: Option<String>,
    pub surgeon_sia_right_power: Option<f32>,
    pub surgeon_sia_right_axis: Option<i32>,
    pub surgeon_sia_left_power: Option<f32>,
    pub surgeon_sia_left_axis: Option<i32>,

    #[serde(rename = "URN*")]
    pub urn: Option<String>,
    #[serde(rename = "side*")]
    pub side: Option<String>,

    #[serde(rename = "constant")]
    pub target_constant: Option<f32>,
    #[serde(rename = "formula")]
    pub target_formula: Option<String>,
    #[serde(rename = "target refraction sphere")]
    pub target_se: Option<f32>,
    #[serde(rename = "target refraction cyl")]
    pub target_cyl_power: Option<f32>,
    #[serde(rename = "target refraction axis")]
    pub target_cyl_axis: Option<i32>,

    #[serde(rename = "date of surgery")]
    pub date: Option<NaiveDate>,
    #[serde(rename = "hospital")]
    pub site: Option<String>,

    #[serde(rename = "SIA power")]
    pub sia_power: Option<f32>,
    #[serde(rename = "SIA axis")]
    pub sia_meridian: Option<i32>,

    #[serde(rename = "IOL model")]
    pub iol_surgeon_label: Option<String>,
    pub iol_model: Option<String>,
    pub iol_name: Option<String>,
    pub iol_focus: Option<String>,
    pub iol_toric: Option<bool>,
    #[serde(rename = "IOL sphere")]
    pub iol_se: Option<f32>,
    #[serde(rename = "IOL cyl")]
    pub iol_cyl_power: Option<f32>,
    #[serde(rename = "IOL axis")]
    pub iol_cyl_axis: Option<i32>,

    #[serde(rename = "adverse event")]
    pub adverse: Option<String>,

    #[serde(rename = "preop BCVA numerator*")]
    pub va_best_before_num: Option<f32>,
    #[serde(rename = "preop BCVA denominator*")]
    pub va_best_before_den: Option<f32>,

    #[serde(rename = "postop BCVA numerator")]
    pub va_best_after_num: Option<f32>,
    #[serde(rename = "postop BCVA denominator")]
    pub va_best_after_den: Option<f32>,

    #[serde(rename = "postop UCVA numerator*")]
    pub va_raw_after_num: Option<f32>,
    #[serde(rename = "postop UCVA denominator*")]
    pub va_raw_after_den: Option<f32>,

    #[serde(rename = "postop near UCVA numerator")]
    pub va_raw_near_after_num: Option<f32>,
    #[serde(rename = "postop near UCVA denominator")]
    pub va_raw_near_after_den: Option<f32>,

    #[serde(rename = "preop refraction sphere*")]
    pub ref_before_sph: Option<f32>,
    #[serde(rename = "preop refraction cyl")]
    pub ref_before_cyl_power: Option<f32>,
    #[serde(rename = "preop refraction axis")]
    pub ref_before_cyl_axis: Option<i32>,

    #[serde(rename = "postop refraction sphere*")]
    pub ref_after_sph: Option<f32>,
    #[serde(rename = "postop refraction cyl")]
    pub ref_after_cyl_power: Option<f32>,
    #[serde(rename = "postop refraction axis")]
    pub ref_after_cyl_axis: Option<i32>,
}

impl FlatCase {
    pub fn from_csv(path: &Path) -> Result<Vec<Self>, FcError> {
        let ws = WriteString::new_from_csv(path)?;
        let json = &ws.0[..];
        let fc: Vec<Self> = serde_json::from_str(json)?;

        Ok(fc)
    }
}

impl From<Case> for FlatCase {
    fn from(case: Case) -> Self {
        let Case {
            surgeon:
                Surgeon {
                    email: surgeon_email,
                    first_name: surgeon_first_name,
                    last_name: surgeon_last_name,
                    site: surgeon_site,
                    sia: surgeon_sia,
                },

            urn,
            side,
            target,
            date,
            site,
            sia,
            iol,
            adverse,

            va:
                OpVa {
                    before:
                        BeforeVaSet {
                            best_far:
                                FarVa(Va {
                                    num: va_best_before_num,
                                    den: va_best_before_den,
                                }),
                        },

                    after:
                        AfterVaSet {
                            best_far: va_best_after,
                            raw_far:
                                FarVa(Va {
                                    num: va_raw_after_num,
                                    den: va_raw_after_den,
                                }),
                            raw_near: va_raw_near_after,
                        },
                },

            refraction:
                OpRefraction {
                    before:
                        Refraction(Sca {
                            sph: ref_before_sph,
                            cyl: ref_before_cyl,
                        }),
                    after:
                        Refraction(Sca {
                            sph: ref_after_sph,
                            cyl: ref_after_cyl,
                        }),
                },
        } = case;

        let (
            surgeon_sia_right_power,
            surgeon_sia_right_axis,
            surgeon_sia_left_power,
            surgeon_sia_left_axis,
        ) = match surgeon_sia {
            Some(SurgeonSia {
                right:
                    Sia(Cyl {
                        power: right_power,
                        axis: right_axis,
                    }),
                left:
                    Sia(Cyl {
                        power: left_power,
                        axis: left_axis,
                    }),
            }) => (
                Some(right_power),
                Some(right_axis.0),
                Some(left_power),
                Some(left_axis.0),
            ),

            None => (None, None, None, None),
        };

        let side = match side {
            Side::Right => String::from("right"),
            Side::Left => String::from("left"),
        };

        let (target_constant, target_formula, target_se, target_cyl_power, target_cyl_axis) =
            match target {
                Some(Target {
                    constant,
                    sca: Sca { sph, cyl },
                }) => {
                    let (constant, formula) = match constant {
                        Some(Constant { value, formula }) => {
                            (Some(value), Some(formula.to_string()))
                        }

                        None => (None, None),
                    };

                    let se = Some(sph);

                    let (cyl_power, cyl_axis) = match cyl {
                        Some(Cyl { power, axis }) => (Some(power), Some(axis.0)),

                        None => (None, None),
                    };

                    (constant, formula, se, cyl_power, cyl_axis)
                }

                None => (None, None, None, None, None),
            };

        let (sia_power, sia_meridian) = match sia {
            Some(Sia(Cyl { power, axis })) => (Some(power), Some(axis.0)),

            None => (None, None),
        };

        let (
            iol_surgeon_label,
            iol_model,
            iol_name,
            iol_focus,
            iol_toric,
            iol_se,
            iol_cyl_power,
            iol_cyl_axis,
        ) = match iol {
            Some(OpIol {
                surgeon_label,
                iol,
                sca: Sca { sph, cyl },
            }) => {
                let (model, name, focus, toric) = match iol {
                    Some(Iol {
                        model,
                        name,
                        focus,
                        toric,
                    }) => {
                        let focus = match focus {
                            Focus::Mono => "mono".to_string(),
                            Focus::Edof => "edof".to_string(),
                            Focus::Multi => "multi".to_string(),
                        };

                        (Some(model), Some(name), Some(focus), Some(toric))
                    }

                    None => (None, None, None, None),
                };

                let se = Some(sph);

                let (cyl_power, cyl_axis) = match cyl {
                    Some(Cyl { power, axis }) => (Some(power), Some(axis.0)),

                    None => (None, None),
                };

                (
                    surgeon_label,
                    model,
                    name,
                    focus,
                    toric,
                    se,
                    cyl_power,
                    cyl_axis,
                )
            }

            None => (None, None, None, None, None, None, None, None),
        };

        let adverse = if let Some(adverse) = adverse {
            let adverse = match adverse {
                Adverse::Rhexis => "rhexis",
                Adverse::Pc => "pc",
                Adverse::Zonule => "zonule",
                Adverse::Other => "other",
            };

            Some(adverse.to_string())
        } else {
            None
        };

        let (va_best_after_num, va_best_after_den) = match va_best_after {
            Some(FarVa(Va { num, den })) => (Some(num), Some(den)),

            None => (None, None),
        };

        let (va_raw_near_after_num, va_raw_near_after_den) = match va_raw_near_after {
            Some(NearVa(Va { num, den })) => (Some(num), Some(den)),

            None => (None, None),
        };

        let (ref_before_cyl_power, ref_before_cyl_axis) = match ref_before_cyl {
            Some(Cyl { power, axis }) => (Some(power), Some(axis.0)),

            None => (None, None),
        };

        let (ref_after_cyl_power, ref_after_cyl_axis) = match ref_after_cyl {
            Some(Cyl { power, axis }) => (Some(power), Some(axis.0)),

            None => (None, None),
        };

        let fc = FlatCase {
            surgeon_email: Some(surgeon_email),
            surgeon_first_name,
            surgeon_last_name,
            surgeon_site,
            surgeon_sia_right_power,
            surgeon_sia_right_axis,
            surgeon_sia_left_power,
            surgeon_sia_left_axis,
            urn: Some(urn),
            side: Some(side),
            target_constant,
            target_formula,
            target_se,
            target_cyl_power,
            target_cyl_axis,
            date: Some(date),
            site,
            sia_power,
            sia_meridian,
            iol_surgeon_label,
            iol_model,
            iol_name,
            iol_focus,
            iol_toric,
            iol_se,
            iol_cyl_power,
            iol_cyl_axis,
            adverse,
            va_best_before_num: Some(va_best_before_num),
            va_best_before_den: Some(va_best_before_den),
            va_best_after_num,
            va_best_after_den,
            va_raw_after_num: Some(va_raw_after_num),
            va_raw_after_den: Some(va_raw_after_den),
            va_raw_near_after_den,
            va_raw_near_after_num,
            ref_before_sph: Some(ref_before_sph),
            ref_before_cyl_axis,
            ref_before_cyl_power,
            ref_after_sph: Some(ref_after_sph),
            ref_after_cyl_power,
            ref_after_cyl_axis,
        };

        fc
    }
}
