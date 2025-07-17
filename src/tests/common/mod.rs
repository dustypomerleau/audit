use dotenvy::dotenv;
use gel_tokio::Client;
use std::{env, sync::LazyLock};

// note: new API for dotenvy will arrive in v16 release
pub static TEST_JWT: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    env::var("TEST_JWT").expect("expected TEST_JWT environment variable to be present")
});

pub async fn test_db() -> Client {
    let jwt = &*TEST_JWT;

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
pub async fn populate_test_db() {}

pub async fn clear_test_db() {}
