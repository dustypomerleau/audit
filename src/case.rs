use crate::{
    axis::Axis,
    cyl::{Cyl, CylPair},
    distance::Distance,
    flatcase::FlatCase,
    iol::{Iol, IolBoundsError},
    refraction::{OpRefraction, RefBoundsError},
    sca::{Sca, ScaBoundsError},
    sia::{Sia, SiaBoundsError},
    surgeon::Surgeon,
    target::{Target, TargetBoundsError},
    va::{DistanceVaSet, OpVa, Va, VaBoundsError, VaPair},
};
use thiserror::Error;
use time::Date;

/// A wrapper for any type of bounds error.
#[derive(Debug, Error)]
enum BoundsError {
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

/// A representation of the required fields for each [`Case`], for use in
/// [`CaseError::MissingField`].
#[derive(Debug, PartialEq)]
enum Required {
    Email,
    Urn,
    Side,
    Date,
    Va,
    Refraction,
}

/// The error type for a [`Case`] with missing mandatory fields or out of bounds values.
#[derive(Debug, Error)]
enum CaseError {
    #[error("out of bounds value on a `Case`: {0:?}")]
    Bounds(BoundsError),
    #[error("{0:?} is a required field on `Case`, but wasn't supplied")]
    MissingField(Required),
}

/// The side of the patient's surgery.
#[derive(Debug, PartialEq)]
pub enum Side {
    Right,
    Left,
}

/// An adverse intraoperative event. It's up to the surgeon to classify, and only one
/// option can be selected. For example, a wrap around split in the rhexis opens the PC, but it's
/// essentially a rhexis complication.
#[derive(Debug, PartialEq)]
pub enum Adverse {
    Rhexis,
    Pc,
    Zonule,
    Other,
}

/// A single surgical case. In the future, biometry values may be added.
#[derive(Debug, PartialEq)]
pub struct Case {
    surgeon: Surgeon,
    /// A unique value provided by the surgeon, such that deanonymization may only be performed by
    /// the surgeon.
    urn: String,
    side: Side,
    /// The surgeon's intended refractive target, based on the formula of their choice.
    target: Option<Target>,
    date: Date,
    /// The institution where surgery was performed.
    site: Option<String>,
    sia: Option<Sia>,
    iol: Option<Iol>,
    adverse: Option<Adverse>,
    va: OpVa,
    refraction: OpRefraction,
}

// This impl needs to surface detailed errors, because calling `FlatCase::try_into::<Case>()` is
// the primary way of bounds checking all the values obtained from the raw CSV before putting them
// in the DB.
impl TryFrom<FlatCase> for Case {
    type Error = CaseError;

    fn try_from(f: FlatCase) -> Result<Self, Self::Error> {
        let surgeon = if let Some(email) = f.surgeon_email {
            Surgeon {
                email,
                first_name: f.surgeon_first_name,
                last_name: f.surgeon_last_name,
                site: f.surgeon_site,
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
            side
        } else {
            return Err(CaseError::MissingField(Required::Side));
        };

        let target = if let Some(sph) = f.target_se {
            let target_sca = Sca::new(sph, f.target_cyl_power, f.target_cyl_axis)?;
            // Avoid calling `.ok()` in order to propagate the `TargetBoundsError`.
            let target = Target::new(f.target_formula, target_sca)?;
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

        let sia = match (f.sia_power, f.sia_meridian) {
            (Some(power), Some(meridian)) => {
                let sia = Cyl::new(power, meridian)?.try_into()?;
                Some(sia)
            }

            (Some(_power), _) => return Err(ScaBoundsError::NoPair(CylPair::Axis).into()),

            (_, Some(_meridian)) => return Err(ScaBoundsError::NoPair(CylPair::Power).into()),

            (..) => None,
        };

        let iol = if let Some(sph) = f.iol_se {
            let (cyl, axis) = match (f.iol_cyl_power, f.iol_cyl_axis) {
                (Some(power), Some(axis)) => (Some(power), Some(axis)),

                (Some(_power), _) => return Err(IolBoundsError::NoPair(CylPair::Axis).into()),

                (_, Some(_axis)) => return Err(IolBoundsError::NoPair(CylPair::Power).into()),

                (..) => (None, None),
            };

            let iol = Sca::new(sph, cyl, axis)?.try_into()?;

            Some(iol)
        } else {
            match (f.iol_cyl_power, f.iol_cyl_axis) {
                (None, None) => None,

                (..) => return Err(IolBoundsError::NoSe.into()),
            }
        };

        let adverse = f.adverse;

        // if you need this function anywhere else, move it to module va
        fn distance_va_set(
            num_before: Option<f32>,
            den_before: Option<f32>,
            num_after: Option<f32>,
            den_after: Option<f32>,
        ) -> Result<DistanceVaSet, VaBoundsError> {
            match (num_before, den_before, num_after, den_after) {
                (Some(nb), Some(db), Some(na), Some(da)) => {
                    let before: Distance<Va> = Va::new(nb, db)?.into();
                    let after: Distance<Va> = Va::new(na, da)?.into();

                    Ok(DistanceVaSet { before, after })
                }

                (None, ..) | (_, _, None, _) => {
                    Err(VaBoundsError::NoPair(VaPair::Numerator).into())
                }

                (_, None, ..) | (_, _, _, None) => {
                    Err(VaBoundsError::NoPair(VaPair::Denominator).into())
                }
            }
        }

        let va = {
            let best_distance = distance_va_set(
                f.va_best_before_num,
                f.va_best_before_den,
                f.va_best_after_num,
                f.va_best_after_den,
            )?;

            // for now, deal only with best distance acuity
            OpVa {
                best_distance,
                best_near: None,
                raw_distance: None,
                raw_near: None,
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
