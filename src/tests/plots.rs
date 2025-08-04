use crate::tests::common::populate_test_db;

#[tokio::test]
async fn makes_a_plot() {
    let client = populate_test_db().await;
    assert!(client.ensure_connected().await.is_ok());
}
