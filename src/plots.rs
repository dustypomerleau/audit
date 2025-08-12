//! Utilities for creating the underlying data for a plot.

mod delta_cyl;

use crate::{
    bounded::{self, Bounded},
    error::AppError,
    model::{Case, SurgeonCase},
};
#[cfg(feature = "ssr")] use crate::{db::db, query::query_select_compare};
pub use delta_cyl::*;
#[cfg(feature = "ssr")] use gel_tokio::Client;
use leptos::prelude::server;
use serde::{Deserialize, Serialize};

/// bookmark: todo: docs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compare {
    surgeon_cases: Vec<SurgeonCase>,
    cohort_cases: Vec<Case>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PolarData {
    // Although we could define:
    //
    // bounded!((PolarAxis, u32, 0..=359));
    //
    // and use that instead of u32, it adds complexity to passing the data to Plotly, and the
    // bounds will already be met because of the constraints on the way into the DB.
    pub theta: Vec<u32>,
    pub r: Vec<f32>,
}

impl FromIterator<(u32, f32)> for PolarData {
    fn from_iter<T: IntoIterator<Item = (u32, f32)>>(iter: T) -> Self {
        let mut polar_data = PolarData::new();

        for (theta, r) in iter {
            polar_data.theta.push(theta);
            polar_data.r.push(r);
        }

        polar_data
    }
}

impl PolarData {
    fn new() -> Self {
        Self {
            theta: Vec::new(),
            r: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PolarCompare {
    pub surgeon: PolarData,
    pub cohort: PolarData,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ScatterData {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

impl FromIterator<(f32, f32)> for ScatterData {
    fn from_iter<T: IntoIterator<Item = (f32, f32)>>(iter: T) -> Self {
        let mut scatter_data = ScatterData::new();

        for (x, y) in iter {
            scatter_data.x.push(x);
            scatter_data.y.push(y);
        }

        scatter_data
    }
}

impl ScatterData {
    fn new() -> Self {
        Self {
            x: Vec::new(),
            y: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ScatterCompare {
    pub surgeon: ScatterData,
    pub cohort: ScatterData,
}

#[cfg(feature = "ssr")]
impl Compare {
    pub fn polar_cyl_before(&self) -> PolarCompare {
        let surgeon = self
            .surgeon_cases
            .iter()
            .map(|sc| {
                let ks = sc.case.biometry.ks;

                // We double the axis for double-angle plot.
                (ks.steep_axis() * 2, (ks.cyl() as f32) / 100.0)
            })
            .collect();

        let cohort = self
            .cohort_cases
            .iter()
            .map(|cc| {
                let ks = cc.biometry.ks;

                (ks.steep_axis() * 2, (ks.cyl() as f32) / 100.0)
            })
            .collect();

        PolarCompare { surgeon, cohort }
    }

    pub fn scatter_delta_cyl(&self) -> ScatterCompare {
        let surgeon = self
            .surgeon_cases
            .iter()
            .map(|sc| {
                let before = sc.case.biometry.ks.cyl() as f32 / 100.0;

                let after = sc
                    .case
                    .refraction
                    .after
                    .cyl
                    .map(|refcyl| (refcyl.power.inner() as f32 / 100.0).abs())
                    .unwrap_or(0_f32);

                (before, after)
            })
            .collect();

        let cohort = self
            .cohort_cases
            .iter()
            .map(|cc| {
                let before = cc.biometry.ks.cyl() as f32 / 100.0;

                let after = cc
                    .refraction
                    .after
                    .cyl
                    .map(|refcyl| (refcyl.power.inner() as f32 / 100.0).abs())
                    .unwrap_or(0_f32);

                (before, after)
            })
            .collect();

        ScatterCompare { surgeon, cohort }
    }
}

#[server]
// In future, you may want the ability to compare a specific date range for the Surgeon, against
// either the cohort, or against the surgeon's own baseline (all other dates outside the range).
//
// The reason we separate `get_compare_with_client()` into its own function is so we can call that
// function directly from tests, and inject a different [`Client`] with a test JWT global.
pub async fn get_compare(year: u32) -> Result<Compare, AppError> {
    let client = db().await?;

    get_compare_with_client(&client, year).await
}

#[cfg(feature = "ssr")]
pub async fn get_compare_with_client(client: &Client, year: u32) -> Result<Compare, AppError> {
    let query = query_select_compare(year);

    if let Some(query_result) = client.query_single_json(query, &()).await? {
        let compare = serde_json::from_str::<Compare>(query_result.as_ref())?;

        Ok(compare)
    } else {
        Err(AppError::Db(
            "the query for Compare was not successful".to_string(),
        ))
    }
}

#[cfg(test)]
#[cfg(feature = "ssr")]
mod tests {}
