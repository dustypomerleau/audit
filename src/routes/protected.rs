#[cfg(feature = "ssr")] use crate::{auth::get_jwt_cookie, db::db, state::AppState};
use crate::{
    components::{Nav, SignedOut},
    surgeon::Surgeon,
};
#[cfg(feature = "ssr")] use gel_tokio::Queryable;
#[cfg(feature = "ssr")] use leptos::prelude::expect_context;
use leptos::prelude::{
    IntoAny, IntoView, OnceResource, RwSignal, ServerFnError, Set, Suspend, Suspense, component,
    provide_context, server, view,
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
            {move || Suspend::new(async move {
                if let Ok(Some(surgeon)) = surgeon_resource.await {
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
    // If there is no `global cur_surgeon` then the surgeon still needs to complete the signup flow.
    // We just return an empty set, and respond to that with a redirect to the signup form and then
    // the terms.
    let query = format!(
        r#"
select {{
    signed_in := (select "{auth_token}" = (select global ext::auth::client_token)),

    surgeon := (select global cur_surgeon {{
        email,
        terms,
        first_name,
        last_name,

        defaults: {{
            site: {{ name }},
            iol: {{ model, name, company, focus, toric }},
            formula,
            custom_constant,
            main
        }},
        
        sia: {{ right: {{ power, axis }}, left: {{ power, axis }} }}
    }})
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

        // If a new user attempts to navigate directly to a protected route without
        // completing sign-up, we will hit this path.
        Ok(Some(SurgeonQuery {
            signed_in: true,
            surgeon: None,
        })) => {
            redirect("/signup");
            Ok(None)
        }

        _ => {
            redirect("/signedout");
            Ok(None)
        }
    }
}
