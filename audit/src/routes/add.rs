use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;

use crate::components::AddCase;

#[component]
pub fn Add() -> impl IntoView {
    view! { <AddCase /> }
}
