use crate::components::Logo;
use leptos::prelude::{ClassAttribute, ElementChild, GlobalAttributes, IntoView, component, view};
use leptos_router::components::Outlet;

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
            // todo: this link should be "sign in" or "sign out" depending on the cookies
            <a href="/killsession" rel="external">
                "sign out"
            </a>
        // todo: burger
        </header>
        <Outlet />
    }
}
