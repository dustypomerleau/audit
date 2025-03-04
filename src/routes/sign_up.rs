use leptos::{
    prelude::{IntoView, component, view},
    server::ServerAction,
};

#[component]
pub fn SignUp() -> impl IntoView {
    // an `<ActionForm>` with all the required details for a new Surgeon
    //
    // on submit:
    // 1. create a Surgeon (without a value for `terms:`)
    // 2. call an async function that sends a transactional email with info
    // 3. redirect to /terms

    // ServerAction::new::<S>() creates an action that will call the server function S
    // let create_surgeon = ServerAction::new();
    // let returned_value = create_surgeon.value();
    // let is_error = move || returned_value.with(|val| matches!(val, Some(err)));
    view! { "sign up and complete your profile" }
}
