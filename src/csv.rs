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
    urn: String, // should be unique for the surgeon's reference, but not used for database uniqueness - recommend surgeons have a column to deanonymize
    side: Side,
    surgeon: Surgeon,
    date: Date, // consider how this will be used: is there any scenario requiring a utc datetime? plan was to have an uploaded datetime, but there isn't any reason to keep this in the struct when you could get it from the DB created_at
    site: String,
    iol: Option<String>,
    incision: Option<i32>,
    adverse: Option<Adverse>,
    sph_before: i32, // multiply sph x 100
    cyl_before: Option<i32>,
    axis_before: Option<i32>,
    sph: i32,
    cyl: i32,
    va_num_before: i32,
    va_num: i32,
    va_den_before: i32,
    va_den: i32,
    va_near_num_before: Option<i32>,
    va_near_num: Option<i32>,
    va_near_den_before: Option<i32>,
    va_near_den: Option<i32>,
}
