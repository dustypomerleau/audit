use leptos::prelude::{ElementChild, IntoView, component, view};

#[component]
pub fn Terms() -> impl IntoView {
    // Clicking on "I agree":
    // 1. sets the current surgeon's `terms` property to `datetime_current()`
    // 2. redirects to `/add`
    view! { "agree to the terms before proceeding" }
}
