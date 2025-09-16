use crate::plots::{PolarData, mean, radians_to_degrees, theta_radians};
use plotly::{Plot, Scatter, common::Mode};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A pair of cartesian datasets, representing the surgeon of interest and a comparison cohort of
/// peers.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CartesianCompare {
    pub surgeon: CartesianData,
    pub cohort: CartesianData,
}

impl CartesianCompare {
    /// Generate a cartesian [`Plot`] from a [`CartesianCompare`].
    pub fn plot(self) -> Plot {
        let Self { surgeon, cohort } = self;

        let ((surgeon_x, surgeon_y), (cohort_x, cohort_y)) =
            (surgeon.split_axes(), cohort.split_axes());

        let surgeon = Scatter::new(surgeon_x, surgeon_y)
            .name("Surgeon")
            .mode(Mode::Markers);

        let cohort = Scatter::new(cohort_x, cohort_y)
            .name("Cohort")
            .mode(Mode::Markers)
            .opacity(0.6);

        let mut plot = Plot::new();
        // note: the surgeon should be added after the cohort, because that allows hover on their
        // points, which are "on top" in the layered plot
        plot.add_traces(vec![cohort, surgeon]);

        plot
    }
}

// todo: consider creating a cartesian tolerance and associated method for the ellipse, like you
// did with the polar plot.

/// A cartesian dataset, with values along the [`x`](CartesianPoint::x) and [`y`](CartesianPoint::y)
/// axes.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CartesianData {
    pub points: Vec<CartesianPoint>,
}

impl FromIterator<(f64, f64)> for CartesianData {
    fn from_iter<T: IntoIterator<Item = (f64, f64)>>(iter: T) -> Self {
        let mut points = Vec::new();

        for (x, y) in iter {
            points.push(CartesianPoint { x, y });
        }

        Self { points }
    }
}

impl CartesianData {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    // Eventually I think you separate this out into a trait Covariance: Variance: Mean (or some
    // arrangement like this), and then functions like centroid could be generic over that.
    /// Calculate the covariance of a cartesian dataset.
    pub fn covariance(&self) -> f64 {
        let (x, y) = self.split_axes();
        let (mean_x, mean_y) = (mean(&x), mean(&y));
        let population = (self.points.len() - 1) as f64;

        let sum = self
            .points
            .iter()
            .map(|CartesianPoint { x, y }| (x - mean_x) * (y - mean_y))
            .sum::<f64>();

        sum / population
    }

    /// Convert a cartesian dataset to a polar dataset.
    pub fn polar(&self) -> PolarData {
        self.points
            .iter()
            .map(|CartesianPoint { x, y }| {
                let theta_radians = theta_radians(*x, *y);
                let theta_degrees = radians_to_degrees(theta_radians);
                let r = f64::sqrt((x * x) + (y * y));

                (theta_degrees, r)
            })
            .collect()
    }

    /// Scale a cartesian dataset in place, multiplying the [`x`](CartesianPoint::x) and
    /// [`y`](CartesianPoint::y) values for each point by the given scale factor.
    pub fn scale(mut self, scale: &Scale) -> Self {
        self.points.iter_mut().for_each(|point| {
            point.x *= scale.x;
            point.y *= scale.y
        });

        self
    }

    /// Separate a cartesian dataset into 2 vectors of equal length, containing values for
    /// [`x`](CartesianPoint::x) and [`y`](CartesianPoint::y), respectively. This is useful for
    /// generating [`Plot`]s, which require each axis to be a separate vector.
    pub fn split_axes(&self) -> (Vec<f64>, Vec<f64>) {
        self.points
            .iter()
            .map(|CartesianPoint { x, y }| (x, y))
            .collect()
    }

    /// Translate a cartesian dataset within its 2-dimensional plane by mutating it in place, adding
    /// appropriate translation values to the [`x`](CartesianPoint::x) and [`y`](CartesianPoint::y)
    /// values for each point.
    pub fn translate(mut self, translate: &Translate) -> Self {
        self.points.iter_mut().for_each(|point| {
            point.x += translate.x;
            point.y += translate.y
        });

        self
    }
}

/// A single point on a cartesian plot, where [`x`](CartesianPoint::x) represents the horizontal
/// axis, and [`y`](CartesianPoint::y) represents the vertical axis.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CartesianPoint {
    pub x: f64,
    pub y: f64,
}

/// A set of scale factors for growing or shrinking a cartesian plot along its axes.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Scale {
    pub x: f64,
    pub y: f64,
}

/// A set of translation values for repositioning a cartesian plot along its axes.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Translate {
    pub x: f64,
    pub y: f64,
}
