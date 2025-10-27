use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;

use crate::components::Hero;
use crate::components::MARKDOWN_PATH;
use crate::components::Markdown;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <Hero />
        <Markdown md=MARKDOWN_PATH.join("landing.md") />
    }
}
