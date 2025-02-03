use super::BASE_AUTH_URL;
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::{
    extract::{
        cookie::{Cookie, Expiration, SameSite},
        CookieJar,
    },
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
    #[error("unable to deserialize the response as JSON: {0:?}")]
    Json(String),
    #[error("did not receive a response from the token request: {0:?}")]
    Request(String),
    #[error("unable to get the PKCE verifier from the cookie jar")]
    Verifier,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::Json(err) => {
                format!("Error: unable to deserialize the response as JSON: {err:?}")
                    .into_response()
            }
            Self::Request(err) => {
                format!("Error: did not receive a response from the token request: {err:?}")
                    .into_response()
            }
            Self::Verifier => {
                "Error: unable to get the PKCE verifier from the cookie jar".into_response()
            }
        }
    }
}

#[debug_handler]
pub async fn auth_code(
    Query(Params { code }): Query<Params>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AuthError> {
    dbg!(&code);
    let base_auth_url = &*BASE_AUTH_URL;

    let verifier = if let Some(verifier) = jar.get("edgedb-pkce-verifier") {
        verifier.value_trimmed()
    } else {
        return Err(AuthError::Verifier);
    };
    dbg!(&verifier);

    let url = format!("{base_auth_url}/token?code={code}&verifier={verifier}");

    let token = reqwest::get(url)
        .await
        .map_err(|err| AuthError::Request(format!("{err:?}")))?
        .text()
        .await
        .map_err(|err| AuthError::Json(format!("{err:?}")))?;
    dbg!(&token);

    let cookie = Cookie::build(("edgedb-auth-token", token))
        .expires(None)
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    let jar = jar.add(cookie);
    dbg!(&jar);

    Ok((jar, Redirect::to("/add")))
    //
    //     let client = create_client()
    //         .await
    //         .expect("expected the DB client to be initialized")
    //         .with_globals_fn(|c| c.set("ext::auth::client_token", auth_token));
}
