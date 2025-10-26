use std::env::current_exe;

use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;

use crate::components::Hero;
use crate::components::Markdown;

#[component]
pub fn Landing() -> impl IntoView {
    let markdown_path_buf = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("site/markdown/landing.md");

    view! {
        <Hero />
        <Markdown md=markdown_path_buf />
    }
}
