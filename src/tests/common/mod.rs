use crate::{
    components::insert_surgeon_case,
    mock::{Mock, random_string},
    model::{Surgeon, SurgeonCase},
};
use dotenvy::dotenv;
use futures::{StreamExt, future::join_all, stream::FuturesOrdered};
use gel_tokio::{Client, Config, create_client};
use std::{env, sync::LazyLock};

// note: new API for dotenvy will arrive in v16 release
pub static TEST_JWT: LazyLock<(String, String)> = LazyLock::new(|| {
    dotenvy::from_filename(".test.env").ok();

    let test_jwt_surgeon = env::var("TEST_JWT_SURGEON")
        .expect("expected TEST_JWT_SURGEON environment variable to be present");

    let test_jwt_cohort = env::var("TEST_JWT_COHORT")
        .expect("expected TEST_JWT_COHORT environment variable to be present");

    (test_jwt_surgeon, test_jwt_cohort)
});

pub async fn test_db() -> Client {
    let jwt = &*TEST_JWT.0;

    create_client()
        .await
        .unwrap()
        .with_globals_fn(|client| client.set("ext::auth::client_token", jwt))
}

/// Add 110 mock cases to a temporary branch of the DB. The first 10 cases use a JWT representing
/// the currently logged-in [`Surgeon`], and the other 100 cases use a different JWT that
/// generically represents the rest of the comparison cohort.
pub async fn populate_test_db() -> Client {
    // let rs = random_string(4);
    // let branch = format!("testdb{rs}");
    // let query = format!(r#"create schema branch {branch} from main;"#);
    // let query = format!(r#"create data branch {branch} from testdbbase;"#);
    // let naive_client = create_client().await.unwrap();
    // naive_client.execute(query, &()).await.unwrap();

    // naive_client
    //     .execute(r#"create data branch testdbactive from testdbbase;"#, &())
    //     .await
    //     .unwrap();

    // let config = Config::default().with_branch(&branch);
    // let config = Config::default().with_branch("testdbactive");
    // let base_client = Client::new(&config);

    let base_client = create_client().await.unwrap();

    let x = base_client.ensure_connected().await;
    dbg!(x);

    let surgeon_client =
        base_client.with_globals_fn(|client| client.set("ext::auth::client_token", &*TEST_JWT.0));

    let x = surgeon_client.ensure_connected().await;
    dbg!(x);

    let cohort_client =
        base_client.with_globals_fn(|client| client.set("ext::auth::client_token", &*TEST_JWT.1));

    let surgeon_mock_cases = (0..=9)
        .map(|_| SurgeonCase::mock())
        .collect::<Vec<SurgeonCase>>();
    // dbg!(&surgeon_mock_cases);

    let cohort_mock_cases = (0..=9)
        .map(|_| SurgeonCase::mock())
        .collect::<Vec<SurgeonCase>>();

    // just test a single case to see if the client works before attempting to iterate.
    insert_surgeon_case(&surgeon_client, SurgeonCase::mock())
        .await
        .unwrap();

    // // bookmark: todo: the DB branch in question doesn't have any [`Surgeon`]s in it yet. So you
    // // need to create the Identity objects, as well as the Surgeon s before you can insert cases
    // for // those Surgeon s.
    // let _insert_surgeon_cases = join_all(
    //     surgeon_mock_cases
    //         .into_iter()
    //         .map(async |sc| insert_surgeon_case(&surgeon_client, sc).await),
    // )
    // .await;
    // dbg!(_insert_surgeon_cases);
    //
    // let _insert_cohort_cases = join_all(
    //     cohort_mock_cases
    //         .into_iter()
    //         .map(async |sc| insert_surgeon_case(&cohort_client, sc).await),
    // )
    // .await;
    // dbg!(_insert_cohort_cases);

    surgeon_client
}

pub async fn drop_test_db<T: AsRef<str>>(client: &Client, branch: T) {
    // Add some safety checks so we don't drop prod.
    assert!(branch.as_ref().contains("testdb"));
    assert!(branch.as_ref().len() == 10);

    // Maybe we want to just get the branch off the client, and just pass the Client only to the
    // function.

    client
        .execute(format!("drop branch {}", branch.as_ref()), &())
        .await
        .unwrap();
}
