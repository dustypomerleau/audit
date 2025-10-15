use crate::{
    components::{Hero, Markdown},
    error::AppError,
};
use leptos::{
    prelude::{
        ElementChild, Get, IntoAny, IntoView, StyleAttribute, Suspense, component, server, view,
    },
    server::{OnceResource, Resource},
};
#[cfg(feature = "ssr")] use markdown::to_html;

#[component]
pub fn Landing() -> impl IntoView {
    let html_resource = OnceResource::new(get_markdown());

    view! {
        <Hero />
        <Suspense>
            {move || {
                if let Some(Ok(html)) = html_resource.get() {
                    view! { <Markdown html=html /> }.into_any()
                } else {
                    "no markdown".into_any()
                }
            }}
        </Suspense>
    }
}

#[server]
pub async fn get_markdown() -> Result<String, AppError> {
    let test_string = r#"
# Some Markdown

- first
- second
- third
    "#;

    let test = to_html(test_string);
    dbg!(&test);

    Ok(test)
}
