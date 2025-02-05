#[cfg(feature = "ssr")] use base64ct::{Base64UrlUnpadded, Encoding};
use chrono::format;
#[cfg(feature = "ssr")] use http::{HeaderValue, header};
use leptos::{
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
            <a href="/signin" rel="external">
                sign in
            </a>
            <a href="https://accounts.google.com">or create a new google account</a>
        </div>
    }
}
