// todo: separate out all these configged functions into a module, and just config that
#[cfg(feature = "ssr")] use crate::state::AppState;
#[cfg(feature = "ssr")] use crate::state::StatePoisonedError;
#[cfg(feature = "ssr")] use gel_tokio::Client;
#[cfg(feature = "ssr")] use leptos::prelude::expect_context;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[cfg(feature = "ssr")] use thiserror::Error;

// todo: I'm not liking the way Gel and Query are different errors here - just a temporary fix
#[derive(Clone, Debug, Deserialize, Error, Serialize)]
#[cfg(feature = "ssr")]
pub enum DbError {
    #[error("Gel error: {0:?}")]
    Gel(String),
    #[error("The DB operation couldn't be completed due to poisoned state: {0:?}")]
    State(StatePoisonedError),
}

#[cfg(feature = "ssr")]
impl From<gel_tokio::Error> for DbError {
    fn from(err: gel_tokio::Error) -> Self {
        Self::Gel(format!("{err:?}"))
    }
}

#[cfg(feature = "ssr")]
impl From<StatePoisonedError> for DbError {
    fn from(err: StatePoisonedError) -> Self {
        Self::State(err)
    }
}

#[cfg(feature = "ssr")]
pub async fn db() -> Result<Client, DbError> {
    let client = expect_context::<AppState>()
        .db
        .get_cloned()
        .map_err(|err| DbError::State(StatePoisonedError(format!("{err:?}"))))?;

    Ok(client)
}

/// Handles the case where an inserted value is an [`Option`] containing a quoted string. If
/// the value is `None`, we only interpolate our `{}` with a single set of quotes, as this would be
/// unquoted in EdgeQL,  but if the value is `Some("string")`, we double the quotes, because the
/// value must remain quoted in EdgeQL after interpolation.
pub fn some_or_empty<T: AsRef<str> + Display>(value: Option<T>) -> String {
    value.map_or("{}".to_string(), |s| format!(r#""{s}""#))
}

/// Takes a value as float, and returns a truncated integer representation for storing in the
/// database.
pub fn to_centi(value: f32) -> i32 {
    // intentionally truncate, rather than rounding
    (value * 100.0) as i32
}

/// Takes an integer value from the database and returns a float representing the user-facing value.
pub fn to_hecto(value: i32) -> f32 {
    (value as f32) / 100.0
}

#[cfg(test)]
mod tests {}
