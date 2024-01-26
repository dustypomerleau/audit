use serde::{Deserialize, Serialize};

/// A unique surgeon
// In the DB, the Surgeon type will have an SIA for right and left eyes, but we don't need that
// value here. After hitting the DB, either the value for `FlatCase::sia` will be `None` (in which
// case we use the surgeon's default value for that side), or it will be `Some()`, in which case
// that case-specific value will override the surgeon's defaults.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Surgeon {
    // probably best to validate this as unique and email form at both the form and database levels
    // - but pulling in the regex crate will probably make your wasm bundle huge
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub site: Option<String>,
}
