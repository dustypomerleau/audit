use csv::Reader;
use serde::Deserialize;

struct Case {
    ur: String, // must be unique per surgeon, but not used for database uniqueness
    date: Date,
}
