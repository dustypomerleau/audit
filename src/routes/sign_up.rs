use leptos::prelude::{IntoView, component, view};

#[component]
pub fn SignUp() -> impl IntoView {
    // a form with all the required details for a new Surgeon
    //
    // on submit:
    // 1. create a Surgeon (without a value for `terms:`)
    // 2. call an async function that sends a transactional email with info
    // 3. redirect to /terms
    view! { "sign up and complete your profile" }
}
