use crate::sia::Sia;
use garde::Validate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Email invalid: ({0:?})")]
pub struct EmailValidationError(garde::Report);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[garde(transparent)]
pub struct Email(#[garde(email)] String);

impl Email {
    pub fn new(email: &str) -> Result<Self, EmailValidationError> {
        let email = Self(email.to_string());

        match email.validate() {
            Ok(_) => Ok(email),
            Err(e) => Err(EmailValidationError(e)),
        }
    }
}

/// A surgeon's default [`Sia`] for right and left eyes
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SurgeonSia {
    pub right: Sia,
    pub left: Sia,
}

/// A unique surgeon
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Surgeon {
    /// A unique, valid email.
    pub email: Email,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub sites: Option<Vec<String>>,
    pub sia: Option<SurgeonSia>,
}
