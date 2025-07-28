use crate::{
    components::insert_surgeon_case,
    mock::{Mock, random_string},
    model::{Surgeon, SurgeonCase},
};
use dotenvy::dotenv;
use gel_tokio::{Client, Config};
use std::{env, sync::LazyLock};

// note: new API for dotenvy will arrive in v16 release
pub static TEST_JWT: LazyLock<(String, String)> = LazyLock::new(|| {
    dotenv().ok();

    let jwt_0 =
        env::var("TEST_JWT_1").expect("expected TEST_JWT_1 environment variable to be present");

    let jwt_1 =
        env::var("TEST_JWT_2").expect("expected TEST_JWT_2 environment variable to be present");

    (jwt_0, jwt_1)
});

pub async fn test_db() -> Client {
    let jwt = &*TEST_JWT.0;

    gel_tokio::create_client()
        .await
        .unwrap()
        .with_globals_fn(|client| client.set("ext::auth::client_token", jwt))
}

// ext::auth::Identity {
//     modified_at: <datetime>'2025-05-17T07:11:24.768235Z',
//     created_at: <datetime>'2025-05-17T07:11:24.768230Z',
//     id: 24b12f10-32ee-11f0-8c3d-7f15b2b4bb4d,
//     issuer: 'https://accounts.google.com',
//     subject: '100301373209713435448',
//   },
//
// can be mocked with:
//
// insert ext::auth::Identity { issuer := "mock issuer", subject := "<random string>" };
//
// and then you need to use a similar query to the [`insert_surgeon`](crate::routes::insert_surgeon)
// function, but with your mock Identity instead of the global ClientTokenIdentity.

/// Adds the following to the database:
/// - 10 mock [`Surgeon`](crate::model::Surgeon)s, 1 of which matches TEST_JWT (others can have
/// random )
/// - 10 mock [`SurgeonCase`](crate::model::SurgeonCase)s for each
///   [`Surgeon`](crate::model::Surgeon)
// note: since mocking the JWT is the hard part, you could try just having one TEST_JWT and doing:
// - set the JWT on the client
// - insert 100 cases
// - modify 90 of the cases with a fake Surgeon, so that only 10 are from the logged in surgeon
#[tokio::test]
pub async fn populate_test_db() {
    let rs = random_string(8);
    let branch = format!("testdb{rs}");
    let client = gel_tokio::create_client().await.unwrap();
    let query = format!(r#"create schema branch {branch} from main"#);
    client.execute(query, &()).await.unwrap();
    let config = Config::default().with_branch(&branch);
    assert!(&config.db.database().unwrap().contains("testdb"));

    let client_0 = Client::new(&config)
        .with_globals_fn(|client| client.set("ext::auth::client_token", &*TEST_JWT.0));

    let client_1 = Client::new(&config)
        .with_globals_fn(|client| client.set("ext::auth::client_token", &*TEST_JWT.1));

    let mock_cases_0 = (0..=9)
        .map(|_| SurgeonCase::mock())
        .collect::<Vec<SurgeonCase>>();

    let mock_cases_1 = (0..=99)
        .map(|_| SurgeonCase::mock())
        .collect::<Vec<SurgeonCase>>();

    // you need futures::future::join_all() to await all of the futures you're iterating over
    let x = mock_cases_0
        .into_iter()
        .map(|sc| async { insert_surgeon_case(client_0.clone(), sc).await });
}

pub async fn clear_test_db<T: AsRef<str>>(branch: T) {}
