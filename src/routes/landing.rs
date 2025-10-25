use std::env::current_exe;
use std::path::Path;

use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;

use crate::components::Hero;
use crate::components::Markdown;
use crate::components::Md;

#[component]
pub fn Landing() -> impl IntoView {
    let markdown_path = current_exe()
        .unwrap_or(".".into())
        .join("site/markdown/landing.md");

    view! {
        <Hero />
        <Markdown md=Md::File(markdown_path.as_str()) />
    }
}
