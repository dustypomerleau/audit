use crate::components::Hero;
use leptos::prelude::{ElementChild, IntoView, StyleAttribute, component, view};

#[component]
pub fn Landing() -> impl IntoView {
    view! { <Hero /> }
}
