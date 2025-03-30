#[cfg(feature = "ssr")] use crate::surgeon::Surgeon;
#[cfg(feature = "ssr")] use crate::{db::db, surgeon::set_current_surgeon};
use leptos::prelude::{
    ElementChild, IntoView, OnAttribute, ServerAction, ServerFnError, StyleAttribute, component,
    server, view,
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
        "I'm just an ophthalmologist, like you. I am doing everything I can to ensure privacy and security, but I'm not a professional developer. Please agree to the terms before continuing:"
        <ul>
            <li>
                "You agree to be honest. The data you enter should be accurate, and there should be"
                <emph>"no selection bias"</emph>"."
            </li>
            <li>
                "You accept that this is beta software. You will not hold me or the site liable for any data loss or privacy breach."
            </li>
        </ul>
        <button
            style="width: 10rem; height: 4rem"
            on:click=move |_| {
                accept_terms_action.dispatch(AcceptTerms {});
            }
        >
            "Accept the terms"
        </button>
    }
}

// todo: we need to update both the DB Surgeon and the server state
// also, we were redirecting on the client if there was a problem, but easier to just do that here
//
// todo: this query won't match the Surgeon struct because you didn't get the fields
#[server]
pub async fn accept_terms() -> Result<(), ServerFnError> {
    let query = r#"
select (
    update Surgeon
    filter .identity = (select global ext::auth::ClientTokenIdentity)
    set {{ terms := datetime_current() }}
) {{
    email,
    terms,
    first_name,
    last_name,
    default_site: {{ name }},
    sia: {{
        right: {{ power, axis}},
        left: {{ power, axis }}
    }}
}};
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
