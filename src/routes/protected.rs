use crate::{components::Nav, db::get_authorized_surgeon, surgeon::Surgeon};
use leptos::{
    prelude::{
        ElementChild, Get, IntoAny, IntoView, OnceResource, ServerFnError, Signal, Suspend,
        Suspense, component, expect_context, provide_context, signal, view,
    },
    server::{self, LocalResource, Resource},
    task::spawn_local,
};
use leptos_router::{components::Outlet, hooks::use_navigate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[component]
pub fn Protected() -> impl IntoView {
    // OnceResource call blows up with:
    //
    // thread 'tokio-runtime-worker' panicked at
    // /Users/dn/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/js-sys-0.3.77/src/lib.rs:6063:
    // 9: cannot access imported statics on non-wasm targets
    //
    // origin of that error is in wasm-bindgen:
    // https://github.com/rustwasm/wasm-bindgen/blob/c35cc9369d5e0dc418986f7811a0dd702fb33ef9/crates/backend/src/codegen.rs#L1848
    //
    // This happens with both OnceResource and Resource, and also happens even when you modify the
    // handler to just return a prefab Surgeon without touching the DB
    //
    // AHA! it's hte navigate()/use_navigate() function - it's trying to call that on the server
    // in the short term, just show a page with links to those pages and then work it out later
    let surgeon_resource = Resource::new(|| (), |_| get_authorized_surgeon());

    // let navigate = use_navigate();
    // navigate("/new/terms", Default::default());
    // let navigate = use_navigate();
    // navigate("/the-resource-fetch-in-protected-failed", Default::default());
    view! {
        <Suspense fallback=move || {
            view! { "Checking authorization for the current surgeon..." }
        }>
            {move || Suspend::new(async move {
                if let Ok(Some(surgeon)) = surgeon_resource.await {
                    if surgeon.terms.is_some() {
                        // provide_context(surgeon);

                        view! {
                            <Nav />
                            <Outlet />
                            {format!("we got a surgeon: {surgeon:?}")}
                        }
                            .into_any()
                    } else {
                        view! { {format!("the surgeon has no terms: {surgeon:?}")} }.into_any()
                    }
                } else {
                    view! { "there is no matching surgeon in the DB" }.into_any()
                }
            })}
        </Suspense>
    }
}
