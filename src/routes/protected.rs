#[cfg(feature = "ssr")] use gel_tokio::create_client;
use leptos::prelude::Get;
use leptos::prelude::IntoAny;
use leptos::prelude::IntoView;
use leptos::prelude::OnceResource;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Suspense;
use leptos::prelude::component;
use leptos::prelude::provide_context;
use leptos::prelude::server;
#[cfg(feature = "ssr")] use leptos::prelude::use_context;
use leptos::prelude::view;
#[cfg(feature = "ssr")] use leptos_axum::redirect;
use leptos_router::components::Outlet;

#[cfg(feature = "ssr")] use crate::auth::get_jwt_cookie;
use crate::components::SignedOut;
#[cfg(feature = "ssr")] use crate::db::db;
use crate::error::AppError;
use crate::model::Surgeon;
#[cfg(feature = "ssr")] use crate::state::AppState;

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

                    view! { <Outlet /> }
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

    // `create_client()` errors if a connection can't immediately be established, so it
    // isn't necessary to call `ensure_connection()` if the client was created through this
    // convenience method.
    let client = create_client()
        .await?
        .with_globals_fn(|client| client.set("ext::auth::client_token", &auth_token));

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
