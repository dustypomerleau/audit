use crate::{
    bounded::Bounded,
    plots::{
        CartesianData, CartesianPoint, PlotStep, Scale, StdDev, ToleranceParams, Translate,
        Variance, degrees_to_radians, mean, radians_to_degrees, theta_radians, variance,
    },
};
use plotly::{Plot, ScatterPolar, common::Mode};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// A pair of polar datasets, representing the surgeon of interest and a comparison cohort of
/// peers.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PolarCompare {
    pub surgeon: PolarData,
    pub cohort: PolarData,
}

impl PolarCompare {
    // todo: consider whether `plot(&self) -> Plot` should be a trait.
    /// Generate a polar [`Plot`] from a [`PolarCompare`].
    pub fn plot(&self) -> Plot {
        let Self { surgeon, cohort } = self;

        let ((surgeon_theta, surgeon_r), (cohort_theta, cohort_r)) =
            (surgeon.split_axes(), cohort.split_axes());

        let (centroid_theta, centroid_r) = self.surgeon.centroid().split_axes();

        let ellipse = self.surgeon.tolerance(&ToleranceParams {
            variance: Variance::Population,
            std_dev: StdDev::new(2.0).unwrap_or_default(),
            step: PlotStep::new(0.01).unwrap_or_default(),
        });

        let ellipse2 = self.surgeon.tolerance(&ToleranceParams {
            variance: Variance::Population,
            std_dev: StdDev::new(3.0).unwrap_or_default(),
            step: PlotStep::new(0.01).unwrap_or_default(),
        });

        let (ellipse_theta, ellipse_r) = ellipse.split_axes();
        let (ellipse2_theta, ellipse2_r) = ellipse2.split_axes();

        let surgeon = ScatterPolar::new(surgeon_theta, surgeon_r)
            .name("Surgeon")
            .mode(Mode::Markers);

        let cohort = ScatterPolar::new(cohort_theta, cohort_r)
            .name("Cohort")
            .mode(Mode::Markers)
            .opacity(0.6);

        let centroid = ScatterPolar::new(centroid_theta, centroid_r)
            .name("Centroid (surgeon)")
            .mode(Mode::Markers);

        let ellipse_1 = ScatterPolar::new(ellipse_theta, ellipse_r)
            .name("Tolerance (2 std dev)")
            .mode(Mode::Lines);

        let ellipse_2 = ScatterPolar::new(ellipse2_theta, ellipse2_r)
            .name("Tolerance (3 std dev)")
            .mode(Mode::Lines);

        let mut plot = Plot::new();
        // note: the surgeon should be added after the cohort, because that allows hover on their
        // points, which are "on top" in the layered plot
        plot.add_traces(vec![cohort, surgeon, centroid, ellipse_1, ellipse_2]);

        plot
    }
}

/// A polar dataset, with angular values ([`Self::theta`]) and radial values ([`Self::r`]).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PolarData {
    // Although we could define:
    //
    // bounded!((PolarAxis, f64, 0.0..360.0));
    //
    // and use that instead of f64, it adds complexity to passing the data to Plotly, and the
    // bounds will already be met because of the constraints on the DB.
    pub points: Vec<PolarPoint>,
}

impl FromIterator<(f64, f64)> for PolarData {
    fn from_iter<T: IntoIterator<Item = (f64, f64)>>(iter: T) -> Self {
        let mut points = Vec::new();

        for (theta, r) in iter {
            points.push(PolarPoint { theta, r });
        }

        Self { points }
    }
}

impl PolarData {
    /// Create a new instance of [`PolarData`], without data points.
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Convert a polar dataset to a cartesian dataset.
    pub fn cartesian(&self) -> CartesianData {
        self.points
            .iter()
            .map(|PolarPoint { theta, r }| {
                let (sin_theta, cos_theta) = degrees_to_radians(*theta).sin_cos();
                let x = r * cos_theta;
                let y = r * sin_theta;

                (x, y)
            })
            .collect()
    }

    /// Returns a [`PolarData`] containing a single [`PolarPoint`] representing the mean vector of
    /// the polar dataset.
    pub fn centroid(&self) -> PolarData {
        let (x, y) = self.cartesian().split_axes();
        let (mean_x, mean_y) = (mean(&x), mean(&y));
        let theta_centroid_radians = theta_radians(mean_x, mean_y);
        let theta_centroid_degrees = radians_to_degrees(theta_centroid_radians);
        let r_centroid = f64::sqrt((mean_x * mean_x) + (mean_y * mean_y));

        PolarData {
            points: vec![PolarPoint {
                theta: theta_centroid_degrees,
                r: r_centroid,
            }],
        }
    }

    /// Perform a rotation on a polar dataset by mutating the values of [`theta`](PolarPoint::theta)
    /// in place.
    pub fn rotate(mut self, degrees: f64) -> Self {
        self.points
            .iter_mut()
            .for_each(|point| point.theta = (point.theta + degrees) % 360.0);

        self
    }

    /// Separate a polar dataset into 2 vectors of equal length, containing values for
    /// [`theta`](PolarPoint::theta) and [`r`](PolarPoint::r), respectively. This is useful for
    /// generating [`Plot`]s, which require each axis to be a separate vector.
    pub fn split_axes(&self) -> (Vec<f64>, Vec<f64>) {
        self.points
            .iter()
            .map(|point| (point.theta, point.r))
            .collect()
    }

    pub fn theta_sort(&mut self) {
        self.points
            .sort_by(|a, b| a.theta.partial_cmp(&b.theta).unwrap_or(Ordering::Equal));
    }

    // This function uses the method
    // [described by Carsten Schelp](https://carstenschelp.github.io/2018/09/14/Plot_Confidence_Ellipse_001.html),
    // but requires extra steps, as we need to convert between polar and cartesian coordinates at
    // the appropriate steps, and we need to manually generate points for our ellipse (Carsten's
    // method relies on built-in Matplotlib functionality to draw the ellipse).
    //
    // Python implementations of Carsten's method can be found at:
    //
    // https://matplotlib.org/stable/gallery/statistics/confidence_ellipse.html#sphx-glr-gallery-statistics-confidence-ellipse-py
    //
    // and
    //
    // https://gist.github.com/CarstenSchelp/b992645537660bda692f218b562d0712
    //
    /// Generate an ellipse encompassing the points within a given number of standard deviations.
    /// This function calculates a tolerance interval (a range where most values fall), rather
    /// than a confidence interval (a range likely to contain the mean).
    pub fn tolerance(&self, params: &ToleranceParams) -> PolarData {
        let ToleranceParams {
            variance: params_variance,
            std_dev,
            step,
        } = params;

        let cartesian_data = self.cartesian();
        let covariance_xy = cartesian_data.covariance();
        let (x, y) = cartesian_data.split_axes();
        let (mean_x, mean_y) = (mean(&x), mean(&y));

        let (variance_x, variance_y) = (
            variance(&x, params_variance.clone()),
            variance(&y, params_variance.clone()),
        );

        let pearson = covariance_xy / f64::sqrt(variance_x * variance_y);
        let radius_x = f64::sqrt(1.0 + pearson);
        let radius_y = f64::sqrt(1.0 - pearson);
        let (lower_x, upper_x) = (0.0 - radius_x, 0.0 + radius_x);
        let mut points = Vec::<CartesianPoint>::new();
        points.push(CartesianPoint { x: lower_x, y: 0.0 });
        points.push(CartesianPoint { x: upper_x, y: 0.0 });
        // Changing the size of the step will adjust the smoothness of the ellipse.
        let mut current_x = lower_x + step.inner();

        // In cartesian coordinates, solving for y, our ellipse equation is:
        //
        // y = +/- (b / a)(sqrt(a^2 - x^2))
        //
        // For each value of x, generate a positive and a negative value for y:
        while current_x < upper_x {
            let positive_y = (radius_y / radius_x)
                * (f64::sqrt((radius_x * radius_x) - (current_x * current_x)));

            let negative_y = -positive_y;

            points.push(CartesianPoint {
                x: current_x,
                y: positive_y,
            });

            points.push(CartesianPoint {
                x: current_x,
                y: negative_y,
            });

            current_x += step.inner();
        }

        let mut ellipse = CartesianData { points }.polar();
        // It's essential to sort the ellipse in theta order before transforming it, while it still
        // surrounds the origin. That way the lines connecting each point will still draw the
        // outside of the ellipse, rather than crossing it.
        ellipse.theta_sort();
        // We duplicate the first point in the ellipse at the end, to ensure that our ellipse
        // is fully closed.
        ellipse.points.push(ellipse.points[0].clone());

        // The standard deviation is the square root of the variance, and we multiply that by the
        // number of standard deviations we want our ellipse to cover.
        let scale = Scale {
            x: f64::sqrt(variance_x) * std_dev.inner(),
            y: f64::sqrt(variance_y) * std_dev.inner(),
        };

        let translate = Translate {
            x: mean_x,
            y: mean_y,
        };

        ellipse
            .rotate(45.0)
            .cartesian()
            .scale(&scale)
            .translate(&translate)
            .polar()
    }
}

// todo: these fields should probably be bounded/newtyped. r must be non-negative, and theta must
// be 0.0..360.0.
/// A single point on a polar plot, where [`theta`](Self::theta) represents the angular axis, and
/// [`r`](Self::r) represents the radial axis.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PolarPoint {
    pub theta: f64,
    pub r: f64,
}

#[cfg(test)]
mod tests {}
