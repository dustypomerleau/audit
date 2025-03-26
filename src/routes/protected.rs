#[cfg(feature = "ssr")] use crate::auth::get_jwt_cookie;
#[cfg(feature = "ssr")] use crate::db::db;
#[cfg(feature = "ssr")] use crate::state::AppState;
use crate::{components::Nav, surgeon::Surgeon};
#[cfg(feature = "ssr")] use gel_tokio::Queryable;
use leptos::prelude::{
    Get, IntoAny, IntoView, Resource, RwSignal, ServerFnError, Set, Suspend, Suspense, component,
    expect_context, provide_context, server, view,
};
#[cfg(feature = "ssr")] use leptos_axum::redirect;
use leptos_router::{components::Outlet, hooks::use_navigate};

#[component]
pub fn Protected() -> impl IntoView {
    let current_surgeon = RwSignal::<Option<Surgeon>>::new(None);

    let surgeon_resource =
        Resource::new(move || current_surgeon.get(), |_| get_authorized_surgeon());

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
                        navigate(
                            &format!("/new/terms?email={}", surgeon.email),
                            Default::default(),
                        );
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
    let surgeon_query = format!(
        r#"
with
    signed_in := (select "{auth_token}" = (select global ext::auth::client_token)),
    identity := (select global ext::auth::ClientTokenIdentity),

    QuerySurgeon := (select Surgeon {{
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

select {{
    signed_in := signed_in,
    surgeon := QuerySurgeon if signed_in = true else {{}}
}};
        "#
    );

    #[derive(Debug, Queryable)]
    struct SurgeonQuery {
        signed_in: bool,
        surgeon: Option<Surgeon>,
    }

    let client = db().await?;

    let surgeon_result = client
        .query_single::<SurgeonQuery, _>(surgeon_query, &())
        .await;
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
                redirect(&format!("/new/terms?email={}", surgeon.email));
                Ok(None)
            }
        }

        _ => {
            redirect("/");
            Ok(None)
        }
    }
}
