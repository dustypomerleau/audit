use std::env;
use std::sync::Arc;

use dotenvy::dotenv;

use crate::mail::EmailType;
use crate::mail::transactional_email_with_mailer;
use crate::mock::Mock;
use crate::model::Email;
use crate::model::Surgeon;
use crate::tests::common::test_mailer;

// We ignore this test by default, to avoid developing a spam reputation for our domain.
#[tokio::test]
#[ignore]
async fn test_email() {
    dotenv().ok();

    let mut surgeon = Surgeon::mock();

    let email_recipient = env::var("TEST_EMAIL_RECIPIENT")
        .expect("expected TEST_EMAIL_RECIPIENT environment variable to be present");

    surgeon.email = Email::new(&email_recipient).unwrap();
    let mailer = Arc::new(test_mailer().await);

    transactional_email_with_mailer(&surgeon, EmailType::Welcome, mailer)
        .await
        .unwrap();
}
