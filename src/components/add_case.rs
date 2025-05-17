use crate::{
    biometry::{Biometry, K, Ks},
    bounded::Bounded,
    case::{Case, FormCase, Side, SurgeonCase},
    iol::{Iol, OpIol},
    refraction::{OpRefraction, Refraction},
    sia::Sia,
    surgeon::{Site, Surgeon, get_current_surgeon},
    target::Target,
    va::{AfterVa, BeforeVa, OpVa, Va},
};
use leptos::{
    prelude::{
        ActionForm, ElementChild, IntoView, ServerAction, ServerFnError, StyleAttribute, component,
        server, view,
    },
    server_fn::error::NoCustomError,
};

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
                    "Unique (anonymised) identifier"
                    <input type="text" name="case[urn]" required autofocus />
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
                    // todo: prefill the surgeon's default formula
                    "Formula" <select name="case[formula]">
                        <optgroup label="Thick lens formulas">
                            <option value="barrett">"Barrett"</option>
                            <option value="evo">"EVO"</option>
                            <option value="hillrbf">"Hill RBF"</option>
                            <option value="holladay2">"Holladay 2"</option>
                            <option value="kane" selected>
                                "Kane"
                            </option>
                            <option value="okulix">"Okulix raytracing"</option>
                            <option value="olsen">"Olsen"</option>
                        </optgroup>
                        <optgroup label="Thin lens formulas">
                            <option value="haigis">"Haigis"</option>
                            <option value="hofferq">"Hoffer Q"</option>
                            <option value="holladay1">"Holladay 1"</option>
                            <option value="srkt">"SRK/T"</option>
                        </optgroup>
                        <optgroup label="Post-refractive formulas">
                            <option value="ascrskrs">"ASCRS"</option>
                            <option value="barretttruek">"Barrett True K"</option>
                            <option value="haigisl">"Haigis-L"</option>
                        </optgroup>
                        <optgroup label="Other">
                            <option value="other">"Not listed"</option>
                        </optgroup>
                    </select>
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
                    <label>
                        "None"
                        <input type="radio" value="none" name="case[adverse]" required checked />
                    </label>
                    <label>
                        "Rhexis"<input type="radio" value="rhexis" name="case[adverse]" required />
                    </label>
                    <label>
                        "PC"<input type="radio" value="pc" name="case[adverse]" required />
                    </label>
                    <label>
                        "Zonule"<input type="radio" value="zonule" name="case[adverse]" required />
                    </label>
                    <label>
                        "Other"<input type="radio" value="other" name="case[adverse]" required />
                    </label>
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

#[server]
pub async fn insert_case(case: FormCase) -> Result<(), ServerFnError> {
    // bookmark:
    // - [x] call into_surgeon_case(case)
    // - [x] pattern match all of the values in the SurgeonCase and assign to vars
    // - [ ] get inner values, handle options with some_or_empty! you may need to rewrite this for
    // newtypes
    // - [ ] write the query to insert unless conflict on urn/side for that surgeon specifically

    let SurgeonCase {
        urn,
        date,
        site,
        case:
            Case {
                side,
                biometry:
                    Biometry {
                        al,
                        ks,
                        acd,
                        lt,
                        cct,
                        wtw,
                    },
                target:
                    Target {
                        formula,
                        custom_constant,
                        se: target_se,
                        cyl: target_cyl,
                    },
                main,
                sia:
                    Sia {
                        power: sia_power,
                        axis: sia_axis,
                    },
                iol:
                    OpIol {
                        iol:
                            Iol {
                                model,
                                name,
                                company,
                                focus,
                                toric,
                            },
                        se: iol_se,
                        axis: iol_axis,
                    },
                adverse,
                va:
                    OpVa {
                        before:
                            BeforeVa {
                                best:
                                    Va {
                                        num: va_best_before_num,
                                        den: va_best_before_den,
                                    },
                                raw: va_raw_before,
                            },
                        after:
                            AfterVa {
                                best: va_best_after,
                                raw:
                                    Va {
                                        num: va_raw_after_num,
                                        den: va_raw_after_den,
                                    },
                            },
                    },
                refraction:
                    OpRefraction {
                        before:
                            Refraction {
                                sph: ref_before_sph,
                                cyl: ref_before_cyl,
                            },
                        after:
                            Refraction {
                                sph: ref_after_sph,
                                cyl: ref_after_cyl,
                            },
                    },
            },
    } = case.into_surgeon_case().await?;

    let date = date.to_string();
    let site_name = site.map(|Site { name }| name).unwrap_or("{}".to_string());

    let side = match side {
        Side::Right => "Side.Right",
        Side::Left => "Side.Left",
    };

    // note: We don't need to cast the integer types, because they are just going into a format
    // string. The <int32> will be inferred based on the object field types in Gel.
    let (al, flat_k_power, flat_k_axis, steep_k_power, steep_k_axis, acd, lt, cct, wtw) = (
        al.inner(),
        ks.flat_power(),
        ks.flat_axis(),
        ks.steep_power(),
        ks.steep_axis(),
        acd.inner(),
        lt.inner(),
        cct.map(|cct| cct.inner().to_string())
            .unwrap_or("{}".to_string()),
        wtw.map(|wtw| wtw.inner().to_string())
            .unwrap_or("{}".to_string()),
    );

    let query = format!(
        "r#
with 
QueryBiometry := (insert Biometry {{
    al := {al},
    ks := (select(insert Ks {{
        flat := (select(insert K {{ power := {flat_k_power}, axis := {flat_k_axis} }})),
        steep := (select(insert K {{ power := {steep_k_power}, axis := {steep_k_axis} }})),
    }})),
    acd := {acd},
    lt := {lt},
    cct := {cct},
    wtw := {wtw}
}}), 
QueryCas := (insert Cas {{
    side := {side},
    biometry := (select QueryBiometry),
    target :=
    year :=
    main :=
    sia :=
    iol :=
    adverse :=
    va :=
    refraction :=
}}),
QuerySurgeonCas := (insert SurgeonCas {{
    surgeon := (select global cur_surgeon), 
    urn := {urn},
    date := {date},
    site := (select(insert Site {{ 
        name := {site_name} 
    }} unless conflict on .name else (select Site))),
    cas := (select QueryCas)
}})
select...choose whatever fields you need to display the item in the list
        #"
    );
    // note: not sure how we enforce that the insert can't have a conflict on (.surgeon, .urn,
    // .case.side) because of the extra hop
    // I think what you might need to do is to duplicate the side in both the SurgeonCas and the
    // Cas, and then have an exclusive constraint on (.surgeon, .urn, .side) in the SurgeonCas and
    // use a generic `unless conflict` on the SurgeonCas insert, without any conditions.
    Ok(())
}
