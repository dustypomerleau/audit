use crate::case::FormCase;
#[cfg(feature = "ssr")]
use crate::{
    db::{db, some_or_empty, to_centi},
    surgeon::set_current_surgeon,
};
use leptos::prelude::{
    ActionForm, ElementChild, IntoView, ServerAction, ServerFnError, StyleAttribute, component,
    server, view,
};
#[cfg(feature = "ssr")] use leptos_axum::redirect;

#[component]
pub fn AddCase() -> impl IntoView {
    // todo: load necessary datalists form the DB:
    // 1. sites
    // 2. IOL models and default constants
    // 3. surgeon defaults (should already be in context)
    //
    let insert_case = ServerAction::<InsertCase>::new();

    // todo: IOL constant value should be calculated as follows:
    // - surgeon value if IOL matches Surgeon::default_constant or Surgeon::constants
    // - DB default value if any other IOL
    view! {
        <ActionForm action=insert_case>
            <div style="display: grid; grid-auto-columns: 1fr; grid-gap: 30px;">
                "Enter the case details (fields are required unless marked optional)"
                // todo: "To change your default values, please [update your profile](link)"
                <label>
                    "Unique (anonymised) identifier" <input type="text" name="case[urn]" required />
                </label> <fieldset>
                    <legend>"Side"</legend>
                    <label>
                        "Right"
                        <input type="radio" value="right" name="case[side]" required checked />
                    </label>
                    <label>
                        "Left"<input type="radio" value="left" name="case[side]" required />
                    </label>
                </fieldset>
                <label>
                    // todo: prefill the surgeon's default formula
                    // todo: populate the available formulas from the DB
                    "Formula"
                    <input list="formulas" value="Kane" name="case[target_constant_formula]" />
                    <datalist id="formulas">
                        <option value="barrett">"Barrett"</option>
                        <option value="kane">"Kane"</option>
                    </datalist>
                </label>
                <label>
                    "IOL constant"
                    <input
                        type="number"
                        min=0
                        max=130
                        step=0.01
                        name="case[target_constant_value]"
                    />
                </label>
                <label>
                    "Target spherical equivalent (D)"
                    <input
                        type="number"
                        min=-6
                        max=2
                        step=0.01
                        value=0
                        name="case[target_se]"
                        required
                    />
                </label>
                <label>
                    "Target cylinder power (D)"
                    <input
                        type="number"
                        min=0
                        max=6
                        step=0.01
                        value=0
                        name="case[target_cyl_power]"
                        required
                    />
                </label>
                <label>
                    "Target cylinder axis (°)"
                    <input
                        type="number"
                        min=0
                        max=179
                        step=1
                        value=0
                        name="case[target_cyl_axis]"
                        required
                    />
                </label> <label>"Date of surgery" <input type="date" name="case[date]" /></label>
                <label>"Site" <input type="text" name="case[site]" /></label>
                <label>
                    "SIA power (D)"
                    <input type="number" min=0 max=2 step=0.01 name="case[sia_power]" />
                </label>
                <label>
                    "SIA axis (°)"
                    <input type="number" min=0 max=179 step=1 name="case[sia_axis]" />
                </label>
                <label>
                    // todo: prefill the surgeon's default IOL
                    "IOL model"<input list="iols" name="case[iol]" /><datalist id="iols">
                        <option value="SN60WF (Alcon)"></option>
                        <option value="DIUxxx, DIB00 (J&J)"></option>
                    </datalist>
                </label> <fieldset>
                    <legend>"Adverse event"</legend>
                    // a label to show
                    <input type="radio" value="none" name="case[adverse]" required checked />
                    <input type="radio" value="rhexis" name="case[adverse]" required />
                    <input type="radio" value="pc" name="case[adverse]" required />
                    <input type="radio" value="zonule" name="case[adverse]" required />
                    <input type="radio" value="other" name="case[adverse]" required />
                </fieldset>
                <div>
                    "Visual acuity"
                    <div>
                        "Preop"
                        <div>
                            "Uncorrected (optional)"
                            <label>
                                "Numerator"
                                <input
                                    type="number"
                                    min=1
                                    max=20
                                    step=1
                                    name="case[va_raw_before_num]"
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input
                                    type="number"
                                    min=1
                                    step=0.1
                                    name="case[va_raw_before_den]"
                                />
                            </label>
                        </div>
                        <div>
                            "Best corrected"
                            <label>
                                "Numerator"
                                <input
                                    type="number"
                                    min=1
                                    max=20
                                    step=1
                                    name="case[va_best_before_num]"
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input
                                    type="number"
                                    min=1
                                    step=0.1
                                    name="case[va_best_before_den]"
                                />
                            </label>
                        </div>
                    </div>
                    <div>
                        "Postop"
                        <div>
                            "Uncorrected"
                            <label>
                                "Numerator"
                                <input
                                    type="number"
                                    min=1
                                    max=20
                                    step=1
                                    name="case[va_raw_after_num]"
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input type="number" min=1 step=0.1 name="case[va_raw_after_den]" />
                            </label>
                        </div>
                        <div>
                            "Best corrected (optional)"
                            <label>
                                "Numerator"
                                <input
                                    type="number"
                                    min=1
                                    max=20
                                    step=1
                                    name="case[va_best_after_num]"
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input
                                    type="number"
                                    min=1
                                    step=0.1
                                    name="case[va_best_after_den]"
                                />
                            </label>
                        </div>
                    </div>
                </div>
                <div>
                    "Refraction"
                    <div>
                        "Preop"
                        <label>
                            "Sphere (D)"
                            <input
                                type="number"
                                min=-20
                                max=20
                                step=0.25
                                name="case[ref_before_sph]"
                            />
                        </label>
                        <label>
                            "Cylinder power (D)"
                            <input
                                type="number"
                                min=-10
                                max=10
                                step=0.25
                                name="case[ref_before_cyl_power]"
                            />
                        </label>
                        <label>
                            "Cylinder axis (°)"
                            <input
                                type="number"
                                min=0
                                max=179
                                step=1
                                name="case[ref_before_cyl_axis]"
                            />
                        </label>
                    </div>
                    <div>
                        "Postop"
                        <label>
                            "Sphere (D)"
                            <input
                                type="number"
                                min=-20
                                max=20
                                step=0.25
                                name="case[ref_after_sph]"
                            />
                        </label>
                        <label>
                            "Cylinder power (D)"
                            <input
                                type="number"
                                min=-10
                                max=10
                                step=0.25
                                name="case[ref_after_cyl_power]"
                            />
                        </label>
                        <label>
                            "Cylinder axis (°)"
                            <input
                                type="number"
                                min=0
                                max=179
                                step=1
                                name="case[ref_after_cyl_axis]"
                            />
                        </label>
                    </div>
                </div> <input type="submit" value="Submit case" />
            </div>
        </ActionForm>
    }
}

// todo: modify this to insert a case
#[server]
pub async fn insert_case(case: FormCase) -> Result<(), ServerFnError> {
    // let FormSurgeon {
    //     email,
    //     first_name,
    //     last_name,
    //     default_site,
    //     sia_right_power,
    //     sia_right_axis,
    //     sia_left_power,
    //     sia_left_axis,
    // } = surgeon;
    //
    // let email = Email::new(&email)?.inner();
    //
    // let (first_name, last_name, default_site) = (
    //     some_or_empty(first_name),
    //     some_or_empty(last_name),
    //     some_or_empty(default_site),
    // );
    //
    // let (sia_right_power, sia_left_power) = (to_cd(sia_right_power), to_cd(sia_left_power));

    //     let query = format!(
    //         r#"
    // with QuerySurgeon := (
    //     insert Surgeon {{
    //         identity := (select global ext::auth::ClientTokenIdentity),
    //         email := "{email}",
    //         first_name := {first_name},
    //         last_name := {last_name},
    //
    //         default_site := (select(insert Site {{
    //             name := {default_site}
    //         }} unless conflict on .name else (select Site))),
    //
    //         sia := (select(insert SurgeonSia {{
    //             right := (select(insert Sia {{
    //                 power := {sia_right_power}, axis := {sia_right_axis}
    //             }})),
    //             left := (select(insert Sia {{
    //                 power := {sia_left_power}, axis := {sia_left_axis}
    //             }}))
    //         }}))
    //     }} unless conflict on .email else (select Surgeon)
    // )
    // select QuerySurgeon {{
    //     email,
    //     terms,
    //     first_name,
    //     last_name,
    //     default_site: {{ name }},
    //     sia: {{
    //         right: {{ power, axis }},
    //         left: {{ power, axis }}
    //     }}
    // }};
    //         "#
    //     );
    //
    //     if let Ok(Some(surgeon)) = db().await?.query_single::<Surgeon, _>(query, &()).await {
    //         set_current_surgeon(Some(surgeon)).await?;
    //         redirect("/terms");
    //     } else {
    //         // if we fail on the insert, then:
    //         // 1. something is wrong with the form validation
    //         // 2. the user already exists (email conflict) - with the current query that will
    // still         //    return a surgeon, but it will be the one that already existed in the
    // DB         // 3. the user navigated directly to the signup page without first signing in
    // (in this case,         //    there would be no `ext::auth::ClientTokenIdentity`)
    //         //
    //         // We'll have to figure out a way to surface those errors, but for now just restart
    // the         // flow. It probably would help to redirect to a simple page that says
    // "please sign in to         // continue," so that it's clear this isn't the same as the
    // landing page.         redirect("/");
    //     }

    Ok(())
}
