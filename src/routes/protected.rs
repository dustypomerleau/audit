use leptos::prelude::{IntoView, component, view};
use leptos_router::components::Outlet;

#[component]
pub fn Protected() -> impl IntoView {
    view! { <Outlet /> }
}
