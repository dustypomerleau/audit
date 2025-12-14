use std::fmt::Display;

use leptos::prelude::use_context;
use sqlx::Pool;
use sqlx::Postgres;

use crate::error::AppError;
use crate::state::AppState;

pub async fn db() -> Result<Pool<Postgres>, AppError> {
    let pool = if let Some(state) = use_context::<AppState>() {
        // The gel_tokio::Client is cheap to clone because its inner fields are Arc<T>
        state.db.clone()
    } else {
        return Err(AppError::Db(
            "AppState is not present in context".to_string(),
        ));
    };

    Ok(pool)
}

/// Handles the case where an inserted value is an [`Option`] containing a quoted string. If
/// the value is `None`, we only interpolate our `{}` with a single set of quotes, as this would be
/// unquoted in EdgeQL,  but if the value is `Some("string")`, we double the quotes, because the
/// value must remain quoted in EdgeQL after interpolation.
pub fn some_or_empty<T: AsRef<str> + Display>(value: Option<T>) -> String {
    value.map_or("{}".to_string(), |s| format!(r#""{s}""#))
}

/// Takes a value as float, and returns a truncated integer representation for storing in the
/// database. We use f32, because this is the default type of float user input coming from the
/// browser.
pub fn to_centi(value: f32) -> i32 {
    // intentionally truncate, rather than rounding
    (value * 100.0) as i32
}

/// Takes an integer value from the database and returns a float representing the user-facing value.
/// We use f64 because this is the default type used in Plotly.
pub fn to_hecto<T: Into<f64>>(value: T) -> f64 { value.into() / 100.0 }

#[cfg(test)]
mod tests {}
