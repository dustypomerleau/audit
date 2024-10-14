use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::format;
use http::{header, HeaderValue};
use leptos::{expect_context, logging::log, server, ServerFnError};
use leptos_axum::{redirect, ResponseOptions};
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

// todo:
// 1. add a path in the router that calls handle_ui_signin() and handle_ui_signup()
// 2. inside handle_ui_signin(), call generate_pkce() and add the resulting challenge value to the
//    search params map with a key of "challenge", then redirect to that updated URI
// 3. set a cookie with the value of "verifier" using the template in:
// https://docs.edgedb.com/guides/auth/built_in_ui#link-to-built-in-ui
// look at leptos/integrations/axum/src/lib.rs `ResponseParts` and `ResponseOptions`
// see leptos/examples/todo_app_sqlite_axum/src/todo.rs for cookie

#[server(HandleSignIn, "/signin")]
pub async fn handle_sign_in() -> Result<(), ServerFnError> {
    let Pkce {
        verifier,
        challenge,
    } = generate_pkce();

    // The correct path for the redirect in dev is:
    // http://localhost:10702/branch/dev/ext/auth/ui/signin?challenge={challenge}
    // you will obviously need a different environment variable for prod
    redirect(&format!(
        "{}/ui/signin?challenge={challenge}",
        &*BASE_AUTH_URL
    ));

    // -----------------------------------------

    let response = expect_context::<ResponseOptions>();

    response.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&format!("edgedb-pkce-verifier={verifier}"))?,
    );

    log!("{response:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use leptos::server;

    #[test]
    fn generates_pkce() {
        let pkce = generate_pkce();

        println!(
            "{pkce:?}, verifier length: {:?}, challenge length: {:?}",
            pkce.verifier.len(),
            pkce.challenge.len()
        );

        assert!(pkce.verifier.len() == pkce.challenge.len() && pkce.verifier.len() == 43);
    }

    #[test]
    fn test_env_vars() {
        dotenv().ok();

        println!(
            "base auth URL: {}, server port: {}",
            &*BASE_AUTH_URL, &*SERVER_PORT
        );
    }
}
