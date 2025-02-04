#[cfg(feature = "ssr")] use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::format;
#[cfg(feature = "ssr")] use http::{HeaderValue, header};
use leptos::{
    logging::log,
    prelude::{
        Await, ElementChild, IntoView, OnAttribute, Read, Result, ServerFnError, StorageAccess,
        StyleAttribute, Suspend, Suspense, component, expect_context, server, view,
    },
    server::{LocalResource, OnceResource, Resource},
    task::spawn_local,
};
#[cfg(feature = "ssr")] use leptos_axum::{ResponseOptions, redirect};
use leptos_router::hooks::{query_signal, use_navigate, use_params_map, use_query_map};
#[cfg(feature = "ssr")] use rand::{Rng, random, rng};
use serde::Deserialize;
/* todo re: wasm https://github.com/rust-random/rand/issues/991 */
#[cfg(feature = "ssr")] use sha2::{Digest, Sha256};
use std::{env, sync::LazyLock};
use thiserror::Error;

#[component]
pub fn SignIn() -> impl IntoView {
    view! {
        <div style="flex">
            <a href="/signin"></a>
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
