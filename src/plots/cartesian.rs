use crate::plots::{AsPlot, Polar, PolarData, PolarPoint, mean, radians_to_degrees, theta_radians};
use plotly::{
    Layout, Plot, Scatter,
    common::{Anchor, Font, HoverInfo, LegendGroupTitle, Marker, Mode, Orientation},
    layout::{Axis, Legend, Margin, TraceOrder},
};
use serde::{Deserialize, Serialize};

/// Convert a polar dataset to a cartesian dataset.
pub trait Cartesian {
    type CartesianOutput;

    fn cartesian(&self) -> Self::CartesianOutput;
}

/// A pair of cartesian datasets, representing the surgeon of interest and a comparison cohort of
/// peers.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CartesianCompare {
    pub surgeon: CartesianData,
    pub cohort: CartesianData,
}

impl AsPlot for CartesianCompare {
    fn plot(&self) -> Plot {
        /// Place values into bins of 0.25 D, and return a vec of bin/mean pairs, where x is the
        /// middle of the bin.
        fn bins(data: &CartesianData) -> CartesianData {
            todo!()
        }

        /// Create custom hover labels for the plot.
        fn labels(data: &CartesianData) -> Vec<String> {
            data.points
                .iter()
                .map(|CartesianPoint { x, y }| format!("Pre: {x:.2} D, Post: {y:.2} D"))
                .collect()
        }

        // todo: set these as constants app-wide, adapt for light mode, and use in all plots
        // (see plots/polar.rs as well)
        let cohort_centroid_marker_color = "#f5f5f6";
        let cohort_confidence_color = "#848998";
        let cohort_marker_color = "#848998";
        let grid_color = "#363a48";
        let label_color = "#eaebed";
        let legend_font_color = "#caccd1";
        let legend_group_font_color = "#eaebed";
        let paper_background_color = "#252833";
        let surgeon_centroid_marker_color = "#00f115";
        let surgeon_confidence_color = "#f100dc";
        let surgeon_marker_color = "#ff7b00";
        let tick_color = "#acafb9";

        let Self { surgeon, cohort } = self;

        let surgeon = surgeon
            .scatter()
            .name("cases")
            .legend_group("surgeon")
            .legend_group_title(
                LegendGroupTitle::new()
                    .text("Surgeon")
                    .font(Font::new().color(legend_group_font_color)),
            )
            .mode(Mode::Markers)
            .marker(Marker::new().color(surgeon_marker_color))
            // .hover_template(hover_template);
            .hover_info(HoverInfo::Text)
            .hover_text_array(labels(surgeon));

        let cohort = cohort
            .scatter()
            .name("cases")
            .legend_group("cohort")
            .legend_group_title(
                LegendGroupTitle::new()
                    // Hack: adding spaces to the name because Plotly doesn't have horizontal group
                    // spacing.
                    .text("Peer cohort    ")
                    .font(Font::new().color(legend_group_font_color)),
            )
            .mode(Mode::Markers)
            .marker(Marker::new().color(cohort_marker_color))
            .opacity(0.4)
            .hover_info(HoverInfo::Skip);

        // todo: add plots for 0.25 D bins using the centroid colors

        let mut plot = Plot::new();
        plot.add_traces(vec![cohort, surgeon]);

        let axis = Axis::new()
            .color(label_color)
            .show_line(false)
            .zero_line(false)
            .grid_color(grid_color);

        let layout = Layout::new()
            .x_axis(axis.clone())
            .y_axis(axis)
            .paper_background_color(paper_background_color)
            .plot_background_color(paper_background_color)
            .margin(Margin::new().top(30).right(30).bottom(0).left(30))
            .legend(
                Legend::new()
                    .font(Font::new().color(legend_font_color))
                    .trace_order(TraceOrder::Grouped)
                    .orientation(Orientation::Horizontal)
                    .x_anchor(Anchor::Center)
                    .x(0.5)
                    .y_anchor(Anchor::Top)
                    .y(-0.1),
            );

        plot.set_layout(layout);

        plot
    }
}

// todo: consider creating a cartesian confidence and associated method for the ellipse, like you
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

impl FromIterator<CartesianPoint> for CartesianData {
    fn from_iter<T: IntoIterator<Item = CartesianPoint>>(iter: T) -> Self {
        let mut points = Vec::new();

        for point in iter {
            points.push(point);
        }

        Self { points }
    }
}

impl Polar for CartesianData {
    type PolarOutput = PolarData;

    fn polar(&self) -> Self::PolarOutput {
        self.points
            .iter()
            .map(|CartesianPoint { x, y }| {
                let theta_radians = theta_radians(*x, *y);
                let theta = radians_to_degrees(theta_radians);
                let r = f64::sqrt((x * x) + (y * y));

                PolarPoint { r, theta }
            })
            .collect()
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

    /// Scale a cartesian dataset in place, multiplying the [`x`](CartesianPoint::x) and
    /// [`y`](CartesianPoint::y) values for each point by the given scale factor.
    pub fn scale(mut self, scale: &Scale) -> Self {
        self.points.iter_mut().for_each(|point| {
            point.x *= scale.x;
            point.y *= scale.y
        });

        self
    }

    /// Create a [`Scatter`] trace from a [`CartesianData`].
    pub fn scatter(&self) -> Box<Scatter<f64, f64>> {
        let (x, y) = self.split_axes();

        Scatter::new(x, y)
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

impl Polar for CartesianPoint {
    type PolarOutput = PolarPoint;

    fn polar(&self) -> Self::PolarOutput {
        let CartesianPoint { x, y } = self;
        let theta_radians = theta_radians(*x, *y);
        let theta = radians_to_degrees(theta_radians);
        let r = f64::sqrt((x * x) + (y * y));

        PolarPoint { r, theta }
    }
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

