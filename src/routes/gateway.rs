use leptos::prelude::{ElementChild, IntoView, StyleAttribute, component, view};

#[component]
pub fn Gateway() -> impl IntoView {
    view! {
        <div style="display: grid; gap: 20px;">
            <div>"Welcome to the gateway"</div>
            <a href="/protected/add">"Existing users"</a>
            <a href="/new/signup">"New user sign up"</a>
        </div>
    }
}
