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

// i think we basically want the route to be Report, and then the view on Report contains
// PlotContainer(compare: Compare) and then all the subviews take &Compare and return a view
// if there's common patterns then maybe you do some work in PlotContainer and supply the plot with
// a ScatterCompare, if it's the same for several plots, but the data is shown differently
use crate::{
    bounded::Bounded,
    case::{Case, CaseError, SurgeonCase},
    db::{DbError, db},
    surgeon::{Email, Surgeon},
};
use leptos::{
    IntoView,
    prelude::{Get, GlobalAttributes, IntoAny, RwSignal, component, expect_context, server},
    reactive::spawn_local,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ScatterData {
    x: Vec<f32>,
    y: Vec<f32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ScatterCompare {
    surgeon: ScatterData,
    cohort: ScatterData,
}

// If you find that you are doing a lot of the same processing for future plots, you could
// have a Cased trait and impl it for both SurgeonCase and Case, and then just write these
// once over a generic, but hold off on this for now, it's premature.
impl Compare {
    pub fn scatter_delta_cyl(&self) -> ScatterCompare {
        let surgeon: (Vec<f32>, Vec<f32>) = self
            .surgeon_cases
            .iter()
            .map(|sc| {
                let ks = sc.case.biometry.ks;
                let pre = ((ks.steep_power() - ks.flat_power()) / 100) as f32;

                let post = sc
                    .case
                    .refraction
                    .after
                    .cyl
                    .map(|refcyl| (refcyl.power.inner() / 100) as f32)
                    .unwrap_or(0_f32);

                (pre, post)
            })
            .collect();

        let cohort: (Vec<f32>, Vec<f32>) = self
            .cohort_cases
            .iter()
            .map(|cc| {
                let ks = cc.biometry.ks;
                let pre = ((ks.steep_power() - ks.flat_power()) / 100) as f32;

                let post = cc
                    .refraction
                    .after
                    .cyl
                    .map(|refcyl| (refcyl.power.inner() / 100) as f32)
                    .unwrap_or(0_f32);

                (pre, post)
            })
            .collect();

        ScatterCompare {
            surgeon: ScatterData {
                x: surgeon.0,
                y: surgeon.1,
            },
            cohort: ScatterData {
                x: cohort.0,
                y: cohort.1,
            },
        }
    }
}

#[component]
pub fn DeltaCyl() -> impl IntoView {
    let email = expect_context::<Option<Surgeon>>().unwrap().email;
    let year = RwSignal::new(2025_u32);
    let compare_resource = OnceResource::new(get_compare(email, year.get()));

    // for each `Scatter` we are plotting magnitude of preop corneal cyl versus magnitude of
    // postoperative refractive cyl (do they need to be in the same plane, or is it ok that the
    // outcome measure is apples:apples)
    let ScatterCompare { surgeon, cohort } = if let Some(Ok(compare)) = compare_resource.get() {
        compare.scatter_delta_cyl()
    } else {
        return view! { "Query for the surgeon and cohort was not successful" }.into_any();
    };

    let surgeon = Scatter::new(surgeon.x, surgeon.y).name("Surgeon");
    let cohort = Scatter::new(cohort.x, cohort.y).name("Cohort");
    let mut plot = Plot::new();
    plot.add_traces(vec![surgeon, cohort]);
    spawn_local(async move { plotly::bindings::new_plot("plotly-delta-cyl", &plot).await });

    view! { <div id="plotly-delta-cyl"></div> }.into_any()
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
