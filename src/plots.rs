//! Utilities for creating [`Plot`]s and their underlying data.

// Notes on converting subj refraction powers to corneal plane:
// Power(K plane) =
//   Power(Spec plane) / (1 - (Power(Spec Plane)(Vertex distance in m)))
//
// The range for a standard vertex distance starting point is typically 12-14 mm, so using 13 mm as
// a default is reasonable for these calcs.
//
// This needs to be created as a method on RefSph and RefCylPower, or a trait they share.

use crate::{
    bounded::Bounded,
    db::db,
    error::AppError,
    model::{Case, SurgeonCase},
    query::query_select_compare,
};
use gel_tokio::Client;
use plotly::{Plot, Scatter, ScatterPolar, common::Mode};
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
    // bounds will already be met because of the constraints on the DB.
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

impl PolarCompare {
    pub fn polar_plot(self) -> Plot {
        let Self { surgeon, cohort } = self;

        let surgeon = ScatterPolar::new(surgeon.theta, surgeon.r)
            .name("Surgeon")
            .mode(Mode::Markers);

        let cohort = ScatterPolar::new(cohort.theta, cohort.r)
            .name("Cohort")
            .mode(Mode::Markers)
            .opacity(0.6);

        let mut polar_plot = Plot::new();
        // note: the surgeon should be added after the cohort, because that allows hover on their
        // points, which are "on top" in the layered plot
        polar_plot.add_traces(vec![cohort, surgeon]);

        polar_plot
    }
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

impl ScatterCompare {
    pub fn scatter_plot(self) -> Plot {
        let Self { surgeon, cohort } = self;

        let surgeon = Scatter::new(surgeon.x, surgeon.y)
            .name("Surgeon")
            .mode(Mode::Markers);

        let cohort = Scatter::new(cohort.x, cohort.y)
            .name("Cohort")
            .mode(Mode::Markers)
            .opacity(0.6);

        let mut scatter_plot = Plot::new();
        // note: the surgeon should be added after the cohort, because that allows hover on their
        // points, which are "on top" in the layered plot
        scatter_plot.add_traces(vec![cohort, surgeon]);

        scatter_plot
    }
}

impl Compare {
    pub fn polar_cyl_before(&self) -> PolarCompare {
        fn k_cyl_double_angle(case: &Case) -> (u32, f32) {
            let ks = case.biometry.ks;

            // We double the axis for double-angle plot.
            // Consider halving the axis instead, so it is properly labeled as 0-179.
            (ks.steep_axis() * 2, (ks.cyl() as f32) / 100.0)
        }

        let surgeon = self
            .surgeon_cases
            .iter()
            .map(|sc| k_cyl_double_angle(&sc.case))
            .collect();

        let cohort = self.cohort_cases.iter().map(k_cyl_double_angle).collect();

        PolarCompare { surgeon, cohort }
    }

    pub fn scatter_delta_cyl(&self) -> ScatterCompare {
        fn k_cyl_before(case: &Case) -> f32 {
            case.biometry.ks.cyl() as f32 / 100.0
        }

        fn ref_cyl_after(case: &Case) -> f32 {
            case.refraction
                .after
                .cyl
                .map(|refcyl| (refcyl.power.inner() as f32 / 100.0).abs())
                .unwrap_or(0_f32)
        }

        let surgeon = self
            .surgeon_cases
            .iter()
            .map(|sc| (k_cyl_before(&sc.case), ref_cyl_after(&sc.case)))
            .collect();

        let cohort = self
            .cohort_cases
            .iter()
            .map(|cc| (k_cyl_before(cc), ref_cyl_after(cc)))
            .collect();

        ScatterCompare { surgeon, cohort }
    }
}

// In future, you may want the ability to compare a specific date range for the Surgeon, against
// either the cohort, or against the surgeon's own baseline (all other dates outside the range).
//
// The reason we separate `get_compare_with_client()` into its own function is so we can call that
// function directly from tests, and inject a different [`Client`] with a test JWT global.
pub async fn get_compare(year: u32) -> Result<Compare, AppError> {
    let client = db().await?;

    get_compare_with_client(&client, year).await
}

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
mod tests {}

// For reference, the exported HTML that Plotly produces looks like:
// <!doctype html>
// <html lang="en">
//     <head>
//         <meta charset="utf-8" />
//     </head>
//
//     <body>
//         <div>
//             <script src="https://cdn.plot.ly/plotly-2.12.1.min.js"></script>
//             <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
//             <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.0/es5/tex-mml-chtml.js"></script>
//
//             <div
//                 id="plotly-html-element"
//                 class="plotly-graph-div"
//                 style="height:100%; width:100%;"
//             ></div>
//
//             <script type="module">
//                 const graph_div = document.getElementById("plotly-html-element");
//                 await Plotly.newPlot(graph_div, {
//                     data: [
//                         { type: "scatter", name: "Surgeon", x: [5.0, 0.0], y: [0.0, 0.0] },
//                         { type: "scatter", name: "Cohort", x: [0.0], y: [0.0] },
//                     ],
//                     layout: {},
//                     config: {},
//                 });
//             </script>
//         </div>
//     </body>
// </html>
