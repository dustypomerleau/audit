#[cfg(feature = "ssr")] use crate::state::AppState;
use crate::{
    bounded::BoundsError,
    model::{Formula, Iol, Main, Sia},
};
use chrono::{DateTime, Utc};
use garde::Validate;
use leptos::prelude::{ServerFnError, expect_context, server};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Email invalid: ({0:?})")]
pub struct EmailValidationError(garde::Report);

/// A [`garde`]-checked valid email [`String`].
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Validate)]
#[garde(transparent)]
pub struct Email(#[garde(email)] String);

// Implementing Display allows directly including an Email in a format String.
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
    Sia(BoundsError),
}

/// A surgeon's default [`Sia`] for right and left eyes
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    pub default_iol: Option<String>,
    pub default_formula: Option<String>,
    pub custom_constant: Option<String>,
    pub main: f32,
    pub sia_power: f32,
    pub sia_right_axis: u32,
    pub sia_left_axis: u32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Site {
    pub name: String,
}

/// A unique surgeon
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Surgeon {
    /// A unique, valid email.
    pub email: Email,
    pub terms: Option<DateTime<Utc>>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub defaults: Option<SurgeonDefaults>,
    pub sia: SurgeonSia,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SurgeonDefaults {
    site: Option<Site>,
    iol: Option<Iol>,
    formula: Option<Formula>,
    custom_constant: bool,
    main: Main,
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
mod tests {}
