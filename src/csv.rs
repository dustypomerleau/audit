use csv::Reader;
use serde::Deserialize;
use time::{Date, OffsetDateTime};

pub enum Side {
    Right,
    Left,
    Both,
}

pub enum Adverse {
    Rhexis,
    Pc,
    Zonule,
    Other,
}

pub struct Vision {
    num: i32,
    den: i32,
}

// these could be inside an enum, but the whole point is to make sure they aren't interchangeable
pub struct VaDistance(Vision);
pub struct VaNear(Vision);

pub trait Va {} // for common methods on acuity

pub struct Cyl {
    power: f32,
    axis: i32,
}

pub struct Refraction {
    sph: f32,
    cyl: Option<Cyl>,
}

pub struct Target {
    se: f32,
    cyl: Option<Cyl>, // confirm which plane the biometry is predicting
}

pub struct Incision {
    meridian: i32,
    sia: Option<i32>,
}

/// A unique surgeon
pub struct Surgeon {
    email: String, // probably best to validate this as unique and email form at both the form and database levels - but pulling in the regex crate will probably make your wasm bundle huge
    first_name: String,
    last_name: String,
    site: Option<String>,
}

/// A single surgical case
// consider moving this elsewhere, as csv::Case seems munted.
// for now, leave biometry parameters out - these can be added later with a working system
pub struct Case {
    surgeon: Surgeon,
    urn: String, // should be unique for the surgeon's reference, but not used for database uniqueness - recommend surgeons have a column to deanonymize
    side: Side,
    ref_before: Refraction,
    va_before: VaDistance,
    va_near_before: Option<VaNear>,
    target: Option<Target>,
    date: Date, // consider how this will be used: is there any scenario requiring a utc datetime? plan was to have an uploaded datetime, but there isn't any reason to keep this in the struct when you could get it from the DB created_at
    site: Option<String>,
    incision: Option<Incision>,
    iol: Option<String>,
    adverse: Option<Adverse>,
    ref_after: Refraction,
    va_after: VaDistance,
    va_near_after: Option<VaNear>,
}
