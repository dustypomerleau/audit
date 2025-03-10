use crate::db::InsertSurgeon;
use leptos::{
    prelude::{ActionForm, ElementChild, IntoView, component, view},
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

    // ServerAction::<S>::new() creates an action that will call the server function S
    // let insert_surgeon = ServerAction::<InsertSurgeon>::new();
    // let returned_value = insert_surgeon.value();
    // let is_error = move || returned_value.with(|val| matches!(val, Some(err)));

    let insert_surgeon = ServerAction::<InsertSurgeon>::new();

    view! {
        "sign up and complete your profile"
        <ActionForm action=insert_surgeon>
            <label>"Email" <input type="email" name="surgeon[email]" required /></label>
            <label>"First Name" <input type="text" name="surgeon[first_name]" /></label>
            <label>"Last Name" <input type="text" name="surgeon[last_name]" /></label>
            <label>
                "Default Hospital/Site" <input type="text" name="surgeon[default_site]" />
            </label>
            <label>
                "SIA power for right eyes (D)"
                <input
                    type="number"
                    min=0
                    max=2
                    step=0.05
                    name="surgeon[sia[right[power]]]"
                    required
                />
            </label>
            <label>
                "SIA axis for right eyes (°)"
                <input
                    type="number"
                    min=0
                    max=179
                    step=1
                    name="surgeon[sia[right[axis]]]"
                    required
                />
            </label>
            <label>
                "SIA power for left eyes (D)"
                <input
                    type="number"
                    min=0
                    max=2
                    step=0.05
                    name="surgeon[sia[left[power]]]"
                    required
                />
            </label>
            <label>
                "SIA axis for left eyes (°)"
                <input
                    type="number"
                    min=0
                    max=179
                    step=1
                    name="surgeon[sia[left[axis]]]"
                    required
                />
            </label>
            <input type="submit" />
        </ActionForm>
    }
}
