//! Utilities for creating the underlying data for a plot.

mod delta_cyl;

use crate::{
    bounded::Bounded,
    model::{Case, CaseError, SurgeonCase},
};
#[cfg(feature = "ssr")] use crate::{
    db::{DbError, db},
    query::query_select_compare,
};
pub use delta_cyl::*;
use leptos::prelude::server;
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
                let pre = ((ks.steep_power() - ks.flat_power()) as f32 / 100.0).abs();

                let post = sc
                    .case
                    .refraction
                    .after
                    .cyl
                    .map(|refcyl| (refcyl.power.inner() as f32 / 100.0).abs())
                    .unwrap_or(0_f32);

                (pre, post)
            })
            .collect();

        let cohort: (Vec<f32>, Vec<f32>) = self
            .cohort_cases
            .iter()
            .map(|cc| {
                let ks = cc.biometry.ks;
                let pre = ((ks.steep_power() - ks.flat_power()) as f32 / 100.0).abs();

                let post = cc
                    .refraction
                    .after
                    .cyl
                    .map(|refcyl| (refcyl.power.inner() as f32 / 100.0).abs())
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

#[server]
// In future, you may want the ability to compare a specific date range for the Surgeon, against
// either the cohort, or against the surgeon's own baseline (all other dates outside the range).
pub async fn get_compare(year: u32) -> Result<Compare, CaseError> {
    let query = query_select_compare(year);

    let query_result = db()
        .await?
        // The JSON returned here is just one object, mapping to a Compare
        .query_single_json(query, &())
        .await
        .map_err(|err| DbError::Gel(format!("{err:?}")))?
        .unwrap(); // todo: handle properly once you get the test below passing

    let compare = serde_json::from_str::<Compare>(query_result.as_ref())?;

    Ok(compare)
}

#[cfg(test)]
#[cfg(feature = "ssr")]
mod tests {
    use super::*;
    use crate::db::tests::test_db;

    #[tokio::test]
    async fn queries_compare() {
        let year = 2025_u32;
        let query = query_select_compare(year);

        let query_result = test_db()
            .await
            .query_single_json(query, &())
            .await
            .unwrap()
            .unwrap();
        dbg!(&query_result);

        let compare = serde_json::from_str::<Compare>(query_result.as_ref()).unwrap();
        dbg!(&compare);
    }
}
