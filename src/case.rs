use crate::{
    cyl::{Cyl, CylPair},
    flatcase::FlatCase,
    iol::{Focus, Iol, IolBoundsError, OpIol},
    refraction::{OpRefraction, RefBoundsError, Refraction},
    sca::{Sca, ScaBoundsError},
    sia::{Sia, SiaBoundsError},
    surgeon::{Surgeon, SurgeonSia},
    target::{Constant, ConstantPair, Formula, Target, TargetBoundsError},
    va::{AfterVaSet, BeforeVaSet, FarVa, NearVa, OpVa, Va, VaBoundsError},
};
use chrono::NaiveDate;
use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A wrapper for any type of bounds error.
#[derive(Debug, Error)]
pub enum BoundsError {
    #[error("IOL bounds error: ({0:?})")]
    Iol(IolBoundsError),

    #[error("refraction bounds error: ({0:?})")]
    Ref(RefBoundsError),

    #[error("SCA bounds error: ({0:?})")]
    Sca(ScaBoundsError),

    #[error("SIA bounds error: ({0:?})")]
    Sia(SiaBoundsError),

    #[error("target bounds error: ({0:?})")]
    Target(TargetBoundsError),

    #[error("VA bounds error: ({0:?})")]
    Va(VaBoundsError),
}

impl From<IolBoundsError> for CaseError {
    fn from(err: IolBoundsError) -> Self { Self::Bounds(BoundsError::Iol(err)) }
}

impl From<RefBoundsError> for CaseError {
    fn from(err: RefBoundsError) -> Self { Self::Bounds(BoundsError::Ref(err)) }
}

impl From<ScaBoundsError> for CaseError {
    fn from(err: ScaBoundsError) -> Self { Self::Bounds(BoundsError::Sca(err)) }
}

impl From<SiaBoundsError> for CaseError {
    fn from(err: SiaBoundsError) -> Self { Self::Bounds(BoundsError::Sia(err)) }
}

impl From<TargetBoundsError> for CaseError {
    fn from(err: TargetBoundsError) -> Self { Self::Bounds(BoundsError::Target(err)) }
}

impl From<VaBoundsError> for CaseError {
    fn from(err: VaBoundsError) -> Self { Self::Bounds(BoundsError::Va(err)) }
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

/// The side of the patient's surgery.
#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
#[serde(rename_all = "lowercase")]
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
#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
#[serde(rename_all = "lowercase")]
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
    /// A unique value provided by the surgeon, such that deanonymization may only be performed by
    /// the surgeon.
    pub urn: String,
    pub side: Side,
    /// The surgeon's intended refractive target, based on the formula of their choice.
    pub target: Option<Target>,
    pub date: NaiveDate,
    /// The institution where surgery was performed.
    pub site: Option<String>,
    // If no SIA is provided at the case level, the surgeon's defaults will be used.
    pub sia: Option<Sia>,
    pub iol: Option<OpIol>,
    pub adverse: Option<Adverse>,
    pub va: OpVa,
    pub refraction: OpRefraction,
}

// This impl needs to surface detailed errors, because calling `FlatCase::try_into::<Case>()` is
// the primary way of bounds checking all the values obtained from the raw CSV before putting them
// in the DB. For this reason, we avoid calling `ok()` to get our `Option` types, and instead
// propagate the specific error.
impl TryFrom<FlatCase> for Case {
    type Error = CaseError;

    // note: We can consume the FlatCase here, because the Case will be used for subsequent DB
    // insertion.
    fn try_from(f: FlatCase) -> Result<Self, Self::Error> {
        let surgeon_sia = match (
            f.surgeon_sia_right_power,
            f.surgeon_sia_right_axis,
            f.surgeon_sia_left_power,
            f.surgeon_sia_left_axis,
        ) {
            (Some(right_power), Some(right_axis), Some(left_power), Some(left_axis)) => {
                Some(SurgeonSia {
                    right: Cyl::new(right_power, right_axis)?.try_into()?,
                    left: Cyl::new(left_power, left_axis)?.try_into()?,
                })
            }

            (None, None, None, None) => None,

            (Some(_power), ..) | (_, _, Some(_power), _) => {
                return Err(ScaBoundsError::NoPair(CylPair::Axis).into())
            }

            (_, Some(_axis), ..) | (_, _, _, Some(_axis)) => {
                return Err(ScaBoundsError::NoPair(CylPair::Power).into())
            }
        };

        let surgeon = if let Some(email) = f.surgeon_email {
            Surgeon {
                email,
                first_name: f.surgeon_first_name,
                last_name: f.surgeon_last_name,
                site: f.surgeon_site,
                sia: surgeon_sia,
            }
        } else {
            return Err(CaseError::MissingField(Required::Email));
        };

        let urn = if let Some(urn) = f.urn {
            urn
        } else {
            return Err(CaseError::MissingField(Required::Urn));
        };

        let side = if let Some(side) = f.side {
            match side.to_lowercase().as_str() {
                "right" => Side::Right,
                "r" => Side::Right,
                "re" => Side::Right,
                "od" => Side::Right,
                "left" => Side::Left,
                "l" => Side::Left,
                "le" => Side::Left,
                "os" => Side::Left,
                _ => return Err(CaseError::MissingField(Required::Side)),
            }
        } else {
            return Err(CaseError::MissingField(Required::Side));
        };

        let target_constant = match (f.target_constant, f.target_formula) {
            (Some(constant), Some(formula)) => {
                let constant = Constant {
                    value: constant,
                    formula: Formula::new_from_str(&formula)?,
                };

                Some(constant)
            }

            (None, None) => None,

            (None, _) => return Err(TargetBoundsError::NoPair(ConstantPair::Value).into()),

            (_, None) => return Err(TargetBoundsError::NoPair(ConstantPair::Formula).into()),
        };

        let target = if let Some(sph) = f.target_se {
            let target_sca = Sca::new(sph, f.target_cyl_power, f.target_cyl_axis)?;
            let target = Target::new(target_constant, target_sca)?;

            Some(target)
        } else {
            None
        };

        let date = if let Some(date) = f.date {
            date
        } else {
            return Err(CaseError::MissingField(Required::Date));
        };

        let site = f.site;

        let sia = match (f.sia_power, f.sia_axis) {
            (Some(power), Some(axis)) => {
                let sia = Cyl::new(power, axis)?.try_into()?;

                Some(sia)
            }

            (None, None) => None,

            (Some(_power), _) => return Err(ScaBoundsError::NoPair(CylPair::Axis).into()),

            (_, Some(_axis)) => return Err(ScaBoundsError::NoPair(CylPair::Power).into()),
        };

        let iol = match f.iol_se {
            Some(se) => {
                let (cyl, axis) = match (f.iol_cyl_power, f.iol_cyl_axis) {
                    (Some(power), Some(axis)) => (Some(power), Some(axis)),

                    (None, None) => (None, None),

                    (Some(_power), _) => return Err(IolBoundsError::NoPair(CylPair::Axis).into()),

                    (_, Some(_axis)) => return Err(IolBoundsError::NoPair(CylPair::Power).into()),
                };

                let surgeon_label = f.iol_surgeon_label;

                let iol = match (f.iol_model, f.iol_name, f.iol_focus, f.iol_toric) {
                    (Some(model), Some(name), Some(focus), Some(toric)) => {
                        let focus = match focus.to_lowercase().as_str() {
                            "monofocal" => Focus::Mono,
                            "mono" => Focus::Mono,
                            "multifocal" => Focus::Multi,
                            "multi" => Focus::Multi,
                            "edof" => Focus::Edof,
                            _ => return Err(IolBoundsError::Iol.into()),
                        };

                        Some(Iol {
                            model,
                            name,
                            focus,
                            toric,
                        })
                    }

                    (None, None, None, None) => None,

                    (..) => return Err(IolBoundsError::Iol.into()),
                };

                let sca = Sca::new(se, cyl, axis)?;

                Some(OpIol::new(surgeon_label, iol, sca)?)
            }

            // If no sphere is provided, we confirm that no cylinder power is provided. If there is
            // a cyl, we expect a matching sph, so we return an error. If there is no cylinder
            // power, we can skip the check on axis and just assume there is no `Iol` for this
            // `Case`.
            None => match f.iol_cyl_power {
                Some(_power) => return Err(IolBoundsError::NoSe.into()),

                None => None,
            },
        };

        let adverse = match f.adverse {
            Some(adverse) => match adverse.to_lowercase().as_str() {
                "rhexis" => Some(Adverse::Rhexis),
                "pc" => Some(Adverse::Pc),
                "zonule" => Some(Adverse::Zonule),
                "other" => Some(Adverse::Other),
                _ => None,
            },

            None => None,
        };

        let before = BeforeVaSet {
            best_far: if let Some(va) = Va::try_new(f.va_best_before_num, f.va_best_before_den)? {
                FarVa(va)
            } else {
                return Err(CaseError::MissingField(Required::Va));
            },
        };

        let after = AfterVaSet {
            best_far: if let Some(va) = Va::try_new(f.va_best_after_num, f.va_best_after_den)? {
                Some(FarVa(va))
            } else {
                None
            },

            raw_far: if let Some(va) = Va::try_new(f.va_raw_after_num, f.va_raw_after_den)? {
                FarVa(va)
            } else {
                return Err(CaseError::MissingField(Required::Va));
            },

            raw_near: if let Some(va) =
                Va::try_new(f.va_raw_near_after_num, f.va_raw_near_after_den)?
            {
                Some(NearVa(va))
            } else {
                None
            },
        };

        let va = OpVa { before, after };

        let refraction = {
            if let (Some(before_sph), Some(after_sph)) = (f.ref_before_sph, f.ref_after_sph) {
                let before: Refraction =
                    Sca::new(before_sph, f.ref_before_cyl_power, f.ref_before_cyl_axis)?
                        .try_into()?;
                let after: Refraction =
                    Sca::new(after_sph, f.ref_after_cyl_power, f.ref_after_cyl_axis)?.try_into()?;

                OpRefraction { before, after }
            } else {
                return Err(CaseError::MissingField(Required::Refraction));
            }
        };

        let case = Case {
            surgeon,
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
        };

        Ok(case)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // todo: eventually this will be replaced with a series of mocked `FlatCases` with random but
    // legal values.
    fn flatcase() -> FlatCase {
        FlatCase {
            surgeon_email: Some("testemail@email.com".to_string()),
            surgeon_first_name: Some("john".to_string()),
            surgeon_last_name: Some("wick".to_string()),
            surgeon_site: Some("the hospital".to_string()),
            surgeon_sia_right_power: Some(0.1),
            surgeon_sia_right_axis: Some(10),
            surgeon_sia_left_power: Some(0.1),
            surgeon_sia_left_axis: Some(10),
            urn: Some("abc123".to_string()),
            side: Some("re".to_string()),
            target_constant: Some(119.36),
            target_formula: Some("Barrett".to_string()),
            target_se: Some(-0.2),
            target_cyl_power: Some(0.15),
            target_cyl_axis: Some(90),
            date: NaiveDate::from_ymd_opt(2023, 05, 01), // returns Option<NaiveDate>
            site: Some("the hospital site".to_string()),
            sia_power: Some(0.1),
            sia_axis: Some(100),
            iol_surgeon_label: Some("symfony toric".to_string()),
            iol_model: Some("ZXTxxx".to_string()),
            iol_name: Some("Tecnis Symfony".to_string()),
            iol_focus: Some("edof".to_string()),
            iol_toric: Some(true),
            iol_se: Some(24.5),
            iol_cyl_power: Some(3.25),
            iol_cyl_axis: Some(179),
            adverse: None,

            va_best_before_num: Some(6.0),
            va_best_before_den: Some(24.0),
            va_best_after_num: None,
            va_best_after_den: None,

            va_raw_after_num: Some(6.0),
            va_raw_after_den: Some(6.0),

            va_raw_near_after_num: Some(6.0),
            va_raw_near_after_den: Some(5.0),

            ref_before_sph: Some(-5.25),
            ref_before_cyl_power: Some(-1.50),
            ref_before_cyl_axis: Some(67),

            ref_after_sph: Some(-0.5),
            ref_after_cyl_power: Some(-0.5),
            ref_after_cyl_axis: Some(10),
        }
    }

    #[test]
    fn case_implements_try_from_flatcase() {
        let fc = flatcase();

        assert!(<FlatCase as TryInto<Case>>::try_into(fc).is_ok())
    }
}
