use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::{
    TypedHeader,
    extract::{
        CookieJar,
        cookie::{Cookie, Expiration, SameSite},
    },
};
use axum_macros::debug_handler;
use base64ct::{Base64UrlUnpadded, Encoding};
use rand::{Rng, random, rng};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, env, error::Error, sync::LazyLock};
use thiserror::Error;

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

/// Holds the verifier/challenge pair that is used during site authentication. The challenge is
/// passed via the URL, and the verifier is stored in an HTTP-only cookie for access after the
/// authentication flow is completed. Successful authentication returns a `code` param in the URL.
/// Supplying the `code/verifier` pair in a `GET` request to the EdgeDB Auth token server will
/// return an auth token in JSON format, and storing this JSON as a cookie will allow you to check
/// authentication for access to protected routes.
#[derive(Debug)]
pub struct Pkce {
    verifier: String,
    challenge: String,
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
pub async fn handle_sign_in(jar: CookieJar) -> Result<(CookieJar, Redirect), AuthError> {
    let Pkce {
        challenge,
        verifier,
    } = generate_pkce();

    let base_auth_url = &*BASE_AUTH_URL;

    let cookie = Cookie::build(("edgedb-pkce-verifier", verifier))
        .expires(None)
        .http_only(true)
        .path("/")
        // `Lax` is required to send the cookie to the auth URL
        .same_site(SameSite::Lax)
        .secure(true)
        .build();

    let jar = jar.add(cookie);
    let url = format!("{base_auth_url}/ui/signin?challenge={challenge}");

    Ok((jar, Redirect::to(&url)))
}

/// A PKCE authentication code returned by the OAuth provider via URL query string.
#[derive(Debug, Deserialize)]
pub struct Params {
    code: String,
}

/// Step 2 of the auth flow: After returning from successful authentication with the OAuth
/// provider, use the code provided in the URL query params, along with the verifier you previously
/// stored in a cookie, to request a Gel auth token from the Gel Auth JSON API. Storing the auth
/// token as a cookie allows you to confirm the logged-in surgeon when accessing protected routes.
#[debug_handler]
pub async fn handle_pkce_code(
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

    // todo: create a DB client with the auth token as a global, and place it in a reactive store
    //
    //     let client = create_client()
    //         .await
    //         .expect("expected the DB client to be initialized")
    //         .with_globals_fn(|c| c.set("ext::auth::client_token", auth_token));
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use leptos::logging::log;

    #[test]
    #[cfg(feature = "ssr")]
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
