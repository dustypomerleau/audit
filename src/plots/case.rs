use gel_tokio::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::db::db;
use crate::error::AppError;
use crate::model::Case;
use crate::model::RefCyl;
use crate::model::Refraction;
use crate::model::SurgeonCase;
use crate::model::Target;
use crate::model::TargetCyl;
use crate::plots::Cartesian;
use crate::plots::CartesianCompare;
use crate::plots::CartesianPoint;
use crate::plots::Polar;
use crate::plots::PolarCompare;
use crate::plots::PolarPoint;
use crate::plots::VertexK;
use crate::query::query_select_compare;
use crate::query::query_select_self_compare;

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

pub fn ref_cyl_double_angle(case: &Case) -> PolarPoint {
    match case.refraction.after.cyl {
        None => PolarPoint { r: 0.0, theta: 0.0 },

        Some(RefCyl { power, axis }) => {
            // Convert to diopters and vertex to the corneal plane.
            let power = power.vertex();

            // We double the axis to create a double angle plot. Since the type from the DB
            // can't exceed 179, our doubled axis can't exceed 358.
            let axis = f64::from(axis.inner()) * 2.0;

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

                PolarPoint {
                    r: power,
                    theta: axis,
                }
            } else {
                PolarPoint {
                    r: power,
                    theta: axis,
                }
            }
        }
    }
}

impl CaseCompare {
    /// Compare preoperative corneal cylinder values.
    pub fn polar_cyl_before(&self) -> PolarCompare {
        fn k_cyl_double_angle(case: &Case) -> PolarPoint {
            let ks = case.biometry.ks;

            PolarPoint {
                r: f64::from(ks.cyl()) / 100.0,
                // We double the axis to create a double-angle plot.
                theta: f64::from(ks.steep_axis()) * 2.0,
            }
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
        let surgeon = self
            .surgeon
            .iter()
            .map(|sc| ref_cyl_double_angle(&sc.case))
            .collect();

        let cohort = self.cohort.iter().map(ref_cyl_double_angle).collect();

        PolarCompare { surgeon, cohort }
    }

    // todo: Do we need an equivalent for SE or sph?
    // todo: you don't want this to generate negative differences
    pub fn polar_cyl_target_error(&self) -> PolarCompare {
        fn delta_target(case: &Case) -> PolarPoint {
            let target = if let Target {
                cyl: Some(TargetCyl { power, axis }),
                ..
            } = case.target
            {
                // todo: We are presuming that the target power is in the spectacle plane, and
                // therefore needs to be vertexed to corneal plane, but this needs to be
                // confirmed (Abulafia article says yes).
                PolarPoint {
                    r: power.vertex(),
                    theta: f64::from(axis.inner()),
                }
                .cartesian()
            } else {
                CartesianPoint { x: 0.0, y: 0.0 }
            };

            let refraction = if let Refraction {
                cyl: Some(RefCyl { power, axis }),
                ..
            } = case.refraction.after
            {
                PolarPoint {
                    r: power.vertex(),
                    theta: f64::from(axis.inner()),
                }
                .cartesian()
            } else {
                CartesianPoint { x: 0.0, y: 0.0 }
            };

            CartesianPoint {
                x: refraction.x - target.x,
                y: refraction.y - target.y,
            }
            .polar()
        }

        let CaseCompare { surgeon, cohort } = self;
        let surgeon = surgeon.iter().map(|sc| delta_target(&sc.case)).collect();
        let cohort = cohort.iter().map(delta_target).collect();

        PolarCompare { surgeon, cohort }
    }

    /// Compare preoperative corneal cylinder and postoperative refractive cylinder (vertexed to the
    /// corneal plane). We use the absolute value of the cylinder, because the axis isn't relevant
    /// for this plot.
    pub fn cartesian_delta_cyl(&self) -> CartesianCompare {
        fn k_cyl_before(case: &Case) -> f64 { f64::from(case.biometry.ks.cyl()) / 100.0 }

        fn ref_cyl_after(case: &Case) -> f64 {
            case.refraction
                .after
                .cyl
                .map(|RefCyl { power, .. }| power.vertex().abs())
                .unwrap_or(0.0)
        }

        let surgeon = self
            .surgeon
            .iter()
            .map(|SurgeonCase { case, .. }| CartesianPoint {
                x: k_cyl_before(case),
                y: ref_cyl_after(case),
            })
            .collect();

        let cohort = self
            .cohort
            .iter()
            .map(|case| CartesianPoint {
                x: k_cyl_before(case),
                y: ref_cyl_after(case),
            })
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
