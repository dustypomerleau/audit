#[cfg(feature = "ssr")] use crate::db::db;
use crate::{
    bounded::Bounded,
    error::AppError,
    model::{
        Adverse, AfterVa, BeforeVa, Biometry, Case, FormCase, Formula, Iol, OpIol, OpRefraction,
        OpVa, RefCyl, Refraction, Sia, Side, Site, SurgeonCase, Target, TargetCyl, Va,
    },
};
use chrono::Datelike;
use leptos::{
    prelude::{
        ActionForm, ElementChild, For, Get, GlobalAttributes, IntoView, ServerAction,
        StyleAttribute, Suspense, component, server, view,
    },
    server::OnceResource,
};

#[component]
pub fn AddCase() -> impl IntoView {
    // todo: load necessary datalists form the DB:
    // 1. sites
    // 2. IOL models
    // 3. surgeon defaults (should already be in context)
    //
    let insert_case = ServerAction::<InsertFormCase>::new();
    let iol_resource = OnceResource::new(get_iols());

    let iols = move || {
        iol_resource
            .get()
            .map(|res| res.unwrap_or_default())
            .unwrap_or_default()
    };

    // todo: IOL constant value should be calculated as follows:
    // - surgeon value if IOL matches Surgeon::default_constant or Surgeon::constants
    // - DB default value if any other IOL
    // todo: autofill second K axis
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
                    // todo: we need a signal holding this value to update the Sia
                    <label>
                        "Right"<input type="radio" value="Right" name="case[side]" required />
                    </label>
                    <label>
                        "Left"<input type="radio" value="Left" name="case[side]" required />
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
                            <option value="Barrett">"Barrett"</option>
                            <option value="Evo">"EVO"</option>
                            <option value="HillRbf">"Hill RBF"</option>
                            <option value="Holladay2">"Holladay 2"</option>
                            <option value="Kane" selected>
                                "Kane"
                            </option>
                            <option value="Okulix">"Okulix raytracing"</option>
                            <option value="Olsen">"Olsen"</option>
                        </optgroup>
                        <optgroup label="Thin lens formulas">
                            <option value="Haigis">"Haigis"</option>
                            <option value="HofferQ">"Hoffer Q"</option>
                            <option value="Holladay1">"Holladay 1"</option>
                            <option value="SrkT">"SRK/T"</option>
                        </optgroup>
                        <optgroup label="Post-refractive formulas">
                            <option value="AscrsKrs">"ASCRS"</option>
                            <option value="BarrettTrueK">"Barrett True K"</option>
                            <option value="HaigisL">"Haigis-L"</option>
                        </optgroup>
                        <optgroup label="Other">
                            <option value="Other">"Not listed"</option>
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
                </label> <Suspense fallback=move || view! { "Fetching IOLs..." }>
                    <label>
                        "IOL model" <input list="iols" name="case[iol_model]" required />
                        <datalist id="iols">
                            <For
                                each=iols
                                key=|iol| iol.model.clone()
                                let(Iol { model, name, company, .. })
                            >
                                <option value=model>{name}" ("{company}")"</option>
                            </For>
                        </datalist>
                    </label>
                </Suspense>
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
pub async fn get_iols() -> Result<Vec<Iol>, AppError> {
    let json = db()
        .await?
        .query_json("select Iol { model, name, company, focus, toric };", &())
        .await?
        .to_string();

    Ok(serde_json::from_str::<Vec<Iol>>(json.as_str()).unwrap_or_default())
}

#[server]
pub async fn insert_form_case(form_case: FormCase) -> Result<(), AppError> {
    let client = db().await?;
    let surgeon_case = form_case.into_surgeon_case().await?;

    let returned_json = insert_surgeon_case(client, surgeon_case)
        .await?
        .ok_or(AppError::Db(
            "no JSON was returned after inserting the case".to_string(),
        ))?;

    // todo: redirect to a view that takes the returned String, parses it as JSON into a
    // SurgeonCase, and displays it alongside a button to add another case.

    Ok(())
}

// Passing in the client makes the function customizable for tests.
pub async fn insert_surgeon_case(
    client: gel_tokio::Client,
    surgeon_case: SurgeonCase,
) -> Result<Option<String>, AppError> {
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
                                model: iol_model, ..
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
    } = surgeon_case;

    let year = date.year();
    let date = date.to_string();
    let site_name = site.map(|Site { name }| name).unwrap_or("{}".to_string());

    let side = match side {
        Side::Right => "Side.Right",
        Side::Left => "Side.Left",
    };

    // note: We don't need to cast the integer types, because they are just going into a format
    // string. The <int32> will be inferred based on the object field types in Gel.
    let (flat_k_power, flat_k_axis, steep_k_power, steep_k_axis, cct, wtw) = (
        ks.flat_power(),
        ks.flat_axis(),
        ks.steep_power(),
        ks.steep_axis(),
        cct.map(|cct| cct.inner().to_string())
            .unwrap_or("{}".to_string()),
        wtw.map(|wtw| wtw.inner().to_string())
            .unwrap_or("{}".to_string()),
    );

    let formula = if let Some(formula) = formula {
        match formula {
            Formula::AscrsKrs => "Formula.AscrsKrs",
            Formula::Barrett => "Formula.Barrett",
            Formula::BarrettTrueK => "Formula.BarrettTrueK",
            Formula::Evo => "Formula.Evo",
            Formula::Haigis => "Formula.Haigis",
            Formula::HaigisL => "Formula.HaigisL",
            Formula::HillRbf => "Formula.HillRbf",
            Formula::HofferQ => "Formula.HofferQ",
            Formula::Holladay1 => "Formula.Holladay1",
            Formula::Holladay2 => "Formula.Holladay2",
            Formula::Kane => "Formula.Kane",
            Formula::Okulix => "Formula.Okulix",
            Formula::Olsen => "Formula.Olsen",
            Formula::SrkT => "Formula.SrkT",
            Formula::Other => "Formula.Other",
        }
    } else {
        "{}"
    };

    let target_cyl = if let Some(TargetCyl { power, axis }) = target_cyl {
        format!("(select(insert TargetCyl {{ power := {power}, axis := {axis} }}))")
    } else {
        "{}".to_string()
    };

    let iol_axis = if let Some(iol_axis) = iol_axis {
        format!("{iol_axis}")
    } else {
        "{}".to_string()
    };

    let adverse = if let Some(adverse) = adverse {
        match adverse {
            Adverse::Rhexis => "Adverse.Rhexis",
            Adverse::Pc => "Adverse.Pc",
            Adverse::Zonule => "Adverse.Zonule",
            Adverse::Other => "Adverse.Other",
        }
    } else {
        "{}"
    };

    let va_raw_before = if let Some(Va { num, den }) = va_raw_before {
        format!("(select (insert Va {{ num := {num}, den := {den} }}))")
    } else {
        "{}".to_string()
    };

    let va_best_after = if let Some(Va { num, den }) = va_best_after {
        format!("(select (insert Va {{ num := {num}, den := {den} }}))")
    } else {
        "{}".to_string()
    };

    let ref_before_cyl = if let Some(RefCyl { power, axis }) = ref_before_cyl {
        format!("(select (insert RefCyl {{ power := {power}, axis := {axis} }}))")
    } else {
        "{}".to_string()
    };

    let ref_after_cyl = if let Some(RefCyl { power, axis }) = ref_after_cyl {
        format!("(select (insert RefCyl {{ power := {power}, axis := {axis} }}))")
    } else {
        "{}".to_string()
    };

    let query = format!(
        r#"
with QueryBiometry := (insert Biometry {{
    al := {al},

    ks := (select(insert Ks {{
        flat := (select(insert K {{
            power := {flat_k_power},
            axis := {flat_k_axis}
        }})),

        steep := (select(insert K {{
            power := {steep_k_power},
            axis := {steep_k_axis}
        }}))
    }})),

    acd := {acd},
    lt := {lt},
    cct := {cct},
    wtw := {wtw}
}}),

QueryTarget := (insert Target {{
    formula := {formula},
    custom_constant := {custom_constant},
    se := {target_se},
    cyl := {target_cyl}
}}),

QueryIol := (select (insert OpIol {{
    iol := (select Iol filter .model = "{iol_model}"),
    se := {iol_se},
    axis := {iol_axis}
}})),

QueryVa := (insert OpVa {{
    before := (select (insert BeforeVa {{
        best := (select (insert Va {{
            num := {va_best_before_num},
            den := {va_best_before_den}
        }})),

        raw := {va_raw_before}
    }})),

    after := (select (insert AfterVa {{
        best := {va_best_after},

        raw := (select (insert Va {{
            num := {va_raw_after_num},
            den := {va_raw_after_den}
        }}))
    }}))
}}),

QueryRefraction := (select (insert OpRefraction {{
    before := (select (insert Refraction {{
        sph := {ref_before_sph},
        cyl := {ref_before_cyl}
    }})),

    after := (select (insert Refraction {{
        sph := {ref_after_sph},
        cyl := {ref_after_cyl}
    }}))
}})),

QueryCas := (insert Cas {{
    side := {side},
    biometry := (select QueryBiometry),
    target := (select QueryTarget),
    year := {year},
    main := {main},
    sia := (select (insert Sia {{ power := {sia_power}, axis := {sia_axis} }})),
    iol := (select QueryIol),
    adverse := {adverse},
    va := (select QueryVa),
    refraction := (select QueryRefraction)
}}),

QuerySurgeonCas := (insert SurgeonCas {{
    surgeon := (select global cur_surgeon),
    urn := "{urn}",
    side := {side},
    date := <cal::local_date>"{date}",

    site := (select (insert Site {{
        name := "{site_name}"
    }} unless conflict on .name else (select Site))),

    cas := (select QueryCas)
}})

select QuerySurgeonCas {{
    urn,
    date,
    site: {{ name }},

    cas: {{
        side,
        biometry: {{ al, ks, acd, lt, cct, wtw }},
        main,
        sia: {{ power, axis }},

        iol: {{
            iol: {{ model, name, company, focus, toric }},
            se,
            axis
        }},

        adverse,

        va: {{
            before: {{ best: {{ num, den }}, raw: {{ num, den }} }},
            after: {{ best: {{ num, den }}, raw: {{ num, den }} }},
        }},

        refraction: {{
            before: {{ sph, cyl: {{ power, axis}} }},
            after: {{ sph, cyl: {{ power, axis}} }}
        }}
    }}
}};
        "#
    );

    let case = client
        .query_single_json(query, &())
        .await?
        .map(|json| json.as_ref().to_string());
    dbg!(&case);

    Ok(case)
}
