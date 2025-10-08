use crate::{
    plots::{
        AsPlot, Cartesian, Cohort, PolarCompare, PolarData, PolarPoint, get_compare_with_client,
    },
    tests::common::{populate_test_db, test_db},
};

#[tokio::test]
async fn creates_plots() {
    let client = test_db().await;

    // uncomment below to populate the test DB with cases.
    // note: This is hacky:
    // - As currently written, populate_test_db() will fail if you don't already have at
    // least 2 Surgeons in the DB whose auth tokens match the test tokens in .env.
    // - It will also fail if there are no Iols in the DB, so you have to manually add at
    // least one (for some reason).
    // Write a proper setup from an empty DB.
    //
    // let client = populate_test_db().await;
    // assert!(client.ensure_connected().await.is_ok());

    let compare = get_compare_with_client(&client, 2025, Cohort::Peers)
        .await
        .unwrap();
    // let scatter_plot = compare.cartesian_delta_cyl().plot();
    let polar_plot = compare.polar_cyl_before().plot();
    // scatter_plot.show();
    polar_plot.show();
}

// junk tests to work out issues
#[test]
fn test_cartesian() {
    let polar = PolarData {
        points: vec![
            PolarPoint {
                theta: 45.0,
                r: 2.5,
            },
            PolarPoint {
                theta: 135.0,
                r: 3.5,
            },
            PolarPoint {
                theta: 225.0,
                r: 2.5,
            },
            PolarPoint {
                theta: 315.0,
                r: 4.5,
            },
        ],
    };

    let cartesian = polar.cartesian();
}

#[test]
fn test_confidence() {
    let polar = PolarData {
        points: vec![
            PolarPoint {
                theta: 45.0,
                r: 2.5,
            },
            PolarPoint {
                theta: 35.0,
                r: 3.5,
            },
            PolarPoint {
                theta: 25.0,
                r: 2.5,
            },
            PolarPoint {
                theta: 15.0,
                r: 4.5,
            },
        ],
    };

    let compare = PolarCompare {
        surgeon: polar,
        cohort: PolarData { points: vec![] },
    };

    compare.plot().show();
}
