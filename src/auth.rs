use crate::{error::AppError, state::AppState};
use axum::{
    extract::{Query, State},
    response::Redirect,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use axum_macros::debug_handler;
use base64ct::{Base64UrlUnpadded, Encoding};
use leptos_axum::extract;
use rand::{Rng, rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{env, sync::LazyLock};
use uuid::Uuid;

// note: new API for dotenvy will arrive in v16 release
pub static BASE_AUTH_URL: LazyLock<String> = LazyLock::new(|| {
    env::var("BASE_AUTH_URL").expect("expected BASE_AUTH_URL environment variable to be present")
});

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
    Query(PkceParams { code }): Query<PkceParams>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AppError> {
    let base_auth_url = &*BASE_AUTH_URL;

    let verifier = if let Some(verifier) = jar.get("gel-pkce-verifier") {
        verifier.value()
    } else {
        return Err(AppError::Auth(
            "the verifier cookie was not found in the cookie jar".to_string(),
        ));
    };

    let url = format!("{base_auth_url}/token?code={code}&verifier={verifier}");
    let response = reqwest::get(&url).await?.text().await?;
    let AuthResponse { auth_token, .. } = serde_json::from_str(&response)?;

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
) -> Result<(CookieJar, Redirect), AppError> {
    // A dummy DB client, so that we can replace the `gel_tokio::Client` that contains
    // surgeon-specific globals
    let client = gel_tokio::create_client()
        .await
        .expect("DB client to be created");

    db.set(client)?;
    surgeon.set(None)?;
    let jar = jar.remove(Cookie::from("gel-auth-token"));

    Ok((jar, Redirect::to("/")))
}

// Plan:
// 1. do what you need to do to get AuthError working as ServerFnErrorErr or whatev
// 2. instead of unwrapping, map the error when the token isn't present to AuthError::Token
// 3. visit places where you call this function, and if there's no cookie, redirect to signin
// 4. if there is a cookie, then check the db global, and if it doesn't match, create a new Client
//    and write it to state
pub async fn get_jwt_cookie() -> Result<Option<String>, AppError> {
    let auth_token = extract::<CookieJar>()
        .await?
        .get("gel-auth-token")
        .map(|cookie| cookie.value().to_string());

    Ok(auth_token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use leptos::logging::log;

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
