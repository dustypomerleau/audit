use crate::components::Logo;
use leptos::prelude::{ElementChild, IntoView, StyleAttribute, component, view};

/// A header containing navigation, profile link, and sign out link.
#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: 1fr 100px; gap: 1rem; padding-bottom: 2rem;">
            <div style="width: 10%;">
                <Logo />
            </div>
            <a href="/killsession" rel="external">
                "log out"
            </a>
        </div>
    }
}
