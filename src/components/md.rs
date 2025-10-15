use crate::error::AppError;
use leptos::prelude::{
    Get, InnerHtmlAttribute, IntoView, Resource, Suspense, component, server, view,
};
use markdown::to_html;
use std::path::PathBuf;

pub enum Md {
    Inline(&'static str),
    File(&'static str),
}

/// Types that can be parsed as markdown.
pub trait MdParse {
    /// Parse [`self`] as markdown and return a [`String`] of escaped HTML. This can be done on the
    /// client or the server, depending on the input type.
    fn md_parse(self) -> Result<String, AppError>;
}

impl MdParse for &str {
    fn md_parse(self) -> Result<String, AppError> {
        Ok(to_html(self))
    }
}

impl MdParse for String {
    fn md_parse(self) -> Result<String, AppError> {
        self.as_str().md_parse()
    }
}

impl MdParse for PathBuf {
    fn md_parse(self) -> Result<String, AppError> {
        let html_resource = Resource::new(|| (), move |_| markdown_from_file(self.clone()));

        if let Some(Ok(html)) = html_resource.get() {
            Ok(html)
        } else {
            Err(AppError::Server(
                "unable to get the resource that returns HTML using `markdown_from_file()`"
                    .to_string(),
            ))
        }
    }
}

/// Parses the input markdown from &str or a file, and injects the HTML returned by `md_parse()`.
#[component]
pub fn Markdown(md: Md) -> impl IntoView {
    view! {
        <Suspense>
            {move || {
                let html = match md {
                    Md::Inline(md) => {
                        md.md_parse().unwrap_or("inline markdown could not be parsed.".to_string())
                    }
                    Md::File(path) => {
                        PathBuf::from(path)
                            .md_parse()
                            .unwrap_or("markdown from file could not be opened/parsed".to_string())
                    }
                };

                // todo: Escaping this html with html_escape::encode_text() results in the html
                // being displayed as a string in the view, rather than as html.
                view! { <div inner_html=html></div> }
            }}
        </Suspense>
    }
}

#[server]
pub async fn markdown_from_file(path: PathBuf) -> Result<String, AppError> {
    let markdown = std::fs::read_to_string(path)?;

    Ok(to_html(&markdown))
}
