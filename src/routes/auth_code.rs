use super::BASE_AUTH_URL;
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::{
    extract::{cookie::Cookie, CookieJar},
    TypedHeader,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use std::{collections::HashMap, error::Error};
use thiserror::Error;

#[derive(Deserialize)]
pub struct Params {
    code: String,
}

#[derive(Debug, Error)]
pub enum AuthError {
    // todo: account for different error types
    #[error("an error")]
    Err,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::Err => "Authentication error".into_response(),
        }
    }
}

async fn handle_sign_in() -> Result<(), AuthError> {
    Ok(())
}

#[debug_handler]
pub async fn auth_code(
    Query(Params { code }): Query<Params>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AuthError> {
    #[derive(Debug, Deserialize)]
    struct AuthToken {
        auth_token: String,
        identity_id: String,
        provider_token: String,
        provider_refresh_token: Option<String>,
    };

    let base_auth_url = &*BASE_AUTH_URL;

    dbg!(&code);
    dbg!(&jar);

    let Some(verifier) = jar.get("edgedb-pkce-verifier") else {
        return Err(AuthError::Err);
    };
    dbg!(&verifier);

    let (_, verifier) = verifier.name_value_trimmed();
    dbg!(&verifier);

    let url = format!("{base_auth_url}/token?code={code}&verifier={verifier}");

    let auth_token = reqwest::get(url)
        .await
        // todo: impl error conversion to allow bubbling
        .unwrap()
        .json::<AuthToken>()
        .await
        .unwrap();
    dbg!(&auth_token);

    Ok((jar, Redirect::to("/add")))
    //
    //     let client = create_client()
    //         .await
    //         .expect("expected the DB client to be initialized")
    //         .with_globals_fn(|c| c.set("ext::auth::client_token", auth_token));
}
