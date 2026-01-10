#[cfg(feature = "ssr")] use chrono::Datelike;
use leptos::prelude::ActionForm;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoView;
use leptos::prelude::ServerAction;
use leptos::prelude::Suspense;
use leptos::prelude::component;
use leptos::prelude::server;
use leptos::prelude::view;
use leptos::server::OnceResource;
#[cfg(feature = "ssr")] use sqlx::query_as;

#[cfg(feature = "ssr")] use crate::bounded::Bounded;
#[cfg(feature = "ssr")] use crate::db::db;
use crate::error::AppError;
#[cfg(feature = "ssr")] use crate::model::Adverse;
#[cfg(feature = "ssr")] use crate::model::AfterVa;
#[cfg(feature = "ssr")] use crate::model::BeforeVa;
#[cfg(feature = "ssr")] use crate::model::Biometry;
#[cfg(feature = "ssr")] use crate::model::Case;
#[cfg(feature = "ssr")] use crate::model::Focus;
use crate::model::FormCase;
#[cfg(feature = "ssr")] use crate::model::Formula;
use crate::model::Iol;
#[cfg(feature = "ssr")] use crate::model::OpIol;
#[cfg(feature = "ssr")] use crate::model::OpRefraction;
#[cfg(feature = "ssr")] use crate::model::OpVa;
#[cfg(feature = "ssr")] use crate::model::Refraction;
#[cfg(feature = "ssr")] use crate::model::Sia;
#[cfg(feature = "ssr")] use crate::model::Side;
use crate::model::Site;
#[cfg(feature = "ssr")] use crate::model::SurgeonCase;
#[cfg(feature = "ssr")] use crate::model::Target;
#[cfg(feature = "ssr")] use crate::model::ToricPower;
#[cfg(feature = "ssr")] use crate::model::Va;

/// Display a form that inserts a `SurgeonCas` on submit.
#[component]
pub fn AddCase() -> impl IntoView {
    // TODO: load necessary datalists form the DB:
    //
    // 1. sites
    // 2. IOL models
    // 3. surgeon defaults (should already be in context)
    //
    // You probably want a single query for sites and IOL models.
    // For efficiency, you don't actually want to return a Vec<Iol>, but instead just
    // Vec<IolNameAndModel> or something, because you want a query like:
    //
    // ```
    // select
    //    iol.model as iol_model,
    //    iol.name as iol_name,
    //    site.name as site_name
    // from iol, site;
    //```
    // which you can then use to populate the datalists.
    //
    let insert_case = ServerAction::<InsertFormCase>::new();
    let insert_case_value = insert_case.value();
    dbg!(&insert_case_value);

    let iol_resource = OnceResource::new(get_iols());

    let iols = move || {
        iol_resource
            .get()
            .map(|res| res.unwrap_or_default())
            .unwrap_or_default()
    };

    // TODO: "To change your default values, please [update your profile](link)"
    // TODO: we need a signal holding the side to update the Sia
    // TODO: autofill k2 axis
    view! {
        <ActionForm action=insert_case>
            <div id="form-add-case" class="form-add-case">
                "Enter the case details (fields are required unless marked optional)."
                <br/><br/>"Do not enter any patient-identifying information."
                <br/><br/>"After each successful upload, you will receive a case number, which you can record in your own records to associate the case with a patient."
                <fieldset id="add-side">
                    <legend>"Side"</legend>
                    <label>
                        "Right"<input type="radio" value="Right" name="case[side]" required />
                    </label>
                    <label>
                        "Left"<input type="radio" value="Left" name="case[side]" required />
                    </label>
                </fieldset>
                <fieldset id="add-biometry" class="add-biometry">
                    <legend>"Biometry"</legend>
                    <label>
                        "AL (12–38 mm)"
                        <input type="number" min=12 max=38 step=0.01 name="case[al]" required />
                    </label>
                    <div id="add-ks" class="add-ks">
                        <div id="k1">
                            <label>
                                "K1 power (30–65 D)"
                                <input
                                    type="number"
                                    min=30
                                    max=65
                                    step=0.01
                                    name="case[k1_power]"
                                    required
                                />
                            </label>
                            <label>
                                "K1 axis (0–179°)"
                                <input
                                    type="number"
                                    min=0
                                    max=179
                                    step=1
                                    name="case[k1_axis]"
                                    required
                                />
                            </label>
                        </div>
                        <div id="k2">
                            <label>
                                "K2 power (30–65 D)"
                                <input
                                    type="number"
                                    min=30
                                    max=65
                                    step=0.01
                                    name="case[k2_power]"
                                    required
                                />
                            </label>
                            <label>
                                "K2 axis (0–179°)"
                                <input
                                    type="number"
                                    min=0
                                    max=179
                                    step=1
                                    name="case[k2_axis]"
                                    required
                                />
                            </label>
                        </div>
                    </div>
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
                </fieldset>
                <fieldset id="add-target">
                    <legend>"Target"</legend>
                    <label>
                        // TODO: prefill the surgeon's default formula
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
                        "SIA power (D)"
                        <input
                            type="number"
                            min=0
                            max=2
                            step=0.01
                            name="case[sia_power]"
                            required
                        />
                    </label>
                    <label>
                        "SIA axis (°)"
                        <input type="number" min=0 max=179 step=1 name="case[sia_axis]" required />
                    </label>
                    <label>
                        "Target spherical equivalent (-6–2 D)"
                        <input
                            type="number"
                            min=-6
                            max=2
                            step=0.01
                            name="case[target_se]"
                            required
                        />
                    </label>
                    <label>
                        "Target cylinder power (0–6 D, target cyl is optional but strongly encouraged)"
                        <input type="number" min=0 max=6 step=0.01 name="case[target_cyl_power]" />
                    </label>
                    <label>
                        "Target cylinder axis (0–179°)"
                        <input type="number" min=0 max=179 step=1 name="case[target_cyl_axis]" />
                    </label>
                </fieldset>
                <fieldset id="add-surgical-details">
                    <label>
                        "Date of surgery" <input type="date" name="case[date]" required />
                    </label>
                    <label>
                        "Hospital/Site (optional)" <input type="text" name="case[site]" />
                    </label>
                    <label>
                        "Main incision size (1–6 mm)"
                        <input type="number" min=1 max=6 step=0.05 name="case[main]" required />
                    </label>
                    <Suspense fallback=move || view! { "Fetching IOLs..." }>
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
                        <input
                            type="number"
                            min=-20
                            max=60
                            step=0.25
                            name="case[iol_se]"
                            required
                        />
                    </label>
                    <label>
                        // TODO: hide this field using a signal if the model is nontoric
                        "IOL axis (0–179°)"
                        <input type="number" min=0 max=179 step=1 name="case[iol_axis]" />
                    </label>
                    >
                    <fieldset id="add-adverse">
                        <legend>"Adverse event"</legend>
                        <label>
                            "None"
                            <input
                                type="radio"
                                value="none"
                                name="case[adverse]"
                                required
                                checked
                            />
                        </label>
                        <label>
                            "Rhexis"
                            <input type="radio" value="rhexis" name="case[adverse]" required />
                        </label>
                        <label>
                            "PC"<input type="radio" value="pc" name="case[adverse]" required />
                        </label>
                        <label>
                            "Zonule"
                            <input type="radio" value="zonule" name="case[adverse]" required />
                        </label>
                        <label>
                            "Other"
                            <input type="radio" value="other" name="case[adverse]" required />
                        </label>
                    </fieldset>
                </fieldset>
                <fieldset id="add-va">
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
                                    name="case[va_before_raw_num]"
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input
                                    type="number"
                                    min=1
                                    step=0.1
                                    name="case[va_before_raw_den]"
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
                                    name="case[va_before_best_num]"
                                    required
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input
                                    type="number"
                                    min=1
                                    step=0.1
                                    name="case[va_before_best_den]"
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
                                    name="case[va_after_raw_num]"
                                    required
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input
                                    type="number"
                                    min=1
                                    step=0.1
                                    name="case[va_after_raw_den]"
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
                                    name="case[va_after_best_num]"
                                />
                            </label>
                            <label>
                                "Denominator"
                                <input
                                    type="number"
                                    min=1
                                    step=0.1
                                    name="case[va_after_best_den]"
                                />
                            </label>
                        </div>
                    </div>
                </fieldset>
                <fieldset id="add-refraction">
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
                </fieldset>
                <input type="submit" value="Submit case" />
            </div>
            {insert_case_value.get()}
        </ActionForm>
    }
}

struct DlIol {
    model: String,
    name: String,
}

struct AddCaseDatalists {
    iols: Vec<DlIol>,
    sites: Vec<Site>,
}

// TODO: change get_iols to be a single resource that returns AddCaseDatalists

/// Return a [`Vec`] of all [`Iol`]s in the database.
#[server]
pub async fn get_iols() -> Result<Vec<Iol>, AppError> {
    let iols = query_as!(
        Iol,
        r#"
select model, name, company, focus as "focus: Focus", toric as "toric: ToricPower" from iol;
        "#
    )
    .fetch_all(&db().await?)
    .await?;
    dbg!(&iols);

    Ok(iols)
}

/// Insert a [`SurgeonCas`] into the database on form submit.
#[server]
pub async fn insert_form_case(case: FormCase) -> Result<i32, AppError> {
    let surgeon_case = case.into_surgeon_case().await?;
    let surgeon_case_number = insert_surgeon_case(surgeon_case).await?;

    Ok(surgeon_case_number)
}

/// Insert a [`SurgeonCase`] into the database using the given [`sqlx::Pool`]. Passing in the pool
/// makes it possible to use custom pools for tests.
#[cfg(feature = "ssr")]
pub async fn insert_surgeon_case(surgeon_case: SurgeonCase) -> Result<i32, AppError> {
    use chrono::NaiveDate;

    use crate::model::Acd;
    use crate::model::Al;
    use crate::model::Axis;
    use crate::model::Cct;
    use crate::model::Email;
    use crate::model::IolSe;
    use crate::model::Kpower;
    use crate::model::Lt;
    use crate::model::Main;
    use crate::model::RefCylPower;
    use crate::model::RefSph;
    use crate::model::SiaPower;
    use crate::model::SplitOption;
    use crate::model::TargetCylPower;
    use crate::model::TargetSe;
    use crate::model::VaDen;
    use crate::model::VaNum;
    use crate::model::Wtw;
    use crate::model::Year;

    let SurgeonCase {
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
                        iol,
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
                                        num: va_before_best_num,
                                        den: va_before_best_den,
                                    },
                                raw: va_before_raw,
                            },
                        after:
                            AfterVa {
                                best: va_after_best,
                                raw:
                                    Va {
                                        num: va_after_raw_num,
                                        den: va_after_raw_den,
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
        ..
    } = surgeon_case;

    // TODO: figure out how to pass in the identity of the current Surgeon, matching
    // email is a temporary workaround during wip.
    let email = Email::new("todo@todo.com")?;

    let year = Year::new(date.year())?;
    let site_name = site.map(|site| site.name);
    let (target_cyl_power, target_cyl_axis) = target_cyl.split_option();
    let iol_model = iol.map(|iol| iol.model);
    let (va_before_raw_num, va_before_raw_den) = va_before_raw.split_option();
    let (va_after_best_num, va_after_best_den) = va_after_best.split_option();
    let (ref_before_cyl_power, ref_before_cyl_axis) = ref_before_cyl.split_option();
    let (ref_after_cyl_power, ref_after_cyl_axis) = ref_after_cyl.split_option();

    pub struct SurgeonCaseNumber {
        number: i32,
    }

    let case = query_as!(
        SurgeonCaseNumber,
        r#"
with c as (
    insert into cas (
        side,
        al,
        flat_k_power,
        flat_k_axis,
        steep_k_power,
        steep_k_axis,
        acd,
        lt,
        cct,
        wtw,
        target_formula,
        target_custom_constant,
        target_se,
        target_cyl_power,
        target_cyl_axis,
        year,
        main,
        sia_power,
        sia_axis,
        iol_id,
        iol_se,
        iol_axis,
        adverse,
        va_before_best_num,
        va_before_best_den,
        va_before_raw_num,
        va_before_raw_den,
        va_after_best_num,
        va_after_best_den,
        va_after_raw_num,
        va_after_raw_den,
        ref_before_sph,
        ref_before_cyl_power,
        ref_before_cyl_axis,
        ref_after_sph,
        ref_after_cyl_power,
        ref_after_cyl_axis
    )
    values (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19,
        (select id from iol where model = $20 limit 1),
        $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37
    )
    returning id 
),

s as (
    insert into surgeon_cas (surgeon_id, date, site_id, cas_id)
    values (
        (select id from surgeon where email = $38),
        $39,
        (select id from site where name = $40),
        (select id from c)
    )
    returning number
)

select number from s;
        "#,
        side as Side,
        al as Al,
        ks.flat_power() as Kpower,
        ks.flat_axis() as Axis,
        ks.steep_power() as Kpower,
        ks.steep_axis() as Axis,
        acd as Acd,
        lt as Lt,
        cct as Option<Cct>,
        wtw as Option<Wtw>,
        formula as Option<Formula>,
        custom_constant,
        target_se as TargetSe,
        target_cyl_power as Option<TargetCylPower>,
        target_cyl_axis as Option<Axis>,
        year as Year,
        main as Main,
        sia_power as SiaPower,
        sia_axis as Axis,
        iol_model,
        iol_se as IolSe,
        iol_axis as Option<Axis>,
        adverse as Option<Adverse>,
        va_before_best_num as VaNum,
        va_before_best_den as VaDen,
        va_before_raw_num as Option<VaNum>,
        va_before_raw_den as Option<VaDen>,
        va_after_best_num as Option<VaNum>,
        va_after_best_den as Option<VaDen>,
        va_after_raw_num as VaNum,
        va_after_raw_den as VaDen,
        ref_before_sph as RefSph,
        ref_before_cyl_power as Option<RefCylPower>,
        ref_before_cyl_axis as Option<Axis>,
        ref_after_sph as RefSph,
        ref_after_cyl_power as Option<RefCylPower>,
        ref_after_cyl_axis as Option<Axis>,
        email as Email,
        date as NaiveDate,
        site_name,
    )
    .fetch_one(&db().await?)
    .await?;

    // TODO: you need to decide whether you really want to return all the case data from this
    // function. The thing that makes sense to do is simply to return the value of `number`, and to
    // simply display the rest of the data from the SurgeonCase you passed in. You can do this by,
    // on form submit, showing a view that takes the SurgeonCase as a prop, and loads the number
    // returned from the DB as a resource. In that case, you only need to return i32, and you can
    // dramatically simplify the returned columns from the query. The only issue with that is that
    // it doesn't provide the same degree of confirmation about what was actually inserted, because
    // it doesn't round trip the data.
    //
    // I think the best way forward is not to show the outlet until the resource loads, so
    // basically pass the SurgeonCase to the view optimistically, but put it in a Suspense that
    // waits for the number to come back from the DB.

    Ok(case.number)
}
