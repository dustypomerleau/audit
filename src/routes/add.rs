use crate::components::AddCase;
use leptos::prelude::{ElementChild, IntoView, component, view};

#[component]
pub fn Add() -> impl IntoView {
    view! { <AddCase /> }
}
