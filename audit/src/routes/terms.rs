use leptos::prelude::ElementChild;
use leptos::prelude::IntoView;
use leptos::prelude::OnAttribute;
use leptos::prelude::ServerAction;
use leptos::prelude::ServerFnError;
use leptos::prelude::StyleAttribute;
use leptos::prelude::component;
use leptos::prelude::server;
use leptos::prelude::view;
#[cfg(feature = "ssr")] use leptos_axum::redirect;
#[cfg(feature = "ssr")] use sqlx::query;

#[cfg(feature = "ssr")] use crate::db::db;
#[cfg(feature = "ssr")] use crate::model::set_current_surgeon;

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

// TODO:
#[server]
pub async fn accept_terms() -> Result<(), ServerFnError> {
    query!(
        r#"
update surgeon set terms = now() where email = 'todo@todo.com';
        "#
    );

    // TODO: fix the query to return the QuerySurgeon, convert to Surgeon and update globabl state.
    //
    // if let Ok(Some(surgeon_json)) = db().await?.query_single_json(query, &()).await {
    //     let surgeon = serde_json::from_str(surgeon_json.as_ref())?;
    //     set_current_surgeon(Some(surgeon)).await?;
    //     // TODO: call an async function that sends a transactional email to the new user
    //     redirect("/protected/add");
    // } else {
    //     redirect("/signedout");
    // }

    Ok(())
}
