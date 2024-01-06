use crate::{
    incision::Incision,
    refraction::{OpRefraction, RefBoundsError},
    surgeon::Surgeon,
    target::{Target, TargetBoundsError},
    va::{OpVa, VaBoundsError},
};
use thiserror::Error;
use time::Date;

#[derive(Debug, PartialEq)]
enum Required {
    Surgeon,
    Urn,
    Side,
    Date,
    Va,
    Refraction,
}

#[derive(Debug, Error)]
enum CaseError {
    #[error("{0:?} is a required field on `Case`, but wasn't supplied")]
    MissingField(Required),
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

/// A single surgical case
// for now, leave biometry parameters out - these can be added later with a working system
#[derive(Debug, PartialEq)]
pub struct Case {
    surgeon: Surgeon,
    urn: String, // used for the surgeon's reference, not database uniqueness - recommend surgeons have a column to deanonymize
    side: Side,
    target: Option<Target>,
    date: Date, // consider how this will be used: is there any scenario requiring a utc datetime? plan was to have an uploaded datetime, but there isn't any reason to keep this in the struct when you could get it from the DB created_at
    site: Option<String>,
    incision: Option<Incision>,
    iol: Option<String>,
    adverse: Option<Adverse>,
    va: OpVa,
    refraction: OpRefraction,
}
