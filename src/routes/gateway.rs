use leptos::prelude::ElementChild;
use leptos::prelude::IntoView;
use leptos::prelude::StyleAttribute;
use leptos::prelude::component;
use leptos::prelude::view;

#[component]
pub fn Gateway() -> impl IntoView {
    view! {
        <div style="display: grid; gap: 20px;">
            <div>"Welcome to the gateway"</div>
            <a href="/protected/add">"Existing users"</a>
            <a href="/signup">"New user sign up"</a>
        </div>
    }
}
