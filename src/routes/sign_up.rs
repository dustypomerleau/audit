use leptos::prelude::{IntoView, component, view};

#[component]
pub fn SignUp() -> impl IntoView {
    // a form with all the required details for a new Surgeon
    // on submit, we create a Surgeon (without a value for `terms:`) and redirect to /terms
    view! { "sign up and complete your profile" }
}
