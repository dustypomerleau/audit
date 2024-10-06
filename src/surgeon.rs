use crate::sia::Sia;
use garde::Validate;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Email invalid: ({0:?})")]
pub struct EmailValidationError(garde::Report);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[garde(transparent)]
pub struct Email(#[garde(email)] String);

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

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
    pub default_site: Option<String>,
    pub sia: Option<SurgeonSia>,
}

#[cfg(test)]
#[cfg(feature = "ssr")]
mod tests {
    use super::*;

    fn sample_surgeon() -> Surgeon {
        Surgeon {
            email: Email::new("email@email.com").unwrap(),
            first_name: Some("john".to_string()),
            last_name: Some("smith".to_string()),
            default_site: Some("Royal Melbourne Hospital".to_string()),
            sia: Some(SurgeonSia {
                right: Sia {
                    power: 10,
                    axis: 100,
                },
                left: Sia {
                    power: 10,
                    axis: 100,
                },
            }),
        }
    }

    #[tokio::test]
    async fn inserts_surgeon() {}
}
