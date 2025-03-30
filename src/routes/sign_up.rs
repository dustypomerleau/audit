use crate::surgeon::FormSurgeon;
#[cfg(feature = "ssr")]
use crate::{
    db::{db, some_or_empty, to_cd},
    surgeon::set_current_surgeon,
    surgeon::{Email, Surgeon},
};
use leptos::prelude::{
    ActionForm, ElementChild, IntoView, ServerAction, ServerFnError, StyleAttribute, component,
    server, view,
};
#[cfg(feature = "ssr")] use leptos_axum::redirect;

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

#[server]
pub async fn insert_surgeon(surgeon: FormSurgeon) -> Result<(), ServerFnError> {
    let FormSurgeon {
        email,
        first_name,
        last_name,
        default_site,
        sia_right_power,
        sia_right_axis,
        sia_left_power,
        sia_left_axis,
    } = surgeon;

    let email = Email::new(&email)?.inner();

    let (first_name, last_name, default_site) = (
        some_or_empty(first_name),
        some_or_empty(last_name),
        some_or_empty(default_site),
    );

    let (sia_right_power, sia_left_power) = (to_cd(sia_right_power), to_cd(sia_left_power));

    let query = format!(
        r#"
with QuerySurgeon := (
    insert Surgeon {{
        identity := (select global ext::auth::ClientTokenIdentity),
        email := "{email}",
        first_name := {first_name},
        last_name := {last_name},

        default_site := (select(insert Site {{
            name := {default_site} 
        }} unless conflict on .name else (select Site))),

        sia := (select(insert SurgeonSia {{
            right := (select(insert Sia {{
                power := {sia_right_power}, axis := {sia_right_axis}
            }})),
            left := (select(insert Sia {{
                power := {sia_left_power}, axis := {sia_left_axis}
            }}))
        }}))
    }} unless conflict on .email else (select Surgeon)
)
select QuerySurgeon {{
    email,
    terms,
    first_name,
    last_name,
    default_site: {{ name }},
    sia: {{
        right: {{ power, axis }},
        left: {{ power, axis }}
    }}
}};
        "#
    );

    // We use `query_required_single` in this case, because failure to return a Surgeon means our
    // insert failed.
    if let Ok(surgeon) = db()
        .await?
        .query_required_single::<Surgeon, _>(query, &())
        .await
    {
        set_current_surgeon(Some(surgeon)).await?;
        redirect("/terms");
    } else {
        // if we fail on the insert, then:
        // 1. something is wrong with the form validation
        // 2. the user already exists (email conflict) - with the current query that will still
        //    return a surgeon, but it will be the one that already existed in the DB
        // 3. the user navigated directly to the signup page without first signing in (in this case,
        //    there would be no `ext::auth::ClientTokenIdentity`)
        //
        // We'll have to figure out a way to surface those errors, but for now just restart the
        // flow. It probably would help to redirect to a simple page that says "please sign in to
        // continue," so that it's clear this isn't the same as the landing page.
        redirect("/");
    }

    Ok(())
}
