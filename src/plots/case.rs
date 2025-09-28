use crate::{
    bounded::Bounded,
    db::db,
    error::AppError,
    model::{Case, RefCyl, SurgeonCase, VertexK},
    plots::{CartesianCompare, PolarCompare},
    query::{query_select_compare, query_select_self_compare},
};
use gel_tokio::Client;
use serde::{Deserialize, Serialize};

/// The reference group for plot comparisons. This is either the full cohort of surgeons
/// participating (same year), or the current surgeon (prior year).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Cohort {
    #[default]
    Peers,
    Surgeon,
}

/// A pair of case datasets, representing the surgeon of interest and a comparison cohort of peers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaseCompare {
    surgeon: Vec<SurgeonCase>,
    cohort: Vec<Case>,
}

impl CaseCompare {
    /// Compare preoperative corneal cylinder values.
    pub fn polar_cyl_before(&self) -> PolarCompare {
        // It's important to maintain the pattern of power first, then axis, which means that tuples
        // for plotting should always be in (r, theta) order.
        // An alternative to avoid this pitfall is just to create [`PolarPoint`]s here.
        fn k_cyl_double_angle(case: &Case) -> (f64, f64) {
            let ks = case.biometry.ks;

            // We double the axis to create a double-angle plot.
            (ks.cyl() as f64 / 100.0, ks.steep_axis() as f64 * 2.0)
        }

        let surgeon = self
            .surgeon
            .iter()
            .map(|sc| k_cyl_double_angle(&sc.case))
            .collect();

        let cohort = self.cohort.iter().map(k_cyl_double_angle).collect();

        PolarCompare { surgeon, cohort }
    }

    /// Compare postoperative refractive cylinder values, vertexed to the corneal plane.
    pub fn polar_cyl_after(&self) -> PolarCompare {
        fn ref_cyl_double_angle(case: &Case) -> (f64, f64) {
            let cyl = case.refraction.after.cyl;

            match cyl {
                None => (0.0, 0.0),

                Some(RefCyl { power, axis }) => {
                    // Convert to diopters and vertex to the corneal plane.
                    let power = power.vertex();

                    // We double the axis to create a double angle plot. Since the type from the DB
                    // can't exceed 179, our doubled axis can't exceed 358.
                    let axis = axis.inner() as f64 * 2.0;

                    if power.is_sign_negative() {
                        let power = -power;

                        // When converting to plus cyl, we would normally add 90° % 180, but since
                        // we are already working with doubled angles, we need to add 180° % 360. If
                        // the doubled axis is _exactly_ 180, then we want to return 0.0 rather than
                        // 360.0 for plotting purposes (even though they are equivalent), so we
                        // special-case that situation.
                        let axis = if axis == 180.0 {
                            0.0
                        } else {
                            (axis + 180.0) % 360.0
                        };

                        (power, axis)
                    } else {
                        (power, axis)
                    }
                }
            }
        }

        let surgeon = self
            .surgeon
            .iter()
            .map(|sc| ref_cyl_double_angle(&sc.case))
            .collect();

        let cohort = self.cohort.iter().map(ref_cyl_double_angle).collect();

        PolarCompare { surgeon, cohort }
    }

    /// Compare preoperative corneal cylinder and postoperative refractive cylinder.
    // todo: vertex the postop refractive cylinder to the corneal plane
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
pub async fn get_compare(year: u32, cohort: Cohort) -> Result<CaseCompare, AppError> {
    let client = db().await?;

    get_compare_with_client(&client, year, cohort).await
}

/// Query the database for cases from the given year, using a custom [`gel_tokio::Client`].
pub async fn get_compare_with_client(
    client: &Client,
    year: u32,
    cohort: Cohort,
) -> Result<CaseCompare, AppError> {
    let query = match cohort {
        Cohort::Peers => query_select_compare(year),
        Cohort::Surgeon => query_select_self_compare(year),
    };

    if let Some(query_result) = client.query_single_json(query, &()).await? {
        let compare = serde_json::from_str::<CaseCompare>(query_result.as_ref())?;

        Ok(compare)
    } else {
        Err(AppError::Db(
            "the query for Compare was not successful".to_string(),
        ))
    }
}
