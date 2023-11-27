use csv::Reader;
use serde::Deserialize;
use time::Date;

/// A single surgical case
// consider moving this elsewhere, as csv::Case seems munted.
pub struct Case {
    ur: String, // should be unique for the surgeon's reference, but not used for database uniqueness
    date: Date, // consider how this will be used: is there any scenario requiring a utc datetime?
}
