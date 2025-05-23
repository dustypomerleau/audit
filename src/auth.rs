use crate::state::{AppState, StatePoisonedError};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use axum_macros::debug_handler;
use base64ct::{Base64UrlUnpadded, Encoding};
use gel_tokio::create_client;
use leptos::prelude::ServerFnError;
use leptos_axum::extract;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{env, sync::LazyLock};
use thiserror::Error;
use uuid::Uuid;

// note: new API for dotenvy will arrive in v16 release
pub static BASE_AUTH_URL: LazyLock<String> = LazyLock::new(|| {
    env::var("BASE_AUTH_URL").expect("expected BASE_AUTH_URL environment variable to be present")
});

/// Possible failure modes during the code exchange of a PKCE flow.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("unable to deserialize the response as JSON: {0:?}")]
    Json(String),
    #[error("did not receive a response from the token request: {0:?}")]
    Request(String),
    #[error("unable to read or write the intended state: {0:?}")]
    State(StatePoisonedError),
    #[error("the auth token cookie is not present")]
    Token,
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
            Self::State(err) => {
                format!("Error: unable to read or write the intended state: {err:?}")
                    .into_response()
            }
            Self::Token => "Error: the auth token cookie is not present".into_response(),
            Self::Verifier => {
                "Error: unable to get the PKCE verifier from the cookie jar".into_response()
            }
        }
    }
}

impl From<StatePoisonedError> for AuthError {
    fn from(err: StatePoisonedError) -> Self {
        Self::State(err)
    }
}

/// Holds the verifier/challenge pair that is used during site authentication. The challenge is
/// passed via the URL, and the verifier is stored in an HTTP-only cookie for access after the
/// authentication flow is completed. Successful authentication returns a `code` param in the URL.
/// Supplying the `code/verifier` pair in a `GET` request to the Gel Auth token server will
/// return an auth token in JSON format, and storing this JSON as a cookie will allow you to check
/// authentication for access to protected routes.
#[derive(Debug)]
pub struct Pkce {
    verifier: String,
    challenge: String,
}

/// A PKCE authentication code returned by the OAuth provider via URL query string.
#[derive(Debug, Deserialize)]
pub struct PkceParams {
    code: String,
}

/// A deserialization target for the JSON "gel-auth-token" cookie. Used primarily for holding
/// the current surgeon's identity ID.
#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    /// A Base64 URL-encoded JWT, consisting of `.`-separated header, payload, and signature.
    pub auth_token: String,
    pub identity_id: Uuid,
    pub provider_token: String,
    pub provider_refresh_token: Option<String>,
    pub provider_id_token: String,
}

/// Generate a `verifier/challenge` pair for use in the authentication flow (see [`Pkce`] for
/// details).
pub fn generate_pkce() -> Pkce {
    // 1. generate 32 random bytes and URL-encode it:
    let input: [u8; 32] = rng().random();
    let verifier = Base64UrlUnpadded::encode_string(&input);
    // 2. SHA256 hash the result, then URL-encode again:
    let hash = Sha256::new().chain_update(&verifier).finalize();
    let challenge = Base64UrlUnpadded::encode_string(&hash);

    Pkce {
        challenge,
        verifier,
    }
}

/// Step 1 of the auth flow: Generate a [`Pkce`] challenge/verifier pair, populate the URL params
/// with the challenge, and set a cookie with the verifier, redirecting to the OAuth
/// provider.
#[debug_handler]
pub async fn handle_sign_in(jar: CookieJar) -> (CookieJar, Redirect) {
    let Pkce {
        challenge,
        verifier,
    } = generate_pkce();

    let base_auth_url = &*BASE_AUTH_URL;

    let cookie = Cookie::build(("gel-pkce-verifier", verifier))
        .expires(None)
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax) // required to send the cookie to the auth URL
        .secure(true)
        .build();

    let jar = jar.add(cookie);
    let url = format!("{base_auth_url}/ui/signin?challenge={challenge}");

    (jar, Redirect::to(&url))
}

/// Step 2 of the auth flow: After returning from successful authentication with the OAuth
/// provider, use the code provided in the URL query params, along with the verifier you previously
/// stored in a cookie, to request an auth token from the Gel Auth JSON API. Storing the auth
/// token as a cookie allows you to confirm the logged-in surgeon when accessing protected routes.
#[debug_handler]
pub async fn handle_pkce_code(
    State(AppState { db, .. }): State<AppState>,
    Query(PkceParams { code }): Query<PkceParams>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AuthError> {
    let base_auth_url = &*BASE_AUTH_URL;

    let verifier = if let Some(verifier) = jar.get("gel-pkce-verifier") {
        verifier.value()
    } else {
        return Err(AuthError::Verifier);
    };

    let url = format!("{base_auth_url}/token?code={code}&verifier={verifier}");

    let response = reqwest::get(&url)
        .await
        .map_err(|err| AuthError::Request(format!("{err:?}")))?
        .text()
        .await
        .map_err(|err| AuthError::Json(format!("{err:?}")))?;

    let AuthResponse { auth_token, .. } =
        serde_json::from_str(&response).map_err(|err| AuthError::Json(format!("{err:?}")))?;

    let db_with_globals = create_client()
        .await
        .expect("DB client to be initialized before adding globals")
        .with_globals_fn(|client| client.set("ext::auth::client_token", &auth_token));

    db_with_globals
        .ensure_connected()
        .await
        .expect("DB client with globals to connect");

    db.set(db_with_globals)
        .map_err(|err| StatePoisonedError(format!("{err:?}")))?;

    let cookie = Cookie::build(("gel-auth-token", auth_token))
        .expires(None)
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .secure(true)
        .build();

    // Add the new auth token cookie, and remove the verifier, which is no longer needed.
    let jar = jar.add(cookie).remove(Cookie::from("gel-pkce-verifier"));
    Ok((jar, Redirect::to("/gateway")))
}

/// This function is called when the current surgeon logs out, removing the auth token from the
/// database client, deleting the auth token cookie, and removing the current
/// [`Surgeon`](crate::surgeon::Surgeon) from global state.
#[debug_handler]
pub async fn handle_kill_session(
    State(AppState { db, surgeon, .. }): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AuthError> {
    // A dummy DB client, so that we can replace the `gel_tokio::Client` that contains
    // surgeon-specific globals
    let client = gel_tokio::create_client()
        .await
        .expect("DB client to be created");

    db.set(client)
        .map_err(|err| StatePoisonedError(format!("{err:?}")))?;

    surgeon
        .set(None)
        .map_err(|err| StatePoisonedError(format!("{err:?}")))?;

    let jar = jar.remove(Cookie::from("gel-auth-token"));
    Ok((jar, Redirect::to("/")))
}

pub async fn get_jwt_cookie() -> Result<String, ServerFnError> {
    let auth_token = extract::<CookieJar>()
        .await?
        .get("gel-auth-token")
        .unwrap_or(&Cookie::new(
            "gel-auth-token",
            "the unwrap on `gel-auth-token` failed because it was `None`",
        ))
        .value()
        .to_string();
    dbg!(&auth_token);

    Ok(auth_token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use leptos::logging::log;

    #[cfg(feature = "ssr")]
    #[test]
    fn generates_pkce() {
        let pkce = generate_pkce();

        log!(
            "{pkce:?}, verifier length: {:?}, challenge length: {:?}",
            pkce.verifier.len(),
            pkce.challenge.len()
        );

        assert!(pkce.verifier.len() == pkce.challenge.len() && pkce.verifier.len() == 43);
    }

    #[test]
    fn test_env_vars() {
        dotenv().ok();
        log!("base auth URL: {}", &*BASE_AUTH_URL,);
    }
}
