#[cfg(feature = "ssr")]
use crate::{
    auth::{AuthError, get_jwt_cookie},
    db::{DbError, db},
    state::AppState,
};
use crate::{
    components::{Nav, SignedOut},
    model::Surgeon,
};
#[cfg(feature = "ssr")] use gel_tokio::create_client;
use leptos::{
    prelude::{
        FromServerFnError, Get, IntoAny, IntoMaybeErased, IntoView, OnceResource, RwSignal,
        ServerFnError, ServerFnErrorErr, Set, Suspense, component, expect_context, provide_context,
        server, use_context, view,
    },
    server_fn::codec::JsonEncoding,
};
#[cfg(feature = "ssr")] use leptos_axum::{extract, redirect};
use leptos_router::components::Outlet;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Error, Serialize)]
#[error("error accessing a protected route: {0:?}")]
pub struct ProtectedError(pub String);

#[cfg(feature = "ssr")]
impl From<AuthError> for ProtectedError {
    fn from(err: AuthError) -> Self {
        Self(format!("{err:?}"))
    }
}

#[cfg(feature = "ssr")]
impl From<DbError> for ProtectedError {
    fn from(err: DbError) -> Self {
        Self(format!("{err:?}"))
    }
}

impl FromServerFnError for ProtectedError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        Self(format!("{err}"))
    }
}

#[component]
pub fn Protected() -> impl IntoView {
    let current_surgeon = RwSignal::<Option<Surgeon>>::new(None);
    let surgeon_resource = OnceResource::new(get_authorized_surgeon());

    view! {
        <Suspense fallback=move || {
            view! { "Checking authorization for the current surgeon..." }
        }>
            {move || {
                if let Some(Ok(Some(surgeon))) = surgeon_resource.get() {
                    current_surgeon.set(Some(surgeon));
                    provide_context(current_surgeon);

                    view! {
                        <Nav />
                        <Outlet />
                    }
                        .into_any()
                } else {
                    view! { <SignedOut /> }.into_any()
                }
            }}
        </Suspense>
    }
}

#[server]
pub async fn get_authorized_surgeon() -> Result<Option<Surgeon>, ProtectedError> {
    let auth_token = get_jwt_cookie().await?.unwrap_or_else(|| {
        redirect("/signedout");
        // This feels hacky at best, as we only care about the redirect, but the return
        // types need to match.
        "Redirected to /signedout".to_string()
    });

    let client = create_client()
        .await
        .map_err(|err| ProtectedError(format!("{err:?}")))?
        .with_globals_fn(|client| client.set("ext::auth::client_token", &auth_token));

    client
        .ensure_connected()
        .await
        .map_err(|err| ProtectedError(format!("{err:?}")))?;

    expect_context::<AppState>()
        .db
        .set(client)
        .map_err(|err| ProtectedError(format!("{err:?}")))?;

    let query = r#"
select global cur_surgeon {
    email,
    terms,
    first_name,
    last_name,

    defaults: {
        site: { name },
        iol: { model, name, company, focus, toric },
        formula,
        custom_constant,
        main
    },

    sia: { right: { power, axis }, left: { power, axis } }
};
        "#;

    let query_result = db().await?.query_single_json(query, &()).await;

    match query_result {
        Ok(Some(json)) => {
            if let Ok(surgeon) = serde_json::from_str::<Surgeon>(json.as_ref()) {
                if surgeon.terms.is_some() {
                    expect_context::<AppState>()
                        .surgeon
                        .set(Some(surgeon.clone()))
                        .map_err(|err| ProtectedError(format!("{err:?}")))?;

                    Ok(Some(surgeon))
                } else {
                    redirect("/terms");

                    Ok(None)
                }
            } else {
                redirect("/signup");

                Ok(None)
            }
        }

        _ => Err(ProtectedError(format!("{query_result:?}"))),
    }
}
