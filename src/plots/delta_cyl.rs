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
    case::{Case, CaseError, SurgeonCase},
    db::{DbError, db},
    surgeon::Email,
};
use chrono::NaiveDate;
use leptos::{
    prelude::{ServerFnError, component},
    server,
};
use plotly::Plot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Compare {
    surgeon_cases: Vec<SurgeonCase>,
    cohort_cases: Vec<Case>,
}

#[component]
pub fn DeltaCyl(compare: Compare) -> Plot {}

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
