//! Utilities for creating the underlying data for a plot.

mod delta_cyl;

use crate::{
    bounded::Bounded,
    model::{Case, SurgeonCase},
};
pub use delta_cyl::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[cfg(feature = "ssr")] use crate::db::{DbError, db};
use crate::model::{CaseError, Email};
use leptos::prelude::server;

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
