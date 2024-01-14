use crate::{
    flatcase::FlatCase,
    iol::{Iol, IolBoundsError},
    refraction::{OpRefraction, RefBoundsError},
    sca::{Sca, ScaBoundsError},
    sia::{Sia, SiaBoundsError},
    surgeon::Surgeon,
    target::{Target, TargetBoundsError},
    va::{OpVa, VaBoundsError},
};
use std::{error::Error, fmt::Debug};
use thiserror::Error;
use time::Date;

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
enum CaseError<T: Error + Debug> {
    #[error("out of bounds value on field {0:?} of `Case`")]
    Bounds(T),
    #[error("{0:?} is a required field on `Case`, but wasn't supplied")]
    MissingField(Required),
}

impl<T: Debug + Error> From<T> for CaseError {
    fn from(err: T) -> Self { CaseError::Bounds(err) }
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
            Err(CaseError::MissingField(Required::Email))
        };
    }
}
