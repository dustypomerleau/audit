use std::env;
use std::sync::LazyLock;

use axum::extract::Query;
use axum::extract::State;
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::cookie::SameSite;
use axum_macros::debug_handler;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use leptos_axum::extract;
use rand::Rng;
use rand::rng;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use uuid::Uuid;

use crate::error::AppError;
use crate::state::AppState;

/// Environment variables needed during the OAuth PKCE flow.
struct AuthVars {
    base_auth_url: String,
    cookie_secure: bool,
}

static AUTH_VARS: LazyLock<AuthVars> = LazyLock::new(|| {
    let base_auth_url = if cfg!(test) {
        env::var("TEST_AUTH_URL")
            .expect("expected TEST_AUTH_URL environment variable to be present")
    } else {
        env::var("BASE_AUTH_URL")
            .expect("expected BASE_AUTH_URL environment variable to be present")
    };

    // Unless en environment variable specifically sets `COOKIE_SECURE` to `false` (for dev
    // environments on localhost), the value of `AUTH_VARS.cookie_secure` will be `true`.
    let insecure = matches!(
        env::var("COOKIE_SECURE").unwrap_or_default().as_str(),
        "false"
    );

    AuthVars {
        base_auth_url,
        cookie_secure: !insecure,
    }
});

/// The verifier/challenge pair that is used during site authentication. The challenge is
/// passed via the URL, and the verifier is stored in an HTTP-only cookie for access after the
/// authentication flow is completed. Successful authentication returns a `code` param in the URL.
/// Supplying the `code/verifier` pair in a `GET` request to the Gel Auth token server will
/// return an auth token in JSON format, and storing this JSON as a cookie will allow you to check
/// authentication for access to protected routes.
#[derive(Debug)]
struct Pkce {
    verifier: String,
    challenge: String,
}

/// A PKCE authentication code returned by the OAuth provider via URL query string.
#[derive(Debug, Deserialize)]
pub struct PkceParams {
    code: String,
}

/// A deserialization target for the Gel Auth JSON API.
#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
struct AuthResponse {
    /// A Base64 URL-encoded JWT, consisting of dot-separated header, payload, and signature.
    pub auth_token: String,
    pub identity_id: Uuid,
    pub provider_token: String,
    pub provider_refresh_token: Option<String>,
    pub provider_id_token: String,
}

/// Generate a `verifier/challenge` pair for use in the authentication flow (see [`Pkce`] for
/// details).
fn generate_pkce() -> Pkce {
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
pub async fn handle_sign_in(mut jar: CookieJar) -> (CookieJar, Redirect) {
    let Pkce {
        challenge,
        verifier,
    } = generate_pkce();

    let base_auth_url = &*AUTH_VARS.base_auth_url;

    let cookie = Cookie::build(("gel-pkce-verifier", verifier))
        .expires(None)
        .http_only(true)
        .path("/")
        // Lax is required when arriving from a Gel Auth origin
        .same_site(SameSite::Lax)
        .secure(AUTH_VARS.cookie_secure)
        .build();

    jar = jar.add(cookie);
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
    mut jar: CookieJar,
) -> Result<(CookieJar, Redirect), AppError> {
    let base_auth_url = &*AUTH_VARS.base_auth_url;

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
        // Lax is required when arriving from a Gel Auth origin
        .same_site(SameSite::Lax)
        .secure(AUTH_VARS.cookie_secure)
        .build();

    jar = jar.add(cookie);

    Ok((jar, Redirect::to("/gateway")))
}

/// This function is called when the current surgeon logs out, removing the auth token from the
/// database client, deleting the auth token cookie, and removing the current
/// [`Surgeon`](crate::surgeon::Surgeon) from global state.
#[debug_handler]
pub async fn handle_kill_session(
    State(AppState { db, surgeon, .. }): State<AppState>,
    mut jar: CookieJar,
) -> Result<(CookieJar, Redirect), AppError> {
    // A dummy DB client, so that we can replace the `gel_tokio::Client` that contains
    // surgeon-specific globals
    let client = gel_tokio::create_client()
        .await
        .expect("expected DB client to be created");

    db.set(client)?;
    surgeon.set(None)?;
    jar = jar.remove(Cookie::from("gel-auth-token"));

    Ok((jar, Redirect::to("/")))
}

/// Get the current surgeon's JWT from the `gel-auth-token` cookie.
pub async fn get_jwt_cookie() -> Result<Option<String>, AppError> {
    let auth_token = extract::<CookieJar>()
        .await?
        .get("gel-auth-token")
        .map(|cookie| cookie.value().to_string());

    Ok(auth_token)
}

#[cfg(test)]
mod tests {
    use leptos::logging::log;

    use super::*;

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
}
