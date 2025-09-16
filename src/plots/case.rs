use crate::{
    bounded::Bounded,
    db::db,
    error::AppError,
    model::{Case, SurgeonCase},
    plots::{CartesianCompare, PolarCompare},
    query::query_select_compare,
};
use gel_tokio::Client;
use serde::{Deserialize, Serialize};

/// A pair of case datasets, representing the surgeon of interest and a comparison cohort of peers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaseCompare {
    surgeon: Vec<SurgeonCase>,
    cohort: Vec<Case>,
}

impl CaseCompare {
    /// Compare preoperative corneal cylinder values.
    pub fn polar_cyl_before(&self) -> PolarCompare {
        fn k_cyl_double_angle(case: &Case) -> (f64, f64) {
            let ks = case.biometry.ks;

            // We double the axis to create a double-angle plot.
            (ks.steep_axis() as f64 * 2.0, ks.cyl() as f64 / 100.0)
        }

        let surgeon = self
            .surgeon
            .iter()
            .map(|sc| k_cyl_double_angle(&sc.case))
            .collect();

        let cohort = self.cohort.iter().map(k_cyl_double_angle).collect();

        PolarCompare { surgeon, cohort }
    }

    /// Compare preoperative corneal cylinder and postoperative refractive cylinder.
    pub fn cartesian_delta_cyl(&self) -> CartesianCompare {
        fn k_cyl_before(case: &Case) -> f64 {
            case.biometry.ks.cyl() as f64 / 100.0
        }

        fn ref_cyl_after(case: &Case) -> f64 {
            case.refraction
                .after
                .cyl
                .map(|refcyl| (refcyl.power.inner() as f64 / 100.0).abs())
                .unwrap_or(0_f64)
        }

        let surgeon = self
            .surgeon
            .iter()
            .map(|sc| (k_cyl_before(&sc.case), ref_cyl_after(&sc.case)))
            .collect();

        let cohort = self
            .cohort
            .iter()
            .map(|cc| (k_cyl_before(cc), ref_cyl_after(cc)))
            .collect();

        CartesianCompare { surgeon, cohort }
    }
}

// In future, you may want the ability to compare a specific date range for the Surgeon, against
// either the cohort, or against the surgeon's own baseline (all other dates outside the range).
//
// The reason we separate `get_compare_with_client()` into its own function is so we can call that
// function directly from tests, and inject a different [`Client`] with a test JWT global.
/// Query the database for cases from the given year.
pub async fn get_compare(year: u32) -> Result<CaseCompare, AppError> {
    let client = db().await?;

    get_compare_with_client(&client, year).await
}

/// Query the database for cases from the given year, using a custom [`gel_tokio::Client`].
pub async fn get_compare_with_client(client: &Client, year: u32) -> Result<CaseCompare, AppError> {
    let query = query_select_compare(year);

    if let Some(query_result) = client.query_single_json(query, &()).await? {
        let compare = serde_json::from_str::<CaseCompare>(query_result.as_ref())?;

        Ok(compare)
    } else {
        Err(AppError::Db(
            "the query for Compare was not successful".to_string(),
        ))
    }
}
