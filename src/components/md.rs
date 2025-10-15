use leptos::{IntoView, component, html::InnerHtmlAttribute, view};
#[cfg(feature = "ssr")] use markdown::to_html;

#[cfg(feature = "ssr")]
pub trait MdParse {
    fn md_parse(self) -> String;
}

#[cfg(feature = "ssr")]
impl<T: AsRef<str>> MdParse for T {
    fn md_parse(self) -> String {
        to_html(self.as_ref())
    }
}

#[component]
pub fn Markdown(html: String) -> impl IntoView {
    view! { <div inner_html=html></div> }
}
