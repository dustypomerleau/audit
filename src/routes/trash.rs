use super::BASE_AUTH_URL;
use axum::{extract::Query, response::Redirect};
use axum_extra::{
    extract::{cookie::Cookie, CookieJar},
    TypedHeader,
};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};
use thiserror::Error;

#[derive(Deserialize)]
pub struct Params {
    code: String,
}

#[derive(Debug, Error)]
pub enum AuthError {
    // todo
    #[error("an error")]
    Err,
}

async fn handle_sign_in() -> Result<(), AuthError> {
    Ok(())
}

pub async fn trash(
    Query(Params { code }): Query<Params>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AuthError> {
    let base_auth_url = &*BASE_AUTH_URL;

    dbg!(&code);
    dbg!(&jar);

    let Some(verifier) = jar.get("edgedb-pkce-verifier") else {
        return Err(AuthError::Err);
    };

    Ok((jar, Redirect::to("/add")))
    //
    //     let client = create_client()
    //         .await
    //         .expect("expected the DB client to be initialized")
    //         .with_globals_fn(|c| c.set("ext::auth::client_token", auth_token));
}
