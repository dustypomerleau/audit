use crate::sia::Sia;
use serde::{Deserialize, Serialize};

/// A surgeon's default [`Sia`] for right and left eyes
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SurgeonSia {
    pub right: Sia,
    pub left: Sia,
}

/// A unique surgeon
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Surgeon {
    // todo: pulling in the regex crate will increase wasm bundle size
    // consider how best to validate
    /// A unique, valid email.
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub sites: Option<Vec<String>>,
    pub sia: Option<SurgeonSia>,
}
