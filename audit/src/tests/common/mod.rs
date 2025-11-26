use std::env;
use std::sync::LazyLock;

use dotenvy::dotenv;
use gel_tokio::Client;
use gel_tokio::create_client;
use mailgun_rs::Mailgun;

use crate::components::insert_surgeon_case;
use crate::mail::EmailSender;
use crate::mail::Mailer;
use crate::mock::gen_mocks;
use crate::model::Email;
use crate::model::SurgeonCase;

pub struct TestJwt {
    surgeon: String,
    cohort: String,
}

pub static TEST_JWTS: LazyLock<TestJwt> = LazyLock::new(|| {
    dotenv().ok();

    let surgeon = env::var("SURGEON_TEST_JWT")
        .expect("expected SURGEON_TEST_JWT environment variable to be present");

    let cohort = env::var("COHORT_TEST_JWT")
        .expect("expected COHORT_TEST_JWT environment variable to be present");

    TestJwt { surgeon, cohort }
});

pub async fn test_db() -> Client {
    let jwt = &*TEST_JWTS.surgeon;

    create_client()
        .await
        .unwrap()
        .with_globals_fn(|client| client.set("ext::auth::client_token", jwt))
}

/// Add 110 mock cases to a test branch of the DB. The first 10 cases use a JWT representing
/// the currently logged-in [`Surgeon`], and the other 100 cases use a different JWT that
/// generically represents the rest of the comparison cohort.
#[expect(unused)]
pub async fn populate_test_db() -> Client {
    // TODO: consider shell command to check current DB branch and switch it to testdb.
    //
    // - call `gel branch current` and get the output
    // - check that the output contains `testdb` or whatever unique string you choose
    // - if it doesn't contain that, call `gel branch switch testdb` or similar

    let surgeon_client = test_db()
        .await
        .with_globals_fn(|client| client.set("ext::auth::client_token", &*TEST_JWTS.surgeon));

    let surgeon_mock_cases = gen_mocks::<SurgeonCase>(10);

    // This is much slower than doing a bulk insert from JSON, but it avoids maxing out connections
    // to the [`gel_tokio::Client`] with a non-blocking iterator. At some point, we can create a
    // dedicated version of the insert query for test setup with JSON.
    for case in surgeon_mock_cases {
        insert_surgeon_case(&surgeon_client, case).await.unwrap();
    }

    // let cohort_client = surgeon_client
    //     .with_globals_fn(|client| client.set("ext::auth::client_token", &*TEST_JWT.1));
    //
    // let cohort_mock_cases = gen_mocks::<SurgeonCase>(100);
    //
    // for case in cohort_mock_cases {
    //     insert_surgeon_case(&cohort_client, case).await.unwrap();
    // }

    surgeon_client
}

#[expect(unused)]
pub async fn drop_test_db(client: &Client, branch: &str) {
    // Add some safety checks so we don't drop prod.
    assert!(branch.contains("testdb"));
    assert!(branch.len() == 10);

    client
        .execute(format!("drop branch {branch};"), &())
        .await
        .unwrap();
}

pub async fn test_mailer() -> Mailer {
    dotenv().ok();

    let api_key = env::var("MAILGUN_API_KEY")
        .expect("expected MAILGUN_API_KEY environment variable to be present");

    let domain = env::var("MAILGUN_DOMAIN")
        .expect("expected MAILGUN_DOMAIN environment variable to be present");

    Mailer {
        sender: EmailSender {
            name: "Test EmailSender".to_string(),
            email: Email::new("no-reply@test.com").unwrap(),
        },

        mailgun: Mailgun { api_key, domain },
    }
}
