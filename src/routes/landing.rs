use crate::components::{Hero, Markdown, Md};
use leptos::prelude::{IntoView, component, view};

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <Hero />
        <Markdown md=Md::File("markdown/landing.md") />
    }
}
