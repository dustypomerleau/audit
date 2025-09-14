use crate::components::Logo;
use leptos::prelude::{ClassAttribute, ElementChild, GlobalAttributes, IntoView, component, view};

/// A header containing navigation, profile link, and sign out link.
#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <header id="header" class="header">
            <a href="/" title="Vic Eye Audit | Home">
                <div class="logo">
                    <Logo />
                </div>
            </a>
            <a href="/killsession" rel="external">
                "log out"
            </a>
        // todo: burger
        </header>
    }
}
