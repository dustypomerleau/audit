use leptos::prelude::ElementChild;
use leptos::prelude::IntoView;
use leptos::prelude::component;
use leptos::prelude::view;

#[component]
pub fn SignedOut() -> impl IntoView {
    view! {
        "Please "
        <a href="/signin" rel="external">
            "sign in"
        </a>
        " to proceed."
    }
}
