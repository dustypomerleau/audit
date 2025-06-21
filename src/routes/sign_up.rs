#[cfg(feature = "ssr")] use crate::db::{db, to_centi};
use crate::model::{Email, FormSurgeon, Surgeon, set_current_surgeon};
use leptos::{
    prelude::{
        ActionForm, ElementChild, IntoMaybeErased, IntoView, ServerAction, ServerFnError,
        StyleAttribute, component, view,
    },
    server,
};
#[cfg(feature = "ssr")] use leptos_axum::redirect;

#[component]
pub fn SignUp() -> impl IntoView {
    // ServerAction::<S>::new() creates an action that will call the server function S
    // let insert_surgeon = ServerAction::<InsertSurgeon>::new();
    // let returned_value = insert_surgeon.value();
    // let is_error = move || returned_value.with(|val| matches!(val, Some(err)));
    let insert_surgeon = ServerAction::<InsertSurgeon>::new();

    view! {
        <ActionForm action=insert_surgeon>
            <div style="display: grid; grid-auto-columns: 1fr; grid-gap: 30px;">
                "Sign up and complete your profile (fields with * are required). The values you give here will be used as defaults, but you can override them for an individual surgical case."
                <label>"Email*" <input type="email" name="surgeon[email]" required /></label>
                <label>"First Name" <input type="text" name="surgeon[first_name]" /></label>
                <label>"Last Name" <input type="text" name="surgeon[last_name]" /></label>
                // todo: populate this from the DB, and add a new site in the query if needed
                <label>
                    "Default Hospital/Site" <input list="sites" name="surgeon[default_site]" />
                    <datalist>
                        <option value="Royal Melbourne Hospital (Melbourne, AUS)"></option>
                    </datalist>
                </label>
                <label>
                    // todo: populate this from the DB with all IOLs
                    "Default IOL" <input list="iols" name="surgeon[default_iol]" /> <datalist>
                        <option label="SN60WF" value="sn60wf"></option>
                        <option label="DETxxx" value="detxxx"></option>
                    </datalist>
                </label>
                <label>
                    // todo: populate this from the DB with all formulas
                    "Default formula" <input list="formulas" name="surgeon[default_formula]" />
                    <datalist>
                        <option label="Barrett" value="barrett"></option>
                        <option label="Kane" value="kane"></option>
                    </datalist>
                </label>
                <label>
                    "Check here if you use a custom or optimized constant with your default formula"
                    <input type="checkbox" value="true" name="surgeon[custom_constant]" />
                </label>
                <label>
                    "Main incision size (mm)*"
                    <input type="number" min=1 max=6 step=0.05 name="surgeon[main]" required />
                </label>
                <label>
                    "SIA power (D)*"
                    <input type="number" min=0 max=2 step=0.05 name="surgeon[sia_power]" required />
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
        default_iol,
        default_formula,
        custom_constant,
        main,
        sia_power,
        sia_right_axis,
        sia_left_axis,
    } = surgeon;

    let email = Email::new(&email)?.inner();

    some_or_empty!(
        first_name,
        last_name,
        default_site,
        default_iol,
        default_formula
    );

    fn to_db_formula(formula: &str) -> &str {
        match formula.to_lowercase().as_str() {
            "ascrskrs" => "Formula.AscrsKrs",
            "barrett" | "barretttoric" => "Formula.Barrett",
            "barretttruek" => "Formula.BarrettTrueK",
            "evo" => "Formula.Evo",
            "haigis" => "Formula.Haigis",
            "haigisl" => "Formula.HaigisL",
            "hillrbf" => "Formula.HillRbf",
            "hofferq" => "Formula.HofferQ",
            "holladay1" => "Formula.Holladay1",
            "holladay2" => "Formula.Holladay2",
            "kane" => "Formula.Kane",
            "okulix" => "Formula.Okulix",
            "olsen" => "Formula.Olsen",
            "srkt" => "Formula.SrkT",
            _ => "Formula.Other",
        }
    }

    let default_formula = to_db_formula(&default_formula);
    let custom_constant = custom_constant.is_some_and(|value| value.as_str() == "true");
    let main = to_centi(main);
    let sia_power = to_centi(sia_power);

    let query = format!(
        r#"
with QuerySurgeon := (insert Surgeon {{
        identity := (select global ext::auth::ClientTokenIdentity),
        email := "{email}",
        first_name := {first_name},
        last_name := {last_name},

        defaults := (select (insert SurgeonDefaults {{
            site := (select(insert Site {{
                name := {default_site}
            }} unless conflict on .name else (select Site))),

            iol := (select Iol filter .model = {default_iol}),
            formula := {default_formula},
            custom_constant := {custom_constant},
            main := {main}
        }})),

        sia := (select(insert SurgeonSia {{
            right := (select(insert Sia {{
                power := {sia_power}, axis := {sia_right_axis}
            }})),

            left := (select(insert Sia {{
                power := {sia_power}, axis := {sia_left_axis}
            }}))
        }}))
    }} unless conflict on .email else (select Surgeon))

select QuerySurgeon {{
    email,
    terms,
    first_name,
    last_name,

    defaults: {{
        site: {{ name }},
        iol: {{ model, name, company, focus, toric }},
        formula,
        custom_constant,
        main
    }},

    sia: {{ right: {{ power, axis }}, left: {{ power, axis }} }}
}};
        "#
    );

    if let Ok(Some(surgeon_json)) = db().await?.query_single_json(query, &()).await {
        let surgeon = serde_json::from_str::<Surgeon>(surgeon_json.as_ref())?;
        set_current_surgeon(Some(surgeon)).await?;
        redirect("/terms");
    } else {
        // if we fail on the insert, then:
        //
        // 1. something is wrong with the form validation
        // 2. the user already exists (email conflict) - with the current query that will still
        //    return a surgeon, but it will be the one that already existed in the DB
        // 3. the user navigated directly to the signup page without first signing in (in this case,
        //    there would be no `ext::auth::ClientTokenIdentity`)
        //
        // We'll have to figure out a way to surface those errors, but for now just prompt the user
        // to restart the flow.
        redirect("/signedout");
    }

    Ok(())
}

// todo: run testing and check all permutations of login antics:
//
// A full list of possible scenarios:
//
// 1. The user is new, they click new user and sign up, accepting the terms
// - create the user as you currently do
//
// 2. The user is new, they click existing user and try to sign in
// - redirect to the sign up form and follow the usual new user flow through terms from there
//
// 3. The user is existing, they click existing and try to sign in
// - proceed to the add case form as you currently do
//
// 4. The user is existing, they click new user and try to sign up
//
//   a. the email matches the email they used last time:
//   - just ignore them, and redirect to the same flow as the existing users, checking terms, then
//     on to add case.
//
//   b. the email doesn't match, but you don't want to update the user without informing them that
//   they already have an account with a different email:
//   - todo: add logic for this
//
// 5. The user is new, they click new user and try to sign up with an email that is already being
//    used by a different user (most likely scenario is that this is actually the same user, but
//    they have multiple Google accounts).
//    - currently this results in a redirect to `signed/out` and does not create the user, which is
//    harmless, but not very informative
//    - similar to 4b, todo: add specific logic for this
