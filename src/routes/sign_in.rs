use leptos::prelude::{ElementChild, IntoView, StyleAttribute, component, view};

#[component]
pub fn SignIn() -> impl IntoView {
    view! {
        <div style={"flex"}>
            <a href={"/signin"} rel={"external"}>
                sign in
            </a>
            <a href={"https://accounts.google.com"}>or create a new google account</a>
        </div>
    }
}
