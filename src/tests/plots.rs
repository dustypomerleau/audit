use crate::{
    plots::{ScatterCompare, get_compare_with_client},
    tests::common::{populate_test_db, test_db},
};
use plotly::{Plot, Scatter, common::Mode};

#[tokio::test]
async fn makes_a_plot() {
    let client = test_db().await;

    // uncomment below to populate the test DB with cases.
    // let client = populate_test_db().await;
    // assert!(client.ensure_connected().await.is_ok());

    let ScatterCompare { surgeon, cohort } = get_compare_with_client(&client, 2025)
        .await
        .unwrap()
        .scatter_delta_cyl();

    let surgeon = Scatter::new(surgeon.x, surgeon.y)
        .name("Surgeon")
        .mode(Mode::Markers);

    let cohort = Scatter::new(cohort.x, cohort.y)
        .name("Cohort")
        .mode(Mode::Markers);

    let mut plot = Plot::new();
    // note: the surgeon should be added after the cohort, because that allows hover on their
    // points, which are "on top" in the layered plot
    plot.add_traces(vec![cohort, surgeon]);
    plot.show();
}
