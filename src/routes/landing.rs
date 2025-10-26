use std::env::current_exe;
use std::path::PathBuf;

use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;

use crate::components::Hero;
use crate::components::Markdown;

#[component]
pub fn Landing() -> impl IntoView {
    // // todo: this fails in prod because the binary is at the root, whereas in dev, the binary is
    // // inside debug/, so there is an extra call to parent() here that fails in prod.
    // let markdown_path_buf = current_exe()
    //     .unwrap()
    //     .parent()
    //     .unwrap()
    //     .parent()
    //     .unwrap()
    //     .join("site/markdown/landing.md");

    let markdown_path_buf = PathBuf::from("markdown/landing.md");

    view! {
        <Hero />
        <Markdown md=markdown_path_buf />
    }
}
