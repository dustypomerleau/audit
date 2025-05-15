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
                "You accept that this is beta software. There will be bugs. You will not hold me or the site liable for any data loss or privacy breach."
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

#[server]
pub async fn accept_terms() -> Result<(), ServerFnError> {
    // In theory, you could select only the terms field and update that, rather than replacing the
    // entire Surgeon here, but that can be optimized later.
    let query = r#"
select (
    update Surgeon
    filter .identity = (select global ext::auth::ClientTokenIdentity)
    set { terms := datetime_current() }
) {
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

    if let Ok(Some(surgeon_json)) = db().await?.query_single_json(query, &()).await {
        let surgeon = serde_json::from_str(surgeon_json.as_ref())?;
        set_current_surgeon(Some(surgeon)).await?;
        // todo: call an async function that sends a transactional email to the new user
        redirect("/protected/add");
    } else {
        redirect("/signedout");
    }

    Ok(())
}
