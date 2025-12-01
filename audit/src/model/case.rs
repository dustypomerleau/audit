use std::fmt::Display;

use audit_macro::RangeBounded;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
#[cfg(feature = "ssr")] use crate::error::AppError;
use crate::model::Biometry;
use crate::model::Formula;
use crate::model::OpIol;
use crate::model::OpRefraction;
use crate::model::OpVa;
use crate::model::Sia;
use crate::model::Site;
use crate::model::Target;

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

impl Side {
    pub fn to_db_side(&self) -> &str {
        match self {
            Self::Right => "Side.Right",
            Self::Left => "Side.Left",
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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct Main(#[bounded(range = 100..=600, default = 240, mock_range = 220..=275)] u32);

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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SurgeonCase {
    /// A unique value that allows (only) the surgeon to deanonymize the case.
    pub urn: String,
    pub date: NaiveDate,
    pub site: Option<Site>,
    #[serde(alias = "cas")]
    pub case: Case,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormCase {
    pub urn: String,
    pub date: String,         // prefill today
    pub site: Option<String>, // prefill default
    pub side: Side,
    pub al: f32,
    pub k1_power: f32,
    pub k1_axis: u32,
    pub k2_power: f32,
    pub k2_axis: u32,
    pub acd: f32,
    pub lt: f32,
    pub cct: Option<u32>,
    pub wtw: Option<f32>,
    pub formula: Formula, // prefill default
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

impl FormCase {
    #[cfg(feature = "ssr")]
    pub async fn into_surgeon_case(self) -> Result<SurgeonCase, AppError> {
        use crate::db::db;
        use crate::model::Acd;
        use crate::model::AfterVa;
        use crate::model::Al;
        use crate::model::Axis;
        use crate::model::BeforeVa;
        use crate::model::Cct;
        use crate::model::Iol;
        use crate::model::IolSe;
        use crate::model::K;
        use crate::model::Kpower;
        use crate::model::Ks;
        use crate::model::Lt;
        use crate::model::SiaPower;
        use crate::model::TargetCyl;
        use crate::model::TargetCylPower;
        use crate::model::TargetSe;
        use crate::model::Va;
        use crate::model::VaDen;
        use crate::model::VaNum;
        use crate::model::Wtw;

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

        let date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d")?;

        let site = site.map(|name| Site { name });

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
            wtw: wtw.and_then(|wtw| Wtw::new((wtw * 100.0) as u32).ok()),
        };

        let target_cyl = match (target_cyl_power, target_cyl_axis) {
            (Some(power), Some(axis)) => Some(TargetCyl::new(
                TargetCylPower::new((power * 100.0) as u32)?,
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

        let main = Main::new((main * 100.0) as u32)?;

        let sia = Sia::new(
            SiaPower::new((sia_power * 100.0) as u32)?,
            Axis::new(sia_axis)?,
        );

        // NOTE: For now we are assuming the IOL model is in the DB. To start, offer an option in
        // the datalist that the IOL is not listed, and have a DB option for that.
        let iol = if let Ok(Some(json)) = db()
            .await?
            .query_single_json(
                format!(
                    r#"
select Iol {{
    model, name, company, focus, toric
}} filter .model = "{iol_model}";
                    "#
                ),
                &(),
            )
            .await
        {
            let iol = serde_json::from_str::<Iol>(json.as_ref())?;

            OpIol {
                iol,
                se: IolSe::new((iol_se * 100.0) as i32)?,
                axis: iol_axis.and_then(|axis| Axis::new(axis).ok()),
            }
        } else {
            return Err(AppError::Db("the Iol is not present in the DB".to_string()));
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
                use crate::model::RawCyl;

                Some(RawCyl::new((power * 100.0) as i32, Axis::new(axis)?))
            }

            _ => None,
        };

        let ref_after_raw_cyl = match (ref_after_cyl_power, ref_after_cyl_axis) {
            (Some(power), Some(axis)) => {
                use crate::model::RawCyl;

                Some(RawCyl::new((power * 100.0) as i32, Axis::new(axis)?))
            }

            _ => None,
        };

        let refraction = OpRefraction {
            before: {
                use crate::model::RawSca;
                use crate::model::into_refraction;

                let sca = RawSca::new((ref_before_sph * 100.0) as i32, ref_before_raw_cyl);
                into_refraction(sca)?
            },
            after: {
                use crate::model::RawSca;
                use crate::model::into_refraction;

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
