use crate::{
    components::insert_surgeon_case,
    mock::{Mock, gen_mocks, random_string},
    model::{Surgeon, SurgeonCase},
};
use dotenvy::dotenv;
use futures::{StreamExt, executor, future::join_all, stream::FuturesOrdered};
use gel_tokio::{Client, Config, create_client};
use std::{env, sync::LazyLock};
use tokio::{runtime::Runtime, task};

pub static TEST_JWT: LazyLock<(String, String)> = LazyLock::new(|| {
    dotenv().ok();

    let surgeon_test_jwt = env::var("SURGEON_TEST_JWT")
        .expect("expected SURGEON_TEST_JWT environment variable to be present");

    let cohort_test_jwt = env::var("COHORT_TEST_JWT")
        .expect("expected COHORT_TEST_JWT environment variable to be present");

    (surgeon_test_jwt, cohort_test_jwt)
});

pub async fn test_db() -> Client {
    let jwt = &*TEST_JWT.0;

    create_client()
        .await
        .unwrap()
        .with_globals_fn(|client| client.set("ext::auth::client_token", jwt))
}

/// Add 110 mock cases to a test branch of the DB. The first 10 cases use a JWT representing
/// the currently logged-in [`Surgeon`], and the other 100 cases use a different JWT that
/// generically represents the rest of the comparison cohort.
pub async fn populate_test_db() -> Client {
    // todo: consider shell command to check current DB branch and switch it to testdb.
    //
    // - call `gel branch current` and get the output
    // - check that the output contains `testdb` or whatever unique string you choose
    // - if it doesn't contain that, call `gel branch switch testdb` or similar

    let surgeon_client = test_db()
        .await
        .with_globals_fn(|client| client.set("ext::auth::client_token", &*TEST_JWT.0));

    let surgeon_mock_cases = gen_mocks::<SurgeonCase>(10);

    // This is much slower than doing a bulk insert from JSON, but it avoids maxing out the
    // [`gel_tokio::Client`] with a non-blocking iterator, and it avoids needing a dedicated
    // version of the insert query just for test setup.
    for case in surgeon_mock_cases {
        insert_surgeon_case(&surgeon_client, case).await.unwrap();
    }

    let cohort_client = surgeon_client
        .with_globals_fn(|client| client.set("ext::auth::client_token", &*TEST_JWT.1));

    let cohort_mock_cases = gen_mocks::<SurgeonCase>(100);

    for case in cohort_mock_cases {
        insert_surgeon_case(&cohort_client, case).await.unwrap();
    }

    surgeon_client
}

pub async fn drop_test_db(client: &Client, branch: &str) {
    // Add some safety checks so we don't drop prod.
    assert!(branch.contains("testdb"));
    assert!(branch.len() == 10);

    client
        .execute(format!("drop branch {branch};"), &())
        .await
        .unwrap();
}
