use leptos::prelude::{ElementChild, IntoMaybeErased, IntoView, component, view};

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
