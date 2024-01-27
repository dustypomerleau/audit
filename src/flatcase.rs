use crate::{
    case::{Adverse, Case, Side},
    cyl::Cyl,
    distance::Far,
    refraction::{OpRefraction, Refraction},
    sca::Sca,
    surgeon::Surgeon,
    target::Target,
    va::{FarVaSet, OpVa, Va},
};
use chrono::NaiveDate;
use edgedb_derive::Queryable;

/// A flattened version of the [`Case`](crate::case::Case) struct for use in database queries and
/// the initial ingestion of CSV data.
// todo: this likely needs to be flattened _completely_, which means bringing target_formula in
// line with the DB by matching on a String value, rather than expecting an enum (Case can keep an
// enum)
#[derive(Debug, PartialEq, Queryable)]
pub struct FlatCase {
    pub surgeon_email: Option<String>,
    pub surgeon_first_name: Option<String>,
    pub surgeon_last_name: Option<String>,
    pub surgeon_site: Option<String>,
    pub urn: Option<String>,
    pub side: Option<Side>,
    pub target_formula: Option<String>,
    pub target_se: Option<f32>,
    pub target_cyl_power: Option<f32>,
    pub target_cyl_axis: Option<i32>,
    pub date: Option<NaiveDate>,
    pub site: Option<String>,
    pub sia_power: Option<f32>,
    pub sia_meridian: Option<i32>,
    pub iol_se: Option<f32>,
    pub iol_cyl_power: Option<f32>,
    pub iol_cyl_axis: Option<i32>,
    pub adverse: Option<Adverse>,

    pub va_best_before_num: Option<f32>,
    pub va_best_before_den: Option<f32>,
    pub va_best_after_num: Option<f32>,
    pub va_best_after_den: Option<f32>,

    pub va_raw_before_num: Option<f32>,
    pub va_raw_before_den: Option<f32>,
    pub va_raw_after_num: Option<f32>,
    pub va_raw_after_den: Option<f32>,

    pub va_best_near_before_num: Option<f32>,
    pub va_best_near_before_den: Option<f32>,
    pub va_best_near_after_num: Option<f32>,
    pub va_best_near_after_den: Option<f32>,

    pub va_raw_near_before_num: Option<f32>,
    pub va_raw_near_before_den: Option<f32>,
    pub va_raw_near_after_num: Option<f32>,
    pub va_raw_near_after_den: Option<f32>,

    pub ref_before_sph: Option<f32>,
    pub ref_before_cyl_power: Option<f32>,
    pub ref_before_cyl_axis: Option<i32>,

    pub ref_after_sph: Option<f32>,
    pub ref_after_cyl_power: Option<f32>,
    pub ref_after_cyl_axis: Option<i32>,
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
                    best_far:
                        FarVaSet {
                            before:
                                Far(Va {
                                    num: va_best_before_num,
                                    den: va_best_before_den,
                                }),
                            after:
                                Far(Va {
                                    num: va_best_after_num,
                                    den: va_best_after_den,
                                }),
                        },
                    best_near,
                    raw_far,
                    raw_near,
                },
            refraction:
                OpRefraction {
                    before:
                        Far(Refraction(Sca {
                            sph: ref_before_sph,
                            cyl:
                                Some(Cyl {
                                    power: ref_before_cyl_power,
                                    axis: ref_before_cyl_axis,
                                }),
                        })),
                    after:
                        Far(Refraction(Sca {
                            sph: ref_after_sph,
                            cyl:
                                Some(Cyl {
                                    power: ref_after_cyl_power,
                                    axis: ref_after_cyl_axis,
                                }),
                        })),
                },
        } = case;

        let (target_formula, target_se, target_cyl_power, target_cyl_axis) = match target {
            Some(Target {
                formula: target_formula,
                sca:
                    Sca {
                        sph: target_se,
                        cyl:
                            Some(Cyl {
                                power: target_cyl_power,
                                axis: target_cyl_axis,
                            }),
                    },
            }) => (
                target_formula,
                Some(target_se),
                Some(target_cyl_power),
                Some(*target_cyl_axis),
            ),

            Some(Target {
                formula: target_formula,
                sca:
                    Sca {
                        sph: target_se,
                        cyl: None,
                    },
            }) => (target_formula, Some(target_se), None, None),

            None => (None, None, None, None),
        };

        let fc = FlatCase {
            surgeon_email: Some(surgeon_email),
            surgeon_first_name,
            surgeon_last_name,
            surgeon_site,
            urn: Some(urn),
            side: Some(side),
            target_formula,
            target_se,
            target_cyl_power,
            target_cyl_axis,
        };

        fc
    }
}
