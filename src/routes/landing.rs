use crate::components::{Hero, Markdown, Md};
use leptos::prelude::{IntoView, component, view};

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <Hero />
        <Markdown md=Md::Inline(
            r#"
# Example of markdown as a raw &str

- a bullet in the raw markdown
            "#,
        ) />
        <Markdown md=Md::File("markdown/landing.md") />
    }
}
