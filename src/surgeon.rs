use crate::sia::{Sia, SiaBoundsError};
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

    pub fn inner(self) -> String {
        self.0
    }
}

/// An error type representing an invalid [`Surgeon`], typically as a result of invalid form input.
#[derive(Debug, Error)]
pub enum SurgeonError {
    #[error("invalid email")]
    Email(EmailValidationError),
    #[error("invalid SIA")]
    Sia(SiaBoundsError),
}

/// A surgeon's default [`Sia`] for right and left eyes
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct SurgeonSia {
    pub right: Sia,
    pub left: Sia,
}

/// A proto-[`Surgeon`] representing the surgeon's form input at sign-up.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormSurgeon {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub default_site: Option<String>,
    pub sia_right_power: f32,
    pub sia_right_axis: i32,
    pub sia_left_power: f32,
    pub sia_left_axis: i32,
}

#[cfg_attr(feature = "ssr", derive(Queryable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Site {
    name: String,
}

/// A unique surgeon
#[cfg_attr(feature = "ssr", derive(Queryable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Surgeon {
    /// A unique, valid email.
    pub email: String,
    pub terms: Option<DateTime<Utc>>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub default_site: Option<Site>,
    pub sia: Option<SurgeonSia>,
}

/// Return the current [`Surgeon`] from global server context. In practice, this function should
/// rarely be needed, as accessing a protected route will call
/// [`get_authorized_surgeon`](crate::auth::get_authorized_surgeon), which is then provided as
/// client-side context.
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
