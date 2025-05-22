// In terms of structure, we want something like a parent Plot function/component that takes a
// struct holding a vec of SurgeonCase and a comparison vec of Case. Then this parent component
// passes the right fields for analysis to all of the subplots, so the subplots need to be able to
// accept data by reference.
//
// The idea is to use the builder syntax to make separate traces for the surgeon and the cohort,
// and then put both of the traces on one plot. `let trace = ScatterPolar::new(...);`.
//
// As a test case, perhaps start with a simple Histogram that shows the distribution of
// postoperative refractive astigmatism for surgeon and cohort. This simple plot would not take
// preop data into account, only the goal of complete cyl elimination. And then follow with polar
// scatter double angle plots and the like.
//
// Actually start with x = preop corneal astigmatism, y = postop refractive astigmatism (scatter)

use crate::{
    bounded::Bounded,
    case::{Case, CaseError, SurgeonCase},
    db::{DbError, db},
    surgeon::{Email, Surgeon},
};
use leptos::{
    IntoView,
    prelude::{Get, GlobalAttributes, IntoAny, RwSignal, component, expect_context, server},
    server::OnceResource,
    view,
};
use plotly::{Plot, Scatter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Compare {
    surgeon_cases: Vec<SurgeonCase>,
    cohort_cases: Vec<Case>,
}

#[component]
pub fn DeltaCyl() -> impl IntoView {
    let email = expect_context::<Option<Surgeon>>().unwrap().email;
    let year = RwSignal::new(2025_u32);
    let compare_resource = OnceResource::new(get_compare(email, year.get()));

    // for each `Scatter` we are plotting magnitude of preop corneal cyl versus magnitude of
    // postoperative refractive cyl (do they need to be in the same plane, or is it ok that the
    // outcome measure is apples:apples)
    let (surgeon, cohort): ((Vec<f32>, Vec<f32>), (Vec<f32>, Vec<f32>)) =
        if let Some(Ok(Compare {
            surgeon_cases,
            cohort_cases,
        })) = compare_resource.get()
        {
            let surgeon = surgeon_cases
                .iter()
                .map(|sc| {
                    let ks = sc.case.biometry.ks;
                    let pre = ((ks.steep_power() - ks.flat_power()) / 100) as f32;
                    let refcyl = sc.case.refraction.after.cyl;

                    let post = if let Some(refcyl) = refcyl {
                        // todo: inner().absolute_value() or however it's done
                        (refcyl.power.inner() / 100) as f32
                    } else {
                        0_f32
                    };

                    (pre, post)
                })
                .collect();

            let cohort = cohort_cases
                .iter()
                .map(|cc| {
                    let ks = cc.biometry.ks;
                    let pre = ((ks.steep_power() - ks.flat_power()) / 100) as f32;
                    let refcyl = cc.refraction.after.cyl;

                    let post = if let Some(refcyl) = refcyl {
                        // todo: inner().absolute_value() or however it's done
                        (refcyl.power.inner() / 100) as f32
                    } else {
                        0_f32
                    };

                    (pre, post)
                })
                .collect();

            (surgeon, cohort)
        } else {
            return view! { "Query for the surgeon and cohort was not successful" }.into_any();
        };

    let surgeon = Scatter::new(surgeon.0, surgeon.1).name("Surgeon");
    let cohort = Scatter::new(cohort.0, cohort.1).name("Cohort");
    let mut plot = Plot::new();
    plot.add_traces(vec![surgeon, cohort]);

    view! { <div id="plotly"></div> }.into_any()
}

#[server]
pub async fn get_compare(email: Email, year: u32) -> Result<Compare, CaseError> {
    let query = format!(
        r#"
with
surgeon_cases := (select SurgeonCas {{
    urn,
    side,
    date,
    site {{ name }},
    cas: {{
        side,

        biometry: {{
            al,
            ks {{ flat: {{ power, axis}}, steep: {{ power, axis }} }},
            acd,
            lt,
            cct,
            wtw
        }},
        
        target: {{
            formula,
            custom_constant,
            se,
            cyl: {{ power, axis }}
        }},
        
        year,
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
            after: {{ best: {{ num, den }}, raw: {{ num, den }} }}
        }},

        refraction: {{
            before: {{ sph, cyl: {{ power, axis }} }},
            after: {{ sph, cyl: {{ power, axis }} }},
        }}

    }}
}} filter .surgeon.email = "{email}" and .cas.year = {year}),

cohort_cases := (select Cas {{
    side,

    biometry: {{
        al,
        ks {{ flat: {{ power, axis}}, steep: {{ power, axis }} }},
        acd,
        lt,
        cct,
        wtw
    }},
    
    target: {{
        formula,
        custom_constant,
        se,
        cyl: {{ power, axis }}
    }},
    
    year,
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
        after: {{ best: {{ num, den }}, raw: {{ num, den }} }}
    }},

    refraction: {{
        before: {{ sph, cyl: {{ power, axis }} }},
        after: {{ sph, cyl: {{ power, axis }} }}
    }}
}} filter .year = {year})

select {{ surgeon_cases, cohort_cases }};
        "#
    );

    let query_result = db()
        .await?
        .query_json(query, &())
        .await
        .map_err(|err| DbError::Gel(format!("{err:?}")))?;

    let compare = serde_json::from_str::<Compare>(query_result.as_ref())?;

    Ok(compare)
}
