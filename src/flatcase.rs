use crate::{
    case::{Adverse, Case, Side},
    cyl::Cyl,
    iol::{Focus, Iol, OpIol},
    refraction::{OpRefraction, Refraction},
    sca::Sca,
    sia::Sia,
    surgeon::Surgeon,
    target::{Constant, Formula, Target},
    va::{AfterVaSet, BeforeVaSet, FarVa, OpVa, Va},
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

    pub target_constant: Option<f32>,
    pub target_formula: Option<String>,
    pub target_se: Option<f32>,
    pub target_cyl_power: Option<f32>,
    pub target_cyl_axis: Option<i32>,

    pub date: Option<NaiveDate>,
    pub site: Option<String>,

    pub sia_power: Option<f32>,
    pub sia_meridian: Option<i32>,

    pub iol_surgeon_label: Option<String>,
    pub iol_model: Option<String>,
    pub iol_name: Option<String>,
    pub iol_focus: Option<Focus>,
    pub iol_toric: Option<bool>,
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
                    before:
                        BeforeVaSet {
                            best_far:
                                FarVa(Va {
                                    num: va_best_before_num,
                                    den: va_best_before_den,
                                }),
                            raw_far: va_raw_before,
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
                            sph: ref_before_sph,
                            cyl: ref_after_cyl,
                        }),
                },
        } = case;

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
                    }) => (Some(model), Some(name), Some(focus), Some(toric)),

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

        let (va_raw_before_num, va_raw_before_den) = match va_raw_before {
            Some(FarVa(Va { num, den })) => (Some(num), Some(den)),

            None => (None, None),
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
            va_raw_before_num,
            va_raw_before_den,
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
