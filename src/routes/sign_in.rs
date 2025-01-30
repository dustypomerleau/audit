#[cfg(feature = "ssr")] use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::format;
#[cfg(feature = "ssr")] use http::{header, HeaderValue};
use leptos::{
    logging::log,
    prelude::{
        component, expect_context, server, view, Await, ElementChild, IntoView, OnAttribute, Read,
        Result, ServerFnError, StorageAccess, StyleAttribute, Suspend, Suspense,
    },
    server::{LocalResource, OnceResource, Resource},
    task::spawn_local,
};
#[cfg(feature = "ssr")] use leptos_axum::{redirect, ResponseOptions};
use leptos_router::hooks::{query_signal, use_navigate, use_params_map, use_query_map};
#[cfg(feature = "ssr")] use rand::{thread_rng, Rng};
use serde::Deserialize;
/* todo re: wasm https://github.com/rust-random/rand/issues/991 */
use sha2::{Digest, Sha256};
use std::{env, sync::LazyLock};
use thiserror::Error;

// note: new API for dotenvy will arrive in v16 release
pub static BASE_AUTH_URL: LazyLock<String> = LazyLock::new(|| {
    env::var("BASE_AUTH_URL").expect("expected BASE_AUTH_URL environment variable to be present")
});

pub static SERVER_PORT: LazyLock<String> = LazyLock::new(|| {
    env::var("SERVER_PORT").expect("expected SERVER_PORT environment variable to be present")
});

// #[derive(Debug, Deserialize, Error)]
// pub enum AuthError {
//     #[error("Invalid header value: {0:?}")]
//     InvalidHeader(http::header::InvalidHeaderValue),
//
//     #[error("Reqwest error: {0:?}")]
//     Reqwest(reqwest::Error),
// }
//
// impl From<http::header::InvalidHeaderValue> for AuthError {
//     fn from(err: http::header::InvalidHeaderValue) -> Self {
//         Self::InvalidHeader(err)
//     }
// }
//
// impl From<reqwest::Error> for AuthError {
//     fn from(err: reqwest::Error) -> Self {
//         Self::Reqwest(err)
//     }
// }

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
#[cfg(feature = "ssr")]
pub fn generate_pkce() -> Pkce {
    // 1. generate 32 random bytes and URL-encode it:
    let input: [u8; 32] = thread_rng().r#gen();
    let verifier = Base64UrlUnpadded::encode_string(&input);
    // 2. SHA256 hash the result, then URL-encode again:
    let hash = Sha256::new().chain_update(&verifier).finalize();
    let challenge = Base64UrlUnpadded::encode_string(&hash);

    Pkce {
        verifier,
        challenge,
    }
}

/// Generate a [`Pkce`] challenge/verifier pair, populate the URL params with the
/// challenge, and set a cookie with the verifier, redirecting to the OAuth
/// provider.
#[server]
async fn handle_sign_in() -> Result<(), ServerFnError> {
    let Pkce {
        verifier,
        challenge,
    } = generate_pkce();

    let response = expect_context::<ResponseOptions>();

    response.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            // "edgedb-pkce-verifier={verifier}; HttpOnly; Path=/; SameSite=Strict; Secure;"
            // `Secure` is preventing the cookie from being sent over plain http during dev
            // you need emulate https
            "edgedb-pkce-verifier={verifier}; HttpOnly; Path=/;"
        ))?,
    );

    dbg!(&response);

    redirect(&format!(
        "{}/ui/signin?challenge={challenge}",
        &*BASE_AUTH_URL
    ));

    Ok(())
}

/// Google OAuth redirects to the URL set in `edgedb ui` > Auth > Providers > `redirect_to`. That
/// URL invokes this callback function, which needs to:
///
/// 1. retrieve the value of `code` from the URL query string
/// 1. retrieve the value of `verifier` from the `edgedb-pkce-verifier` cookie
/// t. make a GET request to `{BASE_AUTH_URL}/token?code={code}&verifier={verifier}`
/// 1. save the returned JSON in the variable `auth_token`
/// 1. set the `edgedb-auth-token` HTTP-only cookie to the value of `auth_token`
/// 1. redirect to the surgeon's dashboard
#[server]
pub async fn handle_callback() -> Result<(), ServerFnError> {
    #[derive(Debug, Deserialize)]
    struct JsonWrapper(String);

    let base_auth_url = &*BASE_AUTH_URL;

    let code = use_query_map().read().get("code").expect(
        "expected the auth code to be present in the URL query string after successful OAuth flow",
    );

    let response = expect_context::<ResponseOptions>();

    dbg!(&response); // headers are empty, therefore verifier get() panics

    let verifier = response
        .0
        .clone()
        .read()
        .headers
        .get("edgedb-pkce-verifier")
        .expect("expected the verifier cookie to be present in the header map")
        .to_str()
        .expect(
            "expected the `HeaderValue` of `edgedb-pkce-verifier` to contain only ASCII characters",
        )
        .to_string();

    let url = format!("{base_auth_url}/token?code={code}&verifier={verifier}");
    let auth_token = reqwest::get(url).await?.json::<JsonWrapper>().await?.0;
    dbg!(&auth_token);

    response.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            // "auth_token={auth_token}; HttpOnly; Path=/; SameSite=Strict; Secure;"
            // same issue as above during dev
            "auth_token={auth_token}; HttpOnly; Path=/;"
        ))?,
    );

    redirect("/add");

    Ok(())
}

#[component]
pub fn Code() -> impl IntoView {
    view! { <Await future=handle_callback() blocking=true children=|_| () /> }
}

#[component]
pub fn SignIn() -> impl IntoView {
    view! {
        <div style="flex">

            <button on:click=move |_| {
                spawn_local(async move { handle_sign_in().await.unwrap() });
            }>"Sign in"</button>

            <a href="https://accounts.google.com">or create a new google account</a>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use leptos::server;

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
    fn handles_sign_in() {
        handle_sign_in();
    }

    #[test]
    // #[ignore]
    fn test_env_vars() {
        dotenv().ok();

        log!(
            "base auth URL: {}, server port: {}",
            &*BASE_AUTH_URL,
            &*SERVER_PORT
        );
    }
}
