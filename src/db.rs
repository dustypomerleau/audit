#[cfg(feature = "ssr")] use crate::state::AppState;
use crate::state::StatePoisonedError;
#[cfg(feature = "ssr")] use gel_tokio::Client;
use leptos::prelude::expect_context;
use thiserror::Error;

#[derive(Debug, Error)]
#[cfg(feature = "ssr")]
pub enum DbError {
    #[error("Gel error: {0:?}")]
    Gel(gel_tokio::Error),
    #[error("The DB operation couldn't be completed due to poisoned state: {0:?}")]
    State(StatePoisonedError),
}

#[cfg(feature = "ssr")]
impl From<gel_tokio::Error> for DbError {
    fn from(err: gel_tokio::Error) -> Self {
        Self::Gel(err)
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

/// Handles the case where an inserted value is an [`Option`] containing a quoted [`String`]. If
/// the value is `None`, we only interpolate our `{}` with a single set of quotes, as this would be
/// unquoted in EdgeQL,  but if the value is `Some("string")`, we double the quotes, because the
/// value must remain quoted in EdgeQL after interpolation.
pub fn some_or_empty(value: Option<String>) -> String {
    value.map_or("{}".to_string(), |s| format!(r#""{s}""#))
}

/// Takes a value in whole diopters (D) and returns an integer value of centidiopters for storing
/// in the database.
pub fn to_cd(diopters: f32) -> i32 {
    (diopters * 100.0) as i32
}

/// Takes an integer value of centidiopters from the database and returns a float representing the
/// value in diopters (D).
pub fn to_d(centidiopters: i32) -> f32 {
    (centidiopters as f32) / 100.0
}

#[cfg(test)]
mod tests {}
