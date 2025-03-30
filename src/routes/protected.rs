#[cfg(feature = "ssr")] use crate::{auth::get_jwt_cookie, db::db, state::AppState};
use crate::{components::Nav, surgeon::Surgeon};
#[cfg(feature = "ssr")] use gel_tokio::Queryable;
#[cfg(feature = "ssr")] use leptos::prelude::expect_context;
use leptos::{
    prelude::{
        IntoAny, IntoView, RwSignal, ServerFnError, Set, Suspend, Suspense, component,
        provide_context, server, view,
    },
    server::OnceResource,
};
#[cfg(feature = "ssr")] use leptos_axum::redirect;
use leptos_router::{components::Outlet, hooks::use_navigate};

#[component]
pub fn Protected() -> impl IntoView {
    let current_surgeon = RwSignal::<Option<Surgeon>>::new(None);
    let surgeon_resource = OnceResource::new(get_authorized_surgeon());

    view! {
        <Suspense fallback=move || {
            view! { "Checking authorization for the current surgeon..." }
        }>
            {move || Suspend::new(async move {
                if let Ok(Some(surgeon)) = surgeon_resource.await {
                    if surgeon.terms.is_some() {
                        current_surgeon.set(Some(surgeon));
                        provide_context(current_surgeon);

                        view! {
                            <Nav />
                            <Outlet />
                        }
                            .into_any()
                    } else {
                        let navigate = use_navigate();
                        navigate("/terms", Default::default());
                        ().into_any()
                    }
                } else {
                    let navigate = use_navigate();
                    navigate("/", Default::default());
                    ().into_any()
                }
            })}
        </Suspense>
    }
}

#[server]
pub async fn get_authorized_surgeon() -> Result<Option<Surgeon>, ServerFnError> {
    let auth_token = get_jwt_cookie().await?;

    // In this query, `signed_in` returns a bool that tells us whether the JWT in the
    // `gel-auth-token` cookie matches the JWT stored as a global on the DB client. This is our
    // first check that nothing is fundamentally wrong with the session.
    //
    // Then we check the `Identity` that matches that JWT, which is computed and stored as the
    // global `ext::auth::ClientTokenIdentity`. If there is a `Surgeon` with the same identity,
    // then we return the `Surgeon` object from the DB, so the frontend can share it as context.
    // We also set the `surgeon` value in global server state to the returned `Surgeon`.
    //
    // If there isn't a matching `Surgeon`, then the surgeon still needs to complete the signup
    // flow. We just return an empty set, and respond to that with a redirect to the signup form and
    // then the terms.
    //
    // todo: consider whether you need to check signed_in at all, because if the user isn't signed
    // in, then there would be no ClientTokenIdentity.
    let query = format!(
        r#"
with
    signed_in := (select "{auth_token}" = (select global ext::auth::client_token)),
    identity := (select global ext::auth::ClientTokenIdentity)

select {{
    signed_in := signed_in,

    surgeon := (select Surgeon {{
        email,
        terms,
        first_name,
        last_name,
        default_site: {{ name }},
        sia: {{
            right: {{ power, axis }},
            left: {{ power, axis }}
        }}
    }} filter .identity = identity)
}};
        "#
    );

    #[derive(Debug, Queryable)]
    struct SurgeonQuery {
        signed_in: bool,
        surgeon: Option<Surgeon>,
    }

    let client = db().await?;

    let surgeon_result = client.query_single::<SurgeonQuery, _>(query, &()).await;
    dbg!(&surgeon_result);

    match surgeon_result {
        Ok(Some(SurgeonQuery {
            signed_in: true,
            surgeon: Some(surgeon),
        })) => {
            if surgeon.terms.is_some() {
                expect_context::<AppState>()
                    .surgeon
                    .set(Some(surgeon.clone()))?;

                Ok(Some(surgeon))
            } else {
                redirect("/terms");
                Ok(None)
            }
        }

        _ => {
            redirect("/");
            Ok(None)
        }
    }
}
