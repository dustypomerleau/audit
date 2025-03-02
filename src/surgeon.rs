use crate::sia::Sia;
use garde::Validate;
#[cfg(feature = "ssr")] use gel_tokio::Queryable;
use leptos::prelude::{ServerFnError, expect_context, server};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};
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

/// A unique surgeon
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(Queryable))]
pub struct Surgeon {
    /// A unique, valid email.
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub default_site: Option<String>,
    pub sia: Option<SurgeonSia>,
}

#[server]
pub async fn get_current_surgeon() -> Result<Option<Surgeon>, ServerFnError> {
    let surgeon = expect_context::<Arc<RwLock<Option<Surgeon>>>>().get_cloned()?;
    Ok(surgeon)
}

// using `Option<Surgeon>` as the arg allows clearing the value by setting `None`
#[server]
pub async fn set_current_surgeon(surgeon: Option<Surgeon>) -> Result<(), ServerFnError> {
    expect_context::<Arc<RwLock<Option<Surgeon>>>>().set(surgeon)?;
    Ok(())
}

#[cfg(test)]
#[cfg(feature = "ssr")]
mod tests {
    use super::*;

    fn sample_surgeon() -> Surgeon {
        Surgeon {
            email: Email::new("email@email.com").unwrap().0,
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
