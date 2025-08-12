use crate::{plots::get_compare_with_client, tests::common::test_db};

#[tokio::test]
async fn creates_plots() {
    let client = test_db().await;

    // uncomment below to populate the test DB with cases.
    // let client = populate_test_db().await;
    // assert!(client.ensure_connected().await.is_ok());

    let compare = get_compare_with_client(&client, 2025).await.unwrap();
    let scatter_plot = compare.scatter_delta_cyl().scatter_plot();
    let polar_plot = compare.polar_cyl_before().polar_plot();
    scatter_plot.show();
    polar_plot.show();
}
