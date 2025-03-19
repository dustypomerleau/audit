use crate::db::InsertSurgeon;
use leptos::{
    prelude::{ActionForm, ElementChild, IntoView, StyleAttribute, component, view},
    server::ServerAction,
};

#[component]
pub fn SignUp() -> impl IntoView {
    // an `<ActionForm>` with all the required details for a new Surgeon
    //
    // on submit:
    // 1. create a Surgeon (without a value for `terms:`)
    // 2. call an async function that sends a transactional email with info
    // 3. redirect to /new/terms

    // ServerAction::<S>::new() creates an action that will call the server function S
    // let insert_surgeon = ServerAction::<InsertSurgeon>::new();
    // let returned_value = insert_surgeon.value();
    // let is_error = move || returned_value.with(|val| matches!(val, Some(err)));

    let insert_surgeon = ServerAction::<InsertSurgeon>::new();

    view! {
        <ActionForm action=insert_surgeon>
            <div style="display: grid; grid-auto-columns: 1fr; grid-gap: 30px;">
                "sign up and complete your profile (fields with * are required)"
                <label>"Email*" <input type="email" name="surgeon[email]" required /></label>
                <label>"First Name" <input type="text" name="surgeon[first_name]" /></label>
                <label>"Last Name" <input type="text" name="surgeon[last_name]" /></label>
                // todo: we need a datalist that populates a new default::Site if their site
                // isn't already present.
                <label>
                    "Default Hospital/Site" <input type="text" name="surgeon[default_site]" />
                </label>
                <label>
                    "SIA power for right eyes (D)*"
                    <input
                        type="number"
                        min=0
                        max=2
                        step=0.05
                        name="surgeon[sia_right_power]"
                        required
                    />
                </label>
                <label>
                    "SIA axis for right eyes (°)*"
                    <input
                        type="number"
                        min=0
                        max=179
                        step=1
                        name="surgeon[sia_right_axis]"
                        required
                    />
                </label>
                <label>
                    "SIA power for left eyes (D)*"
                    <input
                        type="number"
                        min=0
                        max=2
                        step=0.05
                        name="surgeon[sia_left_power]"
                        required
                    />
                </label>
                <label>
                    "SIA axis for left eyes (°)*"
                    <input
                        type="number"
                        min=0
                        max=179
                        step=1
                        name="surgeon[sia_left_axis]"
                        required
                    />
                </label> <input type="submit" value="Sign up" />
            </div>
        </ActionForm>
    }
}
