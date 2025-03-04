use leptos::prelude::{IntoView, component, view};

// possibly not needed at all if we can show the outlet using a current surgeon derived from
// context
#[component]
pub fn Container() -> impl IntoView {
    view! { "A container view that only shows the outlet if there's a surgeon" }
}
