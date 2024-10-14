use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::format;
use http::{header, HeaderValue};
use leptos::{
    component, expect_context, logging::log, server, spawn_local, view, IntoView, ServerFnError,
};
#[cfg(feature = "ssr")] use leptos_axum::{redirect, ResponseOptions};
use leptos_router::create_query_signal;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};
use std::{env, sync::LazyLock};

static BASE_AUTH_URL: LazyLock<String> = LazyLock::new(|| {
    env::var("BASE_AUTH_URL").expect("base auth URL environment variable to be present")
});

static SERVER_PORT: LazyLock<String> = LazyLock::new(|| {
    env::var("SERVER_PORT").expect("server port environment variable to be present")
});

#[derive(Debug)]
pub struct Pkce {
    verifier: String,
    challenge: String,
}

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

    redirect(&format!(
        "{}/ui/signin?challenge={challenge}",
        &*BASE_AUTH_URL
    ));

    log!("{response:?}");

    Ok(())
}

#[component]
pub fn SignIn() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async { handle_sign_in().await.unwrap() });
        }>"Sign in"</button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use leptos::server;

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
    fn handles_sign_in() {
        handle_sign_in();
    }

    #[test]
    fn test_env_vars() {
        dotenv().ok();

        log!(
            "base auth URL: {}, server port: {}",
            &*BASE_AUTH_URL,
            &*SERVER_PORT
        );
    }
}
