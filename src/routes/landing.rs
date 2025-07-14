use leptos::prelude::{ElementChild, IntoView, StyleAttribute, component, view};

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        "the landing page!"
        <div style="flex">
            <a href="/signin" rel="external" style="margin-right: 20px">
                sign in (requires Google)
            </a>
            <a href="https://accounts.google.com">or create a new google account</a>
        </div>
    }
}
