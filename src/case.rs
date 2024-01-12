use crate::{
    iol::{Iol, IolBoundsError},
    refraction::{OpRefraction, RefBoundsError},
    sia::SiaBoundsError,
    surgeon::Surgeon,
    target::{Target, TargetBoundsError},
    va::{OpVa, VaBoundsError},
};
use thiserror::Error;
use time::Date;

/// A representation of the required fields for each [`Case`], for use in [`CaseError::MissingField`].
#[derive(Debug, PartialEq)]
enum Required {
    Surgeon,
    Urn,
    Side,
    Date,
    Va,
    Refraction,
}

/// The error type for a [`Case`] with missing mandatory fields or out of bounds values.
#[derive(Debug, Error)]
enum CaseError {
    #[error("out of bounds value on field `iol` of `Case`: {0:?}")]
    Iol(IolBoundsError),
    #[error("{0:?} is a required field on `Case`, but wasn't supplied")]
    MissingField(Required),
    #[error("out of bounds value on field `sia` of `Case`: {0:?}")]
    Sia(SiaBoundsError),
    #[error("out of bounds value on field `target` of `Case`: {0:?}")]
    Target(TargetBoundsError),
    #[error("out of bounds value on field `refraction` of `Case`: {0:?}")]
    Refraction(RefBoundsError),
    #[error("out of bounds value on field `va` of `Case`: {0:?}")]
    Va(VaBoundsError),
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
