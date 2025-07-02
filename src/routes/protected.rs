// bookmark: todo: dramatically simplify error types:
// Auth
// Db
// ...
//
#[cfg(feature = "ssr")] use crate::{auth::get_jwt_cookie, db::db, state::AppState};
use crate::{
    components::{Nav, SignedOut},
    error::AppError,
    model::Surgeon,
};
#[cfg(feature = "ssr")] use gel_tokio::create_client;
use leptos::prelude::{
    Get, IntoAny, IntoMaybeErased, IntoView, OnceResource, RwSignal, Set, Suspense, component,
    provide_context, server, use_context, view,
};
#[cfg(feature = "ssr")] use leptos_axum::redirect;
use leptos_router::components::Outlet;

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
pub async fn get_authorized_surgeon() -> Result<Option<Surgeon>, AppError> {
    let auth_token = get_jwt_cookie().await?.unwrap_or_else(|| {
        redirect("/signedout");
        // Hack: we only care about redirecting, but the return types need to match.
        "Redirected to /signedout".to_string()
    });

    let client = create_client()
        .await?
        .with_globals_fn(|client| client.set("ext::auth::client_token", &auth_token));

    client.ensure_connected().await?;

    if let Some(state) = use_context::<AppState>() {
        state.db.set(client)?;
    } else {
        redirect("/signedout");
    }

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

    if let Some(json) = db().await?.query_single_json(query, &()).await? {
        let surgeon = serde_json::from_str::<Surgeon>(json.as_ref())?;

        if surgeon.terms.is_some() {
            if let Some(state) = use_context::<AppState>() {
                state.surgeon.set(Some(surgeon.clone()))?;

                Ok(Some(surgeon))
            } else {
                Err(AppError::State(
                    "AppState is not present in context".to_string(),
                ))
            }
        } else {
            redirect("/terms");

            Ok(None)
        }
    } else {
        redirect("/signup");

        Ok(None)
    }
}
