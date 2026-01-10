use std::fmt::Display;

use audit_macro::RangeBounded;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::error::AppError;
use crate::model::Acd;
use crate::model::AfterVa;
use crate::model::Al;
use crate::model::Axis;
use crate::model::BeforeVa;
use crate::model::Biometry;
use crate::model::Cct;
use crate::model::Focus;
use crate::model::Formula;
use crate::model::Iol;
use crate::model::IolSe;
use crate::model::K;
use crate::model::Kpower;
use crate::model::Ks;
use crate::model::Lt;
use crate::model::OpIol;
use crate::model::OpRefraction;
use crate::model::OpVa;
use crate::model::RefCyl;
use crate::model::RefCylPower;
use crate::model::RefSph;
use crate::model::Refraction;
use crate::model::Sia;
use crate::model::SiaPower;
use crate::model::Site;
use crate::model::Target;
use crate::model::TargetCyl;
use crate::model::TargetCylPower;
use crate::model::TargetSe;
use crate::model::ToricPower;
use crate::model::Va;
use crate::model::VaDen;
use crate::model::VaNum;
use crate::model::Wtw;
use crate::model::Year;

/// The side of the patient's surgery.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "side"))]
pub enum Side {
    #[default]
    Right,
    Left,
}

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
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "adverse"))]
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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded(range = 100..=600, default = 240, mock_range = 220..=275)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Main(i32);

/// A single surgical case.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SurgeonCase {
    /// A unique value that allows (only) the surgeon to deanonymize the case. URNs and other
    /// unique identifiers are not permitted in the DB. We use `number` rather than `id` or
    /// `identifier` because those terms are easily confused with the `id: UUID` field. It is
    /// highly unlikely that we would ever require such a large number of cases, but using a
    /// [`i32`] guarantees that the value, which on the DB side is an auto-incrementing, positive,
    /// int32 (sequence), can never be out of bounds during deserialization.
    pub number: i32,
    pub date: NaiveDate,
    pub site: Option<Site>,
    #[serde(alias = "cas")]
    pub case: Case,
}

impl TryFrom<QuerySurgeonCase> for SurgeonCase {
    type Error = AppError;

    fn try_from(qsc: QuerySurgeonCase) -> Result<Self, Self::Error> {
        let QuerySurgeonCase {
            number,
            date,
            site_name,
            side,
            biometry_al,
            biometry_flat_k_power,
            biometry_flat_k_axis,
            biometry_steep_k_power,
            biometry_steep_k_axis,
            biometry_acd,
            biometry_lt,
            biometry_cct,
            biometry_wtw,
            target_formula,
            target_custom_constant,
            target_se,
            target_cyl_power,
            target_cyl_axis,
            main,
            sia_power,
            sia_axis,
            iol_model,
            iol_name,
            iol_company,
            iol_focus,
            iol_toric,
            iol_se,
            iol_axis,
            adverse,
            va_before_best_num,
            va_before_best_den,
            va_before_raw_num,
            va_before_raw_den,
            va_after_best_num,
            va_after_best_den,
            va_after_raw_num,
            va_after_raw_den,
            ref_before_sph,
            ref_before_cyl_power,
            ref_before_cyl_axis,
            ref_after_sph,
            ref_after_cyl_power,
            ref_after_cyl_axis,
            ..
        } = qsc;

        let site = site_name.map(|name| Site { name });

        let target_cyl = if let (Some(target_cyl_power), Some(target_cyl_axis)) =
            (target_cyl_power, target_cyl_axis)
        {
            Some(TargetCyl {
                power: target_cyl_power,
                axis: target_cyl_axis,
            })
        } else {
            None
        };

        let iol = if let (Some(model), Some(focus)) = (iol_model, iol_focus) {
            Some(Iol {
                model,
                name: iol_name,
                company: iol_company,
                focus,
                toric: iol_toric,
            })
        } else {
            None
        };

        let va = OpVa {
            before: BeforeVa {
                best: Va {
                    num: va_before_best_num,
                    den: va_before_best_den,
                },

                raw: if let (Some(num), Some(den)) = (va_before_raw_num, va_before_raw_den) {
                    Some(Va { num, den })
                } else {
                    None
                },
            },

            after: AfterVa {
                best: if let (Some(num), Some(den)) = (va_after_best_num, va_after_best_den) {
                    Some(Va { num, den })
                } else {
                    None
                },

                raw: Va {
                    num: va_after_raw_num,
                    den: va_after_raw_den,
                },
            },
        };

        let refraction = OpRefraction {
            before: Refraction {
                sph: ref_before_sph,

                cyl: if let (Some(power), Some(axis)) = (ref_before_cyl_power, ref_before_cyl_axis)
                {
                    Some(RefCyl { power, axis })
                } else {
                    None
                },
            },

            after: Refraction {
                sph: ref_after_sph,

                cyl: if let (Some(power), Some(axis)) = (ref_after_cyl_power, ref_after_cyl_axis) {
                    Some(RefCyl { power, axis })
                } else {
                    None
                },
            },
        };

        let case = Case {
            side,

            biometry: Biometry {
                al: biometry_al,

                ks: Ks::new(
                    K::new(biometry_flat_k_power, biometry_flat_k_axis),
                    K::new(biometry_steep_k_power, biometry_steep_k_axis),
                )?,

                acd: biometry_acd,
                lt: biometry_lt,
                cct: biometry_cct,
                wtw: biometry_wtw,
            },

            target: Target {
                formula: target_formula,
                custom_constant: target_custom_constant,
                se: target_se,
                cyl: target_cyl,
            },

            main,

            sia: Sia {
                power: sia_power,
                axis: sia_axis,
            },

            iol: OpIol {
                iol,
                se: iol_se,
                axis: iol_axis,
            },

            adverse,
            va,
            refraction,
        };

        Ok(SurgeonCase {
            number,
            date,
            site,
            case,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormCase {
    pub date: String,         // prefill today
    pub site: Option<String>, // prefill default
    pub side: Side,
    pub al: f32,
    pub k1_power: f32,
    pub k1_axis: i32,
    pub k2_power: f32,
    pub k2_axis: i32,
    pub acd: f32,
    pub lt: f32,
    pub cct: Option<i32>,
    pub wtw: Option<f32>,
    pub formula: Formula, // prefill default
    pub custom_constant: Option<String>,
    pub target_se: f32,
    pub target_cyl_power: Option<f32>,
    pub target_cyl_axis: Option<i32>,
    pub main: f32,         // prefill default
    pub sia_power: f32,    // prefill default
    pub sia_axis: i32,     // prefill default for side (needs signal)
    pub iol_model: String, // prefill default
    pub iol_se: f32,
    pub iol_axis: Option<i32>,   // cyl power is supplied by the Iol
    pub adverse: String,         // prefill "None"
    pub va_before_best_num: i32, // prefill 6
    pub va_before_best_den: f32,
    pub va_before_raw_num: Option<i32>,
    pub va_before_raw_den: Option<f32>,
    pub va_after_best_num: Option<i32>,
    pub va_after_best_den: Option<f32>,
    pub va_after_raw_num: i32, // prefill 6
    pub va_after_raw_den: f32,
    pub ref_before_sph: f32,
    pub ref_before_cyl_power: Option<f32>,
    pub ref_before_cyl_axis: Option<i32>,
    pub ref_after_sph: f32,
    pub ref_after_cyl_power: Option<f32>,
    pub ref_after_cyl_axis: Option<i32>,
}

impl FormCase {
    #[cfg(feature = "ssr")]
    pub async fn into_surgeon_case(self) -> Result<SurgeonCase, AppError> {
        use sqlx::query_as;

        use crate::db::db;
        use crate::model::Acd;
        use crate::model::AfterVa;
        use crate::model::Al;
        use crate::model::Axis;
        use crate::model::BeforeVa;
        use crate::model::Cct;
        use crate::model::Focus;
        use crate::model::Iol;
        use crate::model::IolSe;
        use crate::model::K;
        use crate::model::Kpower;
        use crate::model::Ks;
        use crate::model::Lt;
        use crate::model::RawCyl;
        use crate::model::RawSca;
        use crate::model::SiaPower;
        use crate::model::TargetCyl;
        use crate::model::TargetCylPower;
        use crate::model::TargetSe;
        use crate::model::ToricPower;
        use crate::model::Va;
        use crate::model::VaDen;
        use crate::model::VaNum;
        use crate::model::Wtw;

        let FormCase {
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
            va_before_best_num,
            va_before_best_den,
            va_before_raw_num,
            va_before_raw_den,
            va_after_best_num,
            va_after_best_den,
            va_after_raw_num,
            va_after_raw_den,
            ref_before_sph,
            ref_before_cyl_power,
            ref_before_cyl_axis,
            ref_after_sph,
            ref_after_cyl_power,
            ref_after_cyl_axis,
        } = self;

        let date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d")?;

        let site = site.map(|name| Site { name });

        // These integer casts intentionally truncate the float values.
        let biometry = Biometry {
            al: Al::new((al * 100.0) as i32)?,
            ks: Ks::new(
                K::new(Kpower::new((k1_power * 100.0) as i32)?, Axis::new(k1_axis)?),
                K::new(Kpower::new((k2_power * 100.0) as i32)?, Axis::new(k2_axis)?),
            )?,
            acd: Acd::new((acd * 100.0) as i32)?,
            lt: Lt::new((lt * 100.0) as i32)?,
            cct: cct.and_then(|cct| Cct::new(cct).ok()),
            wtw: wtw.and_then(|wtw| Wtw::new((wtw * 100.0) as i32).ok()),
        };

        let target_cyl = match (target_cyl_power, target_cyl_axis) {
            (Some(power), Some(axis)) => Some(TargetCyl::new(
                TargetCylPower::new((power * 100.0) as i32)?,
                Axis::new(axis)?,
            )),

            _ => None,
        };

        let target = Target {
            formula: Some(formula),
            custom_constant: custom_constant == Some("true".to_string()),
            se: TargetSe::new((target_se * 100.0) as i32)?,
            cyl: target_cyl,
        };

        let main = Main::new((main * 100.0) as i32)?;

        let sia = Sia::new(
            SiaPower::new((sia_power * 100.0) as i32)?,
            Axis::new(sia_axis)?,
        );

        // TODO: set up the form so that the Iol must be present in the DB.
        let iol = query_as!(
            Iol,
            r#"
select model, name, company, focus as "focus: Focus", toric as "toric: ToricPower"
from iol
where model = $1;
            "#,
            iol_model
        )
        .fetch_one(&db().await?)
        .await;

        let opiol = OpIol {
            iol: iol.ok(),
            se: IolSe::new((iol_se * 100.0) as i32)?,

            axis: if let Some(axis) = iol_axis {
                Some(Axis::new(axis)?)
            } else {
                None
            },
        };

        // Using standard serde parsing here would require you to have Adverse::None.
        // The benefit of Adverse::None is that you no longer need this value to be Option.
        // The downside is that now you can't just select all the DB Cas that have a complication
        // by looking to see if there is a value here. Instead you would need to check for values
        // != to Adverse.None.
        // Probably leave it as option, but think on it.
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
                    VaNum::new(va_before_best_num * 100)?,
                    VaDen::new((va_before_best_den * 100.0) as i32)?,
                ),

                raw: match (va_before_raw_num, va_before_raw_den) {
                    (Some(num), Some(den)) => Some(Va::new(
                        VaNum::new(num * 100)?,
                        VaDen::new((den * 100.0) as i32)?,
                    )),

                    _ => None,
                },
            },

            after: AfterVa {
                best: match (va_after_best_num, va_after_best_den) {
                    (Some(num), Some(den)) => Some(Va::new(
                        VaNum::new(num * 100)?,
                        VaDen::new((den * 100.0) as i32)?,
                    )),

                    _ => None,
                },

                raw: Va::new(
                    VaNum::new(va_after_raw_num * 100)?,
                    VaDen::new((va_after_raw_den * 100.0) as i32)?,
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
            before: RawSca::new((ref_before_sph * 100.0) as i32, ref_before_raw_cyl)
                .into_refraction()?,

            after: RawSca::new((ref_after_sph * 100.0) as i32, ref_after_raw_cyl)
                .into_refraction()?,
        };

        let case = Case {
            side,
            biometry,
            target,
            main,
            sia,
            iol: opiol,
            adverse,
            va,
            refraction,
        };

        Ok(SurgeonCase {
            date,
            site,
            case,
            ..Default::default()
        })
    }
}

#[derive(Clone, Debug)]
pub struct QueryCase {
    pub side: Side,
    pub biometry_al: Al,
    pub biometry_flat_k_power: Kpower,
    pub biometry_flat_k_axis: Axis,
    pub biometry_steep_k_power: Kpower,
    pub biometry_steep_k_axis: Axis,
    pub biometry_acd: Acd,
    pub biometry_lt: Lt,
    pub biometry_cct: Option<Cct>,
    pub biometry_wtw: Option<Wtw>,
    pub target_formula: Option<Formula>,
    pub target_custom_constant: bool,
    pub target_se: TargetSe,
    pub target_cyl_power: Option<TargetCylPower>,
    pub target_cyl_axis: Option<Axis>,
    pub main: Main,
    pub sia_power: SiaPower,
    pub sia_axis: Axis,
    pub iol_model: String,
    pub iol_name: Option<String>,
    pub iol_company: Option<String>,
    pub iol_focus: Focus,
    pub iol_toric: Option<ToricPower>,
    pub iol_se: IolSe,
    pub iol_axis: Option<Axis>,
    pub adverse: Option<Adverse>,
    pub va_before_best_num: VaNum,
    pub va_before_best_den: VaDen,
    pub va_before_raw_num: Option<VaNum>,
    pub va_before_raw_den: Option<VaDen>,
    pub va_after_best_num: Option<VaNum>,
    pub va_after_best_den: Option<VaDen>,
    pub va_after_raw_num: VaNum,
    pub va_after_raw_den: VaDen,
    pub ref_before_sph: RefSph,
    pub ref_before_cyl_power: Option<RefCylPower>,
    pub ref_before_cyl_axis: Option<Axis>,
    pub ref_after_sph: RefSph,
    pub ref_after_cyl_power: Option<RefCylPower>,
    pub ref_after_cyl_axis: Option<Axis>,
}

#[derive(Clone, Debug)]
pub struct QuerySurgeonCase {
    pub number: i32,
    pub date: NaiveDate,
    pub site_name: Option<String>,
    pub side: Side,
    pub biometry_al: Al,
    pub biometry_flat_k_power: Kpower,
    pub biometry_flat_k_axis: Axis,
    pub biometry_steep_k_power: Kpower,
    pub biometry_steep_k_axis: Axis,
    pub biometry_acd: Acd,
    pub biometry_lt: Lt,
    pub biometry_cct: Option<Cct>,
    pub biometry_wtw: Option<Wtw>,
    pub target_formula: Option<Formula>,
    pub target_custom_constant: bool,
    pub target_se: TargetSe,
    pub target_cyl_power: Option<TargetCylPower>,
    pub target_cyl_axis: Option<Axis>,
    pub year: Year,
    pub main: Main,
    pub sia_power: SiaPower,
    pub sia_axis: Axis,
    pub iol_model: Option<String>,
    pub iol_name: Option<String>,
    pub iol_company: Option<String>,
    pub iol_focus: Option<Focus>,
    pub iol_toric: Option<ToricPower>,
    pub iol_se: IolSe,
    pub iol_axis: Option<Axis>,
    pub adverse: Option<Adverse>,
    pub va_before_best_num: VaNum,
    pub va_before_best_den: VaDen,
    pub va_before_raw_num: Option<VaNum>,
    pub va_before_raw_den: Option<VaDen>,
    pub va_after_best_num: Option<VaNum>,
    pub va_after_best_den: Option<VaDen>,
    pub va_after_raw_num: VaNum,
    pub va_after_raw_den: VaDen,
    pub ref_before_sph: RefSph,
    pub ref_before_cyl_power: Option<RefCylPower>,
    pub ref_before_cyl_axis: Option<Axis>,
    pub ref_after_sph: RefSph,
    pub ref_after_cyl_power: Option<RefCylPower>,
    pub ref_after_cyl_axis: Option<Axis>,
}

#[cfg(test)]
mod tests {
    use crate::model::Focus;
    use crate::model::Iol;

    #[test]
    fn deserializes_iol() {
        let mut json = r#"{\"id": "2286e9f4-33b2-11f0-8c1d-9bf7694ed7c6", "name": "Acrysof IQ SN60WF", "focus": "Mono", "model": "sn60wf", "toric": null, "company": "Alcon", "created_at": "2025-05-18T06:34:22.494725+00:00"}"#.to_string();

        json.remove_matches("\\");

        let iol = Iol {
            model: "sn60wf".to_string(),
            name: Some("Acrysof IQ SN60WF".to_string()),
            company: Some("Alcon".to_string()),
            focus: Focus::Mono,
            toric: None,
        };

        let result = serde_json::from_str::<Iol>(json.as_str()).unwrap();
        assert_eq!(result, iol);
    }
}
