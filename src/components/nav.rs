use leptos::prelude::{ElementChild, IntoView, StyleAttribute, component, view};

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: 1fr 100px; gap: 1rem; padding-bottom: 2rem;">
            <div>
                "a nav toolbar containing: logo and menu/links to (add, instructions, list, reports)"
            </div>
            <a href="/killsession" rel="external">
                "log out"
            </a>
        </div>
    }
}
