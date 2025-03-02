use leptos::prelude::{ElementChild, IntoView, component, view};

#[component]
pub fn Terms() -> impl IntoView {
    // a button, which upon clicking `I agree` calls a server route that sets the current
    // surgeon's `terms` property to `datetime_current()` and redirects to `add`
    view! { "agree to the terms before proceeding" }
}
