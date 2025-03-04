use crate::sia::Sia;
#[cfg(feature = "ssr")] use crate::state::AppState;
use chrono::{DateTime, Utc};
use garde::Validate;
#[cfg(feature = "ssr")] use gel_tokio::Queryable;
use leptos::{
    prelude::{ServerFnError, expect_context},
    server,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Email invalid: ({0:?})")]
pub struct EmailValidationError(garde::Report);

/// A [`garde`]-checked valid email [`String`]. We could set the type of [`Surgeon::email`]
/// to [`Email`], but this prevents deriving [`Queryable`], so instead we compromise, passing
/// through the email type as validation, but keeping the value on [`Surgeon`] as a [`String`].
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
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct SurgeonSia {
    pub right: Sia,
    pub left: Sia,
}

// todo: we need to add `terms`, do we need to account for all fields (meaning also include
// auth identity)? If so, we may need to move back to the idea of using DbSurgeon, as we don't
// want to pass the identity Uuid over the wire.
/// A unique surgeon
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct Surgeon {
    /// A unique, valid email. See notes on [`Email`] to understand why we use a plain [`String`]
    /// value here.
    pub email: String,
    pub terms: Option<DateTime<Utc>>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub default_site: Option<String>,
    pub sia: Option<SurgeonSia>,
}

/// Return the current [`Surgeon`] from global server context.
#[server]
pub async fn get_current_surgeon() -> Result<Option<Surgeon>, ServerFnError> {
    let surgeon = expect_context::<AppState>().surgeon.get_cloned()?;
    Ok(surgeon)
}

/// Set the value of the current [`Surgeon`] in global server context. using `Option<Surgeon>` as
/// the input parameter allows clearing the value by setting [`None`].
#[server]
pub async fn set_current_surgeon(surgeon: Option<Surgeon>) -> Result<(), ServerFnError> {
    expect_context::<AppState>().surgeon.set(surgeon)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // use chrono::TimeZone;

    fn sample_surgeon() -> Surgeon {
        Surgeon {
            email: Email::new("email@email.com").unwrap().0,
            // terms: Some(Utc.with_ymd_and_hms(2024, 5, 15, 20, 30, 40).unwrap()),
            terms: None,
            first_name: Some("sample_first_name".to_string()),
            last_name: Some("sample_last_name".to_string()),
            default_site: Some("sample_default_site".to_string()),

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
}
