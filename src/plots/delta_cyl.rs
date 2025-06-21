// In terms of structure, we want something like a parent Plot function/component that takes a
// struct holding a vec of SurgeonCase and a comparison vec of Case. Then this parent component
// passes the right fields for analysis to all of the subplots, so the subplots need to be able to
// accept data by reference.
//
// The idea is to use the builder syntax to make separate traces for the surgeon and the cohort,
// and then put both of the traces on one plot. `let trace = ScatterPolar::new(...);`.
//
// As a test case, perhaps start with a simple Histogram that shows the distribution of
// postoperative refractive astigmatism for surgeon and cohort. This simple plot would not take
// preop data into account, only the goal of complete cyl elimination. And then follow with polar
// scatter double angle plots and the like.
//
// Actually start with x = preop corneal astigmatism, y = postop refractive astigmatism (scatter)

// i think we basically want the route to be Report, and then the view on Report contains
// PlotContainer(compare: Compare) and then all the subviews take &Compare and return a view
// if there's common patterns then maybe you do some work in PlotContainer and supply the plot with
// a ScatterCompare, if it's the same for several plots, but the data is shown differently
use crate::{
    model::Surgeon,
    plots::{ScatterCompare, get_compare},
};
use leptos::{
    IntoView,
    prelude::{Get, GlobalAttributes, IntoAny, RwSignal, component, expect_context},
    reactive::spawn_local,
    server::OnceResource,
    view,
};
use plotly::{Plot, Scatter};

#[component]
pub fn DeltaCyl() -> impl IntoView {
    let email = expect_context::<Option<Surgeon>>().unwrap().email;
    let year = RwSignal::new(2025_u32);
    let compare_resource = OnceResource::new(get_compare(email, year.get()));

    // for each `Scatter` we are plotting magnitude of preop corneal cyl versus magnitude of
    // postoperative refractive cyl (do they need to be in the same plane, or is it ok that the
    // outcome measure is apples:apples)
    let ScatterCompare { surgeon, cohort } = if let Some(Ok(compare)) = compare_resource.get() {
        compare.scatter_delta_cyl()
    } else {
        return view! { "Query for the surgeon and cohort was not successful" }.into_any();
    };

    let surgeon = Scatter::new(surgeon.x, surgeon.y).name("Surgeon");
    let cohort = Scatter::new(cohort.x, cohort.y).name("Cohort");
    let mut plot = Plot::new();
    plot.add_traces(vec![surgeon, cohort]);
    spawn_local(async move { plotly::bindings::new_plot("plotly-delta-cyl", &plot).await });

    view! { <div id="plotly-delta-cyl"></div> }.into_any()
}
