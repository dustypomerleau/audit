use leptos::prelude::{ElementChild, IntoView, component, view};

#[component]
pub fn Add() -> impl IntoView {
    view! {
        "add a new case"
        <a href="/killsession" rel="external">
            "log out"
        </a>
    }
}
