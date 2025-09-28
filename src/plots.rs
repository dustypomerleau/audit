//! Utilities for creating [`Plot`]s and their underlying data.

// Notes on converting subj refraction powers to corneal plane:
// Power(K plane) =
//   Power(Spec plane) / (1 - (Power(Spec Plane)(Vertex distance in m)))
//
// The range for a standard vertex distance starting point is typically 12-14 mm, so using 13 mm as
// a default is reasonable for these calcs.
//
// This needs to be created as a method on RefSph and RefCylPower, or a trait they share.

mod cartesian;
mod case;
mod polar;

use crate::bounded::Bounded;
pub use cartesian::*;
pub use case::*;
pub use polar::*;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

bounded!((StdDev, f64, 1.0..=5.0), (PlotStep, f64, 0.001..=1.0));

/// The characteristics of a confidence ellipse.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConfidenceParams {
    /// Whether to use population variance or sample variance. See [`Variance`].
    variance: Variance,
    /// The number of standard deviations the confidence ellipse should cover.
    std_dev: StdDev,
    /// The size of the steps between points in the confidence ellipse (in the same units as
    /// [`r`](crate::plots::PolarPoint::r)).
    step: PlotStep,
}

/// Represents the variance of a 1-dimensional dataset.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Variance {
    /// The entire population is available in the dataset.
    Population,
    /// The entire dataset is not available, and only a sample has been taken.
    Sample,
}

/// Calculate the variance of a 1-dimensional dataset, for either an entire population or a
/// representative sample of the data.
pub fn variance(data: &[f64], variance: Variance) -> f64 {
    let population = match variance {
        Variance::Population => data.len() as f64,
        Variance::Sample => (data.len() - 1) as f64,
    };

    let mean = mean(data);

    let sum = data
        .iter()
        .map(|value| {
            let diff = value - mean;
            diff * diff
        })
        .sum::<f64>();

    sum / population
}

/// Convert from degrees to radians, typically before performing trigonometric calculations.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Convert from radians to degrees.
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}

/// Calculate the average value of a 1-dimensional dataset.
fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

/// Convert a single, cartesian, XY pair to its corresponding polar angle in radians, taking into
/// account the adjustments needed for each quadrant.
pub fn theta_radians(x: f64, y: f64) -> f64 {
    // We special-case 0.0 because we can't divide by it.
    let theta_radians = if x == 0.0 { PI / 2.0 } else { f64::atan(y / x) };

    // Adjust the value of theta, based on the quadrant of the XY pair.
    match (x.is_sign_negative(), y.is_sign_negative()) {
        // If the cartesian point is in the upper right corner (0° to 90°), then both values are
        // positive and the quotient is positive. We can use the angle unchanged.
        (false, false) => theta_radians,

        // If the cartesian point is in the upper left corner (90° to 180°), then x is negative and
        // the quotient is negative. adding this negative value to Pi radians (180°) is like
        // subtracting the acute angle from 180° to get the obtuse angle.
        (true, false) => PI + theta_radians,

        // If the cartesian point is in the lower left corner (180° to 270°), then both values are
        // negative and the quotient is positive. We add Pi radians (180°) to get the full angle.
        (true, true) => PI + theta_radians,

        // If the cartesian point is in the lower right corner (270° to 360°), then y is negative
        // and the quotient is negative. Adding this negative value to 2Pi radians is like
        // subtracting the acute angle from 360° to get the larger angle.
        (false, true) => (2.0 * PI) + theta_radians,
    }
}

#[cfg(test)]
mod tests {}

// For reference, the exported HTML that Plotly produces looks like this:
//
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
