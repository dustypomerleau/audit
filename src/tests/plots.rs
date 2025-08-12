use crate::{
    plots::{PolarCompare, ScatterCompare, get_compare_with_client},
    tests::common::{populate_test_db, test_db},
};
use plotly::{Plot, Scatter, ScatterPolar, common::Mode};

#[tokio::test]
async fn makes_a_plot() {
    let client = test_db().await;

    // uncomment below to populate the test DB with cases.
    // let client = populate_test_db().await;
    // assert!(client.ensure_connected().await.is_ok());

    let compare = get_compare_with_client(&client, 2025).await.unwrap();

    let ScatterCompare { surgeon, cohort } = compare.scatter_delta_cyl();

    let surgeon = Scatter::new(surgeon.x, surgeon.y)
        .name("Surgeon")
        .mode(Mode::Markers);

    let cohort = Scatter::new(cohort.x, cohort.y)
        .name("Cohort")
        .mode(Mode::Markers);

    let mut scatter_plot = Plot::new();
    // note: the surgeon should be added after the cohort, because that allows hover on their
    // points, which are "on top" in the layered plot
    scatter_plot.add_traces(vec![cohort, surgeon]);
    scatter_plot.show();

    let PolarCompare { surgeon, cohort } = compare.polar_cyl_before();

    let surgeon = ScatterPolar::new(surgeon.theta, surgeon.r)
        .name("Surgeon")
        .mode(Mode::Markers);

    let cohort = ScatterPolar::new(cohort.theta, cohort.r)
        .name("Cohort")
        .mode(Mode::Markers);

    let mut polar_plot = Plot::new();
    polar_plot.add_traces(vec![cohort, surgeon]);
    polar_plot.show();
}
