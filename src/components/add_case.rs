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
                        // todo: we need a signal holding this value to update the Sia
                        "Right" <input type="radio" value="right" name="case[side]" required />
                    </label>
                    <label>
                        "Left"<input type="radio" value="left" name="case[side]" required />
                    </label>
                </fieldset>
                <label>
                    "AL (12–38 mm)"
                    <input type="number" min=12 max=38 step=0.01 name="case[al]" required />
                </label>
                <label>
                    "K1 power (30–65 D)"
                    <input type="number" min=30 max=65 step=0.01 name="case[k1_power]" required />
                </label>
                <label>
                    "K1 axis (0–179°)"
                    <input type="number" min=0 max=179 step=1 name="case[k1_axis]" required />
                </label>
                <label>
                    "K2 power (30–65 D)"
                    <input type="number" min=30 max=65 step=0.01 name="case[k2_power]" required />
                </label>
                <label>
                    "K2 axis (0–179°)"
                    <input type="number" min=0 max=179 step=1 name="case[k2_axis]" required />
                </label>
                <label>
                    "ACD (0–6 mm)"
                    <input type="number" min=0 max=6 step=0.01 name="case[acd]" required />
                </label>
                <label>
                    "LT (2–8 mm)"
                    <input type="number" min=2 max=8 step=0.01 name="case[lt]" required />
                </label>
                <label>
                    "CCT (350–650 µm, optional)"
                    <input type="number" min=350 max=650 step=1 name="case[cct]" />
                </label>
                <label>
                    "WTW (8–14 mm, optional)"
                    <input type="number" min=8 max=14 step=0.01 name="case[wtw]" />
                </label>
                <label>
                    // todo: populate the available formulas from the DB
                    // todo: prefill the surgeon's default formula
                    "Formula" <input list="formulas" value="Kane" name="case[formula]" required />
                    <datalist id="formulas">
                        <option value="barrett">"Barrett"</option>
                        <option value="kane">"Kane"</option>
                    </datalist>
                </label>
                <label>
                    "Check here if you use a custom/optimized IOL constant with this formula"
                    <input type="checkbox" name="case[custom_constant]" value="true" />
                </label>
                <label>
                    "Target spherical equivalent (-6–2 D)"
                    <input type="number" min=-6 max=2 step=0.01 name="case[target_se]" required />
                </label>
                <label>
                    "Target cylinder power (0–6 D, target cyl is optional but strongly encouraged)"
                    <input type="number" min=0 max=6 step=0.01 name="case[target_cyl_power]" />
                </label>
                <label>
                    "Target cylinder axis (0–179°)"
                    <input type="number" min=0 max=179 step=1 name="case[target_cyl_axis]" />
                </label>
                <label>
                    // interop
                    "Date of surgery" <input type="date" name="case[date]" required />
                </label>
                <label>"Hospital/Site (optional)" <input type="text" name="case[site]" /></label>
                <label>
                    "Main incision size (1–6 mm)"
                    <input type="number" min=1 max=6 step=0.05 name="case[main]" required />
                </label>
                <label>
                    "SIA power (D)"
                    <input type="number" min=0 max=2 step=0.01 name="case[sia_power]" required />
                </label>
                <label>
                    "SIA axis (°)"
                    <input type="number" min=0 max=179 step=1 name="case[sia_axis]" required />
                </label>
                <label>
                    // todo: prefill the surgeon's default IOL
                    "IOL model"<input list="iols" name="case[iol]" required /><datalist id="iols">
                        <option value="sn60wf">"SN60WF (Alcon)"</option>
                        <option value="diuxxx">"DIUxxx, DIB00 (J&J)"</option>
                    </datalist>
                </label>
                <label>
                    "IOL spherical equivalent (-20–60 D)"
                    <input type="number" min=-20 max=60 step=0.25 name="case[iol_se]" required />
                </label>
                <label>
                    // todo: hide this field using a signal if the model is nontoric
                    "IOL axis (0–179°)"
                    <input type="number" min=0 max=179 step=1 name="case[iol_axis]" />
                </label>> <fieldset>
                    <legend>"Adverse event"</legend>
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
                                    min=0
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
                                    min=0
                                    max=20
                                    step=1
                                    name="case[va_best_before_num]"
                                    required
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input
                                    type="number"
                                    min=1
                                    step=0.1
                                    name="case[va_best_before_den]"
                                    required
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
                                    min=0
                                    max=20
                                    step=1
                                    name="case[va_raw_after_num]"
                                    required
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input
                                    type="number"
                                    min=1
                                    step=0.1
                                    name="case[va_raw_after_den]"
                                    required
                                />
                            </label>
                        </div>
                        <div>
                            "Best corrected (optional)"
                            <label>
                                "Numerator"
                                <input
                                    type="number"
                                    min=0
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
                                required
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
                                required
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

// bookmark: todo: modify this to insert a case
#[server]
pub async fn insert_case(case: FormCase) -> Result<(), ServerFnError> {
    // first call into_surgeon_case(case)
    // then pattern match all of the values in the SurgeonCase and assign to vars
    // then insert the query unless conflict on urn/side for that surgeon specifically
    Ok(())
}
