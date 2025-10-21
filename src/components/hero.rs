use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;

use crate::components::ArrowIol;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="hero hero-text neon-pink-text">
            <h1>"Level UP"</h1>
            <div class="hero-iol neon-pink-svg">
                <ArrowIol />
            </div>
            <img src="images/neon-iol.avif" class="neon-iol-image" />
        </div>
    }
}
