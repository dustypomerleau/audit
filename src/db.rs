use crate::{error::AppError, state::AppState};
use gel_tokio::Client;
use leptos::prelude::use_context;
use std::fmt::Display;

pub async fn db() -> Result<Client, AppError> {
    let client = if let Some(state) = use_context::<AppState>() {
        state.db.get_cloned()?
    } else {
        return Err(AppError::Db(
            "AppState is not present in context".to_string(),
        ));
    };

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
pub mod tests {
    use dotenvy::dotenv;
    use gel_tokio::Client;
    use std::{env, sync::LazyLock};

    pub async fn test_db() -> Client {
        // note: new API for dotenvy will arrive in v16 release
        pub static TEST_JWT: LazyLock<String> = LazyLock::new(|| {
            dotenv().ok();
            env::var("TEST_JWT").expect("expected TEST_JWT environment variable to be present")
        });

        let jwt = &*TEST_JWT;

        gel_tokio::create_client()
            .await
            .unwrap()
            .with_globals_fn(|client| client.set("ext::auth::client_token", jwt))
    }
}
