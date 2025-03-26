#[cfg(feature = "ssr")] use crate::db::db;
use crate::surgeon::Surgeon;
use leptos::{
    Params,
    prelude::{
        Action, ElementChild, IntoAny, IntoView, OnAttribute, Read, ServerFnError, component,
        expect_context, server, use_context, view,
    },
    server::ServerAction,
};
use leptos_router::{
    hooks::{use_navigate, use_query},
    params::Params,
};

#[derive(Clone, Debug, Default, Params, PartialEq)]
pub struct EmailQuery {
    email: String,
}

#[component]
pub fn Terms() -> impl IntoView {
    // Clicking on "I agree":
    // 1. sets the current surgeon's `terms` property to `datetime_current()`
    // 2 (do we need to do something to update the global state/context so that the surgeon has
    //   the right value for Surgeon::terms?)
    // 3. redirects to `/add`

    if let Ok(email_query) = use_query::<EmailQuery>().read().as_ref() {
        let email = email_query.email.clone();
        let accept_terms_action = ServerAction::<AcceptTerms>::new();

        view! {
            "agree to the terms before proceeding"
            <button on:click=move |_| {
                accept_terms_action.dispatch(email.clone().into());
            }></button>
        }
        .into_any()
    } else {
        let navigate = use_navigate();
        navigate("/", Default::default());
        ().into_any()
    };
}

// todo: we need to update both the DB Surgeon and the server state
#[server]
pub async fn accept_terms(email: String) -> Result<(), ServerFnError> {
    let query = format!(
        r#"update Surgeon filter .email = "{email}" set {{ terms := datetime_current() }};"#
    );

    db().await?
        .query_required_single::<Surgeon, _>(query, &())
        .await?;

    Ok(())
}
