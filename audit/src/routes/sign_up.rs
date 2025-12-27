use leptos::prelude::ActionForm;
use leptos::prelude::ElementChild;
use leptos::prelude::IntoView;
use leptos::prelude::ServerAction;
use leptos::prelude::ServerFnError;
use leptos::prelude::StyleAttribute;
use leptos::prelude::component;
use leptos::prelude::server;
use leptos::prelude::view;
#[cfg(feature = "ssr")] use leptos_axum::redirect;
#[cfg(feature = "ssr")] use sqlx::query_as;

use crate::bounded::Bounded;
#[cfg(feature = "ssr")] use crate::db::db;
use crate::model::Axis;
#[cfg(feature = "ssr")] use crate::model::Email;
use crate::model::Focus;
use crate::model::FormSurgeon;
use crate::model::Formula;
use crate::model::Main;
use crate::model::QuerySurgeon;
use crate::model::SiaPower;
#[cfg(feature = "ssr")] use crate::model::Surgeon;
use crate::model::ToricPower;
#[cfg(feature = "ssr")] use crate::model::set_current_surgeon;

#[component]
pub fn SignUp() -> impl IntoView {
    let insert_surgeon = ServerAction::<InsertSurgeon>::new();

    view! {
        <ActionForm action=insert_surgeon>
            <div style="display: grid; grid-auto-columns: 1fr; grid-gap: 30px;">
                "Sign up and complete your profile (fields with * are required). The values you give here will be used as defaults, but you can override them for an individual surgical case."
                <label>"Email*" <input type="email" name="surgeon[email]" required /></label>
                <label>"Full Name" <input type="text" name="surgeon[full_name]" /></label>
                <label>"Preferred Name: What should we call you?" <input type="text" name="surgeon[preferred_name]" /></label>
                // TODO: populate this from the DB, and add a new site in the query if needed
                <label>
                    "Default Hospital/Site" <input list="sites" name="surgeon[default_site]" />
                    <datalist>
                        <option value="Royal Melbourne Hospital (Melbourne, AUS)"></option>
                    </datalist>
                </label>
                <label>
                    // TODO: populate this from the DB with all IOLs
                    "Default IOL" <input list="iols" name="surgeon[default_iol]" /> <datalist>
                        <option label="SN60WF" value="sn60wf"></option>
                        <option label="DETxxx" value="detxxx"></option>
                    </datalist>
                </label>
                <label>
                    // TODO: populate this from the DB with all formulas
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
                    "SIA power (D)* (power and axis can be overridden per-case)"
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
pub async fn insert_surgeon(surgeon: FormSurgeon) -> Result<Option<Surgeon>, ServerFnError> {
    let FormSurgeon {
        email,
        full_name,
        preferred_name,
        default_site,
        default_iol,
        default_formula,
        custom_constant,
        main,
        sia_power,
        sia_right_axis,
        sia_left_axis,
    } = surgeon;

    let email = Email::new(&email)?;

    let formula = if let Some(formula) = default_formula {
        Some(serde_json::from_str::<Formula>(formula.as_str())?)
    } else {
        None
    };

    let custom_constant = custom_constant.is_some_and(|value| value.as_str() == "true");
    let main = Main::new((main * 100.0) as i32)?;
    let sia_power = SiaPower::new((sia_power * 100.0) as i32)?;
    let (sia_right_axis, sia_left_axis) = (Axis::new(sia_right_axis)?, Axis::new(sia_left_axis)?);

    let query_surgeon_result = query_as!(
        QuerySurgeon,
        r#"
with s as (
    insert into surgeon (
        email,
        full_name,
        preferred_name,
        default_site_id,
        default_iol_id,
        default_formula,
        default_custom_constant,
        default_main,
        default_sia_power,
        default_sia_axis_right,
        default_sia_axis_left
    )
    values (
        $1, $2, $3,
        (select id from site where name = $4),
        (select id from iol where model = $5),
        $6, $7, $8, $9, $10, $11
    )
    returning
        email,
        terms,
        full_name,
        preferred_name,
        default_site_id,
        default_iol_id,
        default_formula,
        default_custom_constant,
        default_main,
        default_sia_power,
        default_sia_axis_right,
        default_sia_axis_left
)

select
    s.email as "email: Email",
    s.terms,
    s.full_name,
    s.preferred_name,

    site.name as default_site_name,

    iol.model as default_iol_model,
    iol.name as default_iol_name,
    iol.company as default_iol_company,
    iol.focus as "default_iol_focus: Focus",
    iol.toric as "default_iol_toric: ToricPower",

    s.default_formula as "default_formula: Formula",
    s.default_custom_constant,
    s.default_main as "default_main: Main",

    s.default_sia_power as "default_sia_power: SiaPower",
    s.default_sia_axis_right as "default_sia_axis_right: Axis",
    s.default_sia_axis_left as "default_sia_axis_left: Axis"

from s
join site on s.default_site_id = site.id
join iol on s.default_iol_id = iol.id;
        "#,
        email as Email,
        full_name,
        preferred_name,
        default_site,
        default_iol,
        formula as Option<Formula>,
        custom_constant,
        main as Main,
        sia_power as SiaPower,
        sia_right_axis as Axis,
        sia_left_axis as Axis,
    )
    .fetch_one(&db().await?)
    .await;
    dbg!(&query_surgeon_result);

    if let Ok(query_surgeon) = query_surgeon_result {
        let surgeon: Surgeon = query_surgeon.into();
        set_current_surgeon(Some(surgeon.clone())).await?;

        if surgeon.terms.is_none() {
            redirect("/terms");
        }

        Ok(Some(surgeon))
    } else {
        // if we fail on the insert, then:
        //
        // 1. something is wrong with the form validation
        // 2. the user already exists (email conflict). TODO: unlike our Gel implementation, this
        //    one will error, so we need to rewrite the query to return the surgeon anyway, like we
        //    used to
        // 3. the user navigated directly to the signup page without first signing in (in this case,
        //    there would be no `ext::auth::ClientTokenIdentity`) We'll have to figure out a way to
        //    surface those errors, but for now just prompt the user to restart the flow.
        redirect("/signedout");

        Ok(None)
    }
}

// TODO: run testing and check all permutations of login antics:
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
//   - TODO: add logic for this
//
// 5. The user is new, they click new user and try to sign up with an email that is already being
//    used by a different user (most likely scenario is that this is actually the same user, but
//    they have multiple Google accounts).
//    - currently this results in a redirect to `signed/out` and does not create the user, which is
//    harmless, but not very informative
//    - similar to 4b, TODO: add specific logic for this
