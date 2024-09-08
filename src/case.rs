#[cfg(feature = "ssr")] use crate::db::DbCase;
use crate::{
    bounds_check::Checked,
    iol::{IolBoundsError, OpIol},
    refraction::{OpRefraction, RefractionBoundsError},
    sca::ScaBoundsError,
    sia::{Sia, SiaBoundsError},
    surgeon::Surgeon,
    target::{Target, TargetBoundsError},
    va::{OpVa, VaBoundsError},
};
use chrono::NaiveDate;
#[cfg(feature = "ssr")] use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A wrapper for any type of bounds error on numeric types.
#[derive(Debug, Error)]
pub enum BoundsError {
    #[error("IOL bounds error: ({0:?})")]
    Iol(IolBoundsError),

    #[error("refraction bounds error: ({0:?})")]
    Refraction(RefractionBoundsError),

    #[error("SCA bounds error: ({0:?})")]
    Sca(ScaBoundsError),

    #[error("SIA bounds error: ({0:?})")]
    Sia(SiaBoundsError),

    #[error("target bounds error: ({0:?})")]
    Target(TargetBoundsError),

    #[error("VA bounds error: ({0:?})")]
    Va(VaBoundsError),
}

/// The required fields for each [`Case`]. Used by [`CaseError::MissingField`].
#[derive(Debug, PartialEq)]
pub enum Required {
    Email,
    Urn,
    Side,
    Date,
    Va,
    Refraction,
}

/// The error type for a [`Case`] with missing fields or out of bounds values.
#[derive(Debug, Error)]
pub enum CaseError {
    #[error("out of bounds value on a `Case`: {0:?}")]
    Bounds(BoundsError),

    #[error("{0:?} is a required field on `Case`, but wasn't supplied")]
    MissingField(Required),
}

impl From<IolBoundsError> for CaseError {
    fn from(err: IolBoundsError) -> Self {
        Self::Bounds(BoundsError::Iol(err))
    }
}

impl From<RefractionBoundsError> for CaseError {
    fn from(err: RefractionBoundsError) -> Self {
        Self::Bounds(BoundsError::Refraction(err))
    }
}

impl From<ScaBoundsError> for CaseError {
    fn from(err: ScaBoundsError) -> Self {
        Self::Bounds(BoundsError::Sca(err))
    }
}

impl From<SiaBoundsError> for CaseError {
    fn from(err: SiaBoundsError) -> Self {
        Self::Bounds(BoundsError::Sia(err))
    }
}

impl From<TargetBoundsError> for CaseError {
    fn from(err: TargetBoundsError) -> Self {
        Self::Bounds(BoundsError::Target(err))
    }
}

impl From<VaBoundsError> for CaseError {
    fn from(err: VaBoundsError) -> Self {
        Self::Bounds(BoundsError::Va(err))
    }
}

/// The side of the patient's surgery.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub enum Side {
    Right,
    Left,
}

/// An adverse intraoperative event. Classification is at the surgeon's discretion, and only one
/// option can be selected. For example, a wrap around split in the rhexis opens the PC, but in the
/// surgeon's view it may be essentially a rhexis complication. For our purposes, we aren't
/// particularly concerned with how the adverse event was handled (for example, whether a
/// vitrectomy was required). We are interested only in the relative outcomes of cases with adverse
/// events versus those without.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub enum Adverse {
    Rhexis,
    Pc,
    Zonule,
    Other,
}

/// A single surgical case. In the future, biometry values may be added.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Case {
    pub surgeon: Surgeon,
    /// A unique value that allows (only) the surgeon to deanonymize the case.
    pub urn: String,
    pub side: Side,
    /// The surgeon's intended refractive target, based on the formula of their choice.
    pub target: Option<Target<Checked>>,
    pub date: NaiveDate,
    /// The institution where surgery was performed.
    pub site: Option<String>,
    /// An [`Sia`] specific to this case. If no SIA is provided at the case level, the surgeon
    /// defaults will be used.
    pub sia: Option<Sia>,
    pub iol: Option<OpIol<Checked>>,
    pub adverse: Option<Adverse>,
    pub va: OpVa,
    pub refraction: OpRefraction,
}

#[cfg(feature = "ssr")]
impl TryFrom<DbCase> for Case {
    type Error = CaseError;

    fn try_from(dbcase: DbCase) -> Result<Self, Self::Error> {
        todo!()
    }
}

mod tests {
    use super::*;
    use crate::{
        bounds_check::BoundsCheck,
        cyl::Cyl,
        iol::{Focus, Iol},
        refraction::Refraction,
        surgeon::SurgeonSia,
        target::{Constant, Formula},
        va::{AfterVa, BeforeVa, Va},
    };
    #[cfg(feature = "ssr")] use edgedb_tokio::create_client;
    use std::marker::PhantomData;
    #[cfg(feature = "ssr")] use tokio::test;

    #[cfg(feature = "ssr")]
    #[tokio::test]
    async fn inserts_a_case() {
        let client = edgedb_tokio::create_client()
            .await
            .expect("DB client to be initialized");

        let Case {
            urn,
            side,
            target,
            date,
            site,
            sia,
            iol,
            adverse,
            va,
            refraction,
            ..
        } = case();

        let (target_constant, target_se, target_cyl) = if let Some(target) = target {
            Target {
                constant,
                se,
                cyl,
                ..
            } = target;
        };

        let args = (urn, side, target_constant, target_se, target_cyl);

        let result = client
            .query("select 1 + 1", &())
            .await
            .unwrap()
            .iter()
            .map(|res| println!("{res:?}"));
    }

    fn case() -> Case {
        Case {
            surgeon: Surgeon {
                email: "email@email.com".to_string(),
                first_name: Some("john".to_string()),
                last_name: Some("smith".to_string()),
                site: None,
                sia: Some(SurgeonSia {
                    right: Sia {
                        power: 010,
                        axis: 100,
                    },
                    left: Sia {
                        power: 010,
                        axis: 100,
                    },
                }),
            },

            urn: "abc123".to_string(),
            side: Side::Right,

            target: Some(
                Target {
                    constant: Some(Constant {
                        value: 11936,
                        formula: Formula::Kane,
                    }),
                    se: 1950,
                    cyl: Some(Cyl {
                        power: -015,
                        axis: 90,
                    }),
                    bounds: PhantomData,
                }
                .check()
                .unwrap(),
            ),

            date: NaiveDate::from_ymd_opt(2022, 3, 15).unwrap(),
            site: Some("RMH".to_string()),
            sia: None,

            iol: Some(
                OpIol {
                    iol: Iol {
                        model: "zxr00v".to_string(),
                        name: "Symfony".to_string(),
                        company: "Johnson and Johnson".to_string(),
                        focus: Focus::Edof,
                        toric: false,
                    },
                    se: 2400,
                    cyl: None,
                    bounds: PhantomData,
                }
                .check()
                .unwrap(),
            ),

            adverse: Some(Adverse::Pc),

            va: OpVa {
                before: BeforeVa {
                    best: Va {
                        num: 600,
                        den: 1200,
                    },
                    raw: None,
                },
                after: AfterVa {
                    best: None,
                    raw: Va { num: 600, den: 500 },
                },
            },

            refraction: OpRefraction {
                before: Refraction {
                    sph: 300,
                    cyl: Some(Cyl {
                        power: -125,
                        axis: 45,
                    }),
                    bounds: PhantomData,
                }
                .check()
                .unwrap(),
                after: Refraction {
                    sph: -025,
                    cyl: Some(Cyl {
                        power: -025,
                        axis: 60,
                    }),
                    bounds: PhantomData,
                }
                .check()
                .unwrap(),
            },
        }
    }
}
