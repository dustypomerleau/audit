#[cfg(feature = "ssr")] use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::format;
#[cfg(feature = "ssr")] use http::{header, HeaderValue};
use leptos::{
    logging::log,
    prelude::{
        component, expect_context, server, view, Await, ElementChild, IntoView, OnAttribute, Read,
        Result, ServerFnError, StorageAccess, StyleAttribute,
    },
    server::{LocalResource, OnceResource, Resource},
    task::spawn_local,
};
#[cfg(feature = "ssr")] use leptos_axum::{redirect, ResponseOptions};
use leptos_router::hooks::{query_signal, use_params_map};
#[cfg(feature = "ssr")] use rand::{thread_rng, Rng}; /* todo re: wasm https://github.com/rust-random/rand/issues/991 */
use sha2::{Digest, Sha256};
use std::{env, sync::LazyLock};

// note: new API for dotenvy will arrive in v16 release
pub static BASE_AUTH_URL: LazyLock<String> = LazyLock::new(|| {
    env::var("BASE_AUTH_URL").expect("base auth URL environment variable to be present")
});

pub static SERVER_PORT: LazyLock<String> = LazyLock::new(|| {
    env::var("SERVER_PORT").expect("server port environment variable to be present")
});

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
    let input: [u8; 32] = thread_rng().gen();
    let verifier = Base64UrlUnpadded::encode_string(&input);
    // 2. SHA256 hash the result, then URL-encode again:
    let hash = Sha256::new().chain_update(&verifier).finalize();
    let challenge = Base64UrlUnpadded::encode_string(&hash);

    Pkce {
        verifier,
        challenge,
    }
}

/// Server function that generates a [`Pkce`] struct, populates the URL param and the verifier
/// cookie, and redirects to the OAuth flow.
#[server(endpoint = "/signin")]
async fn handle_sign_in() -> Result<(), ServerFnError> {
    let Pkce {
        verifier,
        challenge,
    } = generate_pkce();

    let response = expect_context::<ResponseOptions>();

    response.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "edgedb-pkce-verifier={verifier}; HttpOnly; Path=/; SameSite=Strict; Secure;"
        ))?,
    );

    log!("{response:?}");

    redirect(&format!(
        "{}/ui/signin?challenge={challenge}",
        &*BASE_AUTH_URL
    ));

    Ok(())
}

#[server]
pub async fn handle_callback(code: String) -> Result<(), ServerFnError> {
    // 1. Google Oauth redirects to the URL set in `edgedb ui` > Auth > Providers > `redirect_to`.
    // 2. get the code from the query string in the URL (?code=...)
    // 3. get the value of `verifier` from the cookie
    // 4. redirect to `format!({BASE_AUTH_URL}/token?code={code}&verifier={verifier})` (specifically
    //    this is a GET that returns JSON)
    // 5. save the JSON in the variable `auth_token`
    // 6. set a cookie `edgedb-auth-token={auth_token}` (HttpOnly; Path=/; Secure; SameSite=Strict)
    // 7. redirect to "/add" or some dashboard and use the cookie to determine identity

    let base_url = &*BASE_AUTH_URL;

    let verifier = expect_context::<ResponseOptions>()
        .0
        .clone()
        .read()
        .headers
        .get("edgedb-pkce-verifier")
        .expect("expected verifier to be present in the header map")
        .to_str()
        .expect("expected `HeaderValue` to contain only ASCII characters")
        .to_string();

    let auth_token =
        reqwest::get(&format!("{base_url}/token?code={code}&verifier={verifier}")).await?;

    log!("{auth_token:?}");

    // todo: replace dummy redirect once auth flow is working
    redirect("https://google.com");

    Ok(())
}

#[component]
pub fn Code() -> impl IntoView {
    let res = LocalResource::new(|| async move {
        let code = use_params_map()
            .read()
            .get("code")
            .expect("expected code to be present in the params map");

        handle_callback(code).await.unwrap()
    });
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
