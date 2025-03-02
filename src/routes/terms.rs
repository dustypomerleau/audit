use leptos::prelude::{ElementChild, IntoView, component, view};

#[component]
pub fn Terms() -> impl IntoView {
    view! { "agree to the terms before proceeding" <a href="signup"> }
}
