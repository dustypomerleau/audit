use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::InnerHtmlAttribute;
use leptos::prelude::IntoAny;
use leptos::prelude::IntoView;
use leptos::prelude::Resource;
use leptos::prelude::Suspend;
use leptos::prelude::Suspense;
use leptos::prelude::component;
use leptos::prelude::server;
use leptos::prelude::view;
use markdown::to_html;

use crate::error::AppError;

pub static MARKDOWN_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let content_dir =
        env::var("CONTENT_DIR").expect("expected CONTENT_DIR environment variable to be present");

    PathBuf::from(content_dir).join("markdown")
});

/// Types that can be parsed as markdown.
pub trait MdParse {
    /// Parse [`self`] as markdown and return a [`String`] of escaped HTML. This can be done on the
    /// client or the server, depending on the input type.
    // async fn md_parse(self) -> Result<String, AppError>;
    fn md_parse(self) -> impl Future<Output = Result<String, AppError>> + Send;
}

impl MdParse for PathBuf {
    async fn md_parse(self) -> Result<String, AppError> { markdown_from_file(self).await }
}

impl MdParse for &str {
    async fn md_parse(self) -> Result<String, AppError> { Ok(to_html(self)) }
}

impl MdParse for String {
    async fn md_parse(self) -> Result<String, AppError> { self.as_str().md_parse().await }
}

/// Parses the input markdown and injects the HTML returned by `md_parse()`.
#[component]
pub fn Markdown<T: MdParse + Clone + Send + Sync + 'static>(md: T) -> impl IntoView {
    let html_resource = Resource::new(|| (), move |_| md.clone().md_parse());

    view! {
        <Suspense fallback=move || {
            view! { "Loading markdown content..." }
        }>
            {Suspend::new(async move {
                if let Ok(html) = html_resource.await {
                    view! {
                        <div class="markdown-container">
                            <div class="content" inner_html=html></div>
                        </div>
                    }
                        .into_any()
                } else {
                    view! { "unable to load markdown content" }.into_any()
                }
            })}
        </Suspense>
    }
}

#[server]
pub async fn markdown_from_file(path: PathBuf) -> Result<String, AppError> {
    let markdown = tokio::fs::read_to_string(path).await?;

    Ok(to_html(&markdown))
}
