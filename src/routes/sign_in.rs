#[cfg(feature = "ssr")] use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::format;
#[cfg(feature = "ssr")] use http::{header, HeaderValue};
use leptos::{
    logging::log,
    prelude::{
        component, expect_context, server, view, ElementChild, IntoView, OnAttribute, Result,
        ServerFnError, StyleAttribute,
    },
    server::Resource,
    task::spawn_local,
};
#[cfg(feature = "ssr")] use leptos_axum::{redirect, ResponseOptions};
use leptos_router::hooks::query_signal;
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

#[derive(Debug)]
pub struct Pkce {
    verifier: String,
    challenge: String,
}

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

// todo: this (and the callback) need to be blocking, see https://github.com/leptos-rs/leptos/issues/3147#issuecomment-2430892087
#[server(endpoint = "/signin")]
async fn handle_sign_in() -> Result<(), ServerFnError> {
    let Pkce {
        verifier,
        challenge,
    } = generate_pkce();

    let response = expect_context::<ResponseOptions>();

    response.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!("edgedb-pkce-verifier={verifier}"))?,
    );

    log!("{response:?}");

    redirect(&format!(
        "{}/ui/signin?challenge={challenge}",
        &*BASE_AUTH_URL
    ));

    log!("{response:?}");

    Ok(())
}

// wip todo: chip away at this
#[server(endpoint = "/code")]
pub async fn handle_callback() -> Result<(), ServerFnError> {
    let response = expect_context::<ResponseOptions>();

    if let Some(code) = response.0.clone().read().headers.get("code") {
        log!("{code:?}");
        Ok(())
    } else {
        log!("the code wasn't found in the header map");
        Ok(())
    }
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
