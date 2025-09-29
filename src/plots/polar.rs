use crate::{
    bounded::Bounded,
    plots::{
        AsPlot, CartesianData, CartesianPoint, ConfidenceParams, PlotStep, Scale, StdDev,
        Translate, Variance, degrees_to_radians, mean, radians_to_degrees, theta_radians, variance,
    },
};
use plotly::{
    Layout, Plot, ScatterPolar,
    common::{Font, HoverInfo, LegendGroupTitle, Line, Marker, Mode},
    layout::{
        AngularAxis, LayoutPolar, Legend, PolarAxisAttributes, PolarAxisTicks, PolarTickMode,
        RadialAxis, TraceOrder,
    },
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// A pair of polar datasets, representing the surgeon of interest and a comparison cohort of
/// peers.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PolarCompare {
    pub surgeon: PolarData,
    pub cohort: PolarData,
}

impl AsPlot for PolarCompare {
    fn plot(&self) -> Plot {
        /// Create custom hover labels for double-angle plots.
        fn labels(data: &PolarData) -> Vec<String> {
            data.points
                .iter()
                .map(|PolarPoint { r, theta }| format!("{:.2} D × {:.0}°", r, *theta / 2.0))
                .collect()
        }

        let Self { surgeon, cohort } = self;

        let ((surgeon_r, surgeon_theta), (cohort_r, cohort_theta)) =
            (surgeon.split_axes(), cohort.split_axes());

        let (surgeon_centroid, cohort_centroid) = (surgeon.centroid(), cohort.centroid());

        let (
            (surgeon_centroid_r, surgeon_centroid_theta),
            (cohort_centroid_r, cohort_centroid_theta),
        ) = (surgeon_centroid.split_axes(), cohort_centroid.split_axes());

        let (surgeon_labels, surgeon_centroid_labels, cohort_centroid_labels) = (
            labels(surgeon),
            labels(&surgeon_centroid),
            labels(&cohort_centroid),
        );

        let surgeon_ellipse = surgeon.confidence(&ConfidenceParams {
            variance: Variance::Population,
            // todo: unwrap_or_default() hangs the program if your value is out of bounds, because
            // you derived Default in bounded!. You need to rethink that decision and come up with
            // a solution to an out of bounds value in these cases.
            std_dev: StdDev::new(2.0).unwrap(),
            // Ellipse breaks are evident at 0.05, so the step should probably be bounded
            // 0.001..=0.05 or something.
            step: PlotStep::new(0.01).unwrap(),
        });

        let cohort_ellipse = cohort.confidence(&ConfidenceParams {
            variance: Variance::Population,
            std_dev: StdDev::new(2.0).unwrap(),
            step: PlotStep::new(0.01).unwrap(),
        });

        let (surgeon_ellipse_r, surgeon_ellipse_theta) = surgeon_ellipse.split_axes();
        let (cohort_ellipse_r, cohort_ellipse_theta) = cohort_ellipse.split_axes();

        // todo: set these as constants app-wide and use in all plots
        let grid_color = "#363a48";
        let label_color = "#eaebed";
        let legend_font_color = "#caccd1";
        let legend_group_font_color = "#eaebed";
        let tick_color = "#acafb9";

        // These format strings use d3 format syntax:
        // https://d3js.org/d3-format
        // The empty <extra> tag is necessary to avoid displaying the trace name.
        //
        // let hover_template = "Power: %{r:.2f} D<br />Axis: %{theta:.0f}°<extra></extra>";

        let surgeon = ScatterPolar::new(surgeon_theta, surgeon_r)
            .name("cases")
            .legend_group("surgeon")
            .legend_group_title(
                LegendGroupTitle::new()
                    .text("Surgeon")
                    .font(Font::new().color(legend_group_font_color)),
            )
            .mode(Mode::Markers)
            .marker(Marker::new().color("#ff7b00"))
            // .hover_template(hover_template);
            .hover_info(HoverInfo::Text)
            .hover_text_array(surgeon_labels);

        let cohort = ScatterPolar::new(cohort_theta, cohort_r)
            .name("cases")
            .legend_group("cohort")
            .legend_group_title(
                LegendGroupTitle::new()
                    .text("Peer cohort")
                    .font(Font::new().color(legend_group_font_color)),
            )
            .mode(Mode::Markers)
            .marker(Marker::new().color("#848998"))
            .opacity(0.4)
            .hover_info(HoverInfo::Skip);

        let surgeon_centroid = ScatterPolar::new(surgeon_centroid_theta, surgeon_centroid_r)
            .name("centroid")
            .legend_group("surgeon")
            .mode(Mode::Markers)
            .marker(Marker::new().color("#00f115").size(10))
            .hover_info(HoverInfo::Text)
            .hover_text_array(surgeon_centroid_labels);

        let cohort_centroid = ScatterPolar::new(cohort_centroid_theta, cohort_centroid_r)
            .name("centroid")
            .legend_group("cohort")
            .mode(Mode::Markers)
            .marker(Marker::new().color("#f5f5f6").size(10))
            .hover_info(HoverInfo::Text)
            .hover_text_array(cohort_centroid_labels);

        let surgeon_ellipse = ScatterPolar::new(surgeon_ellipse_theta, surgeon_ellipse_r)
            .name("confidence (2 SD)")
            .legend_group("surgeon")
            .mode(Mode::Lines)
            .line(Line::new().color("#f100dc").width(1.5))
            .hover_info(HoverInfo::Skip);

        let cohort_ellipse = ScatterPolar::new(cohort_ellipse_theta, cohort_ellipse_r)
            .name("confidence (2 SD)")
            .legend_group("cohort")
            .mode(Mode::Lines)
            .line(Line::new().color("#848998").width(1.5))
            .opacity(0.7)
            .hover_info(HoverInfo::Skip);

        let mut plot = Plot::new();

        plot.add_traces(vec![
            cohort,
            surgeon,
            cohort_centroid,
            surgeon_centroid,
            cohort_ellipse,
            surgeon_ellipse,
        ]);

        let radial_ticks = PolarAxisTicks::new().tick_color(tick_color);

        let angular_ticks = PolarAxisTicks::new()
            .tick_mode(PolarTickMode::Array {
                tick_values: Some(vec![0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0]),

                tick_text: Some(vec![
                    "0°".to_string(),
                    "".to_string(),
                    "45°".to_string(),
                    "".to_string(),
                    "90°".to_string(),
                    "".to_string(),
                    "135°".to_string(),
                    "".to_string(),
                ]),
            })
            .tick_color(tick_color);

        let axis_attributes = PolarAxisAttributes::new()
            .color(label_color)
            .show_line(false)
            .grid_color(grid_color);

        let radial_axis_attributes = axis_attributes.clone().ticks(radial_ticks);
        let angular_axis_attributes = axis_attributes.ticks(angular_ticks);
        let radial_axis = RadialAxis::new().axis_attributes(radial_axis_attributes.clone());
        let angular_axis = AngularAxis::new().axis_attributes(angular_axis_attributes);

        // todo: use a signal on the plot container to set the colors for light/dark mode.
        // Find a way to do this with CSS variables (not natively available in Plotly).
        let polar_layout = LayoutPolar::new()
            .bg_color("#252833")
            .radial_axis(radial_axis)
            .angular_axis(angular_axis);

        let layout = Layout::new()
            .paper_background_color("#252833")
            .polar(polar_layout)
            .legend(
                Legend::new()
                    .font(Font::new().color(legend_font_color))
                    .trace_order(TraceOrder::Grouped)
                    .trace_group_gap(30),
            );

        plot.set_layout(layout);

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

        for (r, theta) in iter {
            points.push(PolarPoint { r, theta });
        }

        Self { points }
    }
}

impl FromIterator<PolarPoint> for PolarData {
    fn from_iter<T: IntoIterator<Item = PolarPoint>>(iter: T) -> Self {
        let mut points = Vec::new();

        for point in iter {
            points.push(point);
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
            .map(|PolarPoint { r, theta }| {
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
                r: r_centroid,
                theta: theta_centroid_degrees,
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
    ///  [`r`](PolarPoint::r) and [`theta`](PolarPoint::theta), respectively. This is useful for
    /// generating [`Plot`]s, which require each axis to be a separate vector.
    // todo: it's probably better not to use this tuple, because it's easy to confuse the
    // order of r and theta, but instead to directly produce a ScatterPolar::new and
    // return it.
    pub fn split_axes(&self) -> (Vec<f64>, Vec<f64>) {
        self.points
            .iter()
            .map(|point| (point.r, point.theta))
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
    /// This function calculates a confidence ellipse, which is more closely related to a tolerance
    /// interval (a range where most values fall) than a confidence interval (a range likely to
    /// contain the mean).
    pub fn confidence(&self, params: &ConfidenceParams) -> PolarData {
        let ConfidenceParams {
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
    pub r: f64,
    pub theta: f64,
}

#[cfg(test)]
mod tests {}
