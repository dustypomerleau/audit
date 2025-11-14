use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;
use leptos_router::components::Outlet;

use crate::components::Logo;

/// A header containing navigation, profile link, and sign out link.
#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <header id="header" class="header">
            <a href="/" rel="external" title="Vic Eye Audit | Home">
                <div class="logo">
                    <Logo />
                </div>
            </a>
            // TODO: this link should be "sign in" or "sign out" depending on the cookies
            // but put it inside a burger on mobile
            <a href="/killsession" rel="external">
                "sign out"
            </a>
        // TODO: burger
        // <a href="/signin" rel="external">
        // "sign in"
        // </a>
        </header>
        <Outlet />
    }
}

