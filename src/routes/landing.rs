use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;

use crate::components::Hero;
use crate::components::Markdown;
use crate::components::Md;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <Hero />
        <Markdown md=Md::File("markdown/landing.md") />
    }
}
