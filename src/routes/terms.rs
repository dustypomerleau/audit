#[cfg(feature = "ssr")] use crate::surgeon::Surgeon;
#[cfg(feature = "ssr")] use crate::{db::db, surgeon::set_current_surgeon};
use leptos::prelude::{
    ElementChild, IntoView, OnAttribute, ServerAction, ServerFnError, component, server, view,
};
#[cfg(feature = "ssr")] use leptos_axum::redirect;

#[component]
pub fn Terms() -> impl IntoView {
    // Clicking on "I agree":
    // 1. sets the current surgeon's `terms` property to `datetime_current()`
    // 2 (do we need to do something to update the global state/context so that the surgeon has
    //   the right value for Surgeon::terms?)
    // 3. redirects to `/add`

    let accept_terms_action = ServerAction::<AcceptTerms>::new();

    view! {
        "agree to the terms before proceeding"
        <button on:click=move |_| {
            accept_terms_action.dispatch(AcceptTerms {});
        }></button>
    }
}

// todo: we need to update both the DB Surgeon and the server state
// also, we were redirecting on the client if there was a problem, but easier to just do that here
#[server]
pub async fn accept_terms() -> Result<(), ServerFnError> {
    let query = r#"
update Surgeon filter .identity = (select global ext::auth::ClientTokenIdentity)
set {{ terms := datetime_current() }};
    "#;

    if let Ok(surgeon) = db()
        .await?
        .query_required_single::<Surgeon, _>(query, &())
        .await
    {
        set_current_surgeon(Some(surgeon)).await?;
        redirect("/protected/add");
    } else {
        redirect("/");
    }

    Ok(())
}
