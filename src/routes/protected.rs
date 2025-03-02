use crate::components::Nav;
use leptos::prelude::{IntoView, component, view};
use leptos_router::components::Outlet;

#[component]
pub fn Protected() -> impl IntoView {
    // todo: the signal checks not only that `app_state.surgeon.is_some()`, but also that
    // `app_state.surgeon.terms` is a Some value before the present time
    view! {
        <Nav />
        <Outlet />
    }
}
