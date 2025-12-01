use std::env;
use std::sync::Arc;
use std::sync::LazyLock;

use leptos::prelude::use_context;
use mailgun_rs::Attachment;
use mailgun_rs::EmailAddress;
use mailgun_rs::Mailgun;
use mailgun_rs::MailgunRegion;
use mailgun_rs::Message;
use mailgun_rs::SendResponse;

use crate::error::AppError;
use crate::model::Email;
use crate::model::Surgeon;
use crate::state::AppState;

#[derive(Debug)]
pub struct Mailer {
    pub sender: EmailSender,
    pub mailgun: Mailgun,
}

// This impl is necessary because the [`Mailgun`] foreign type is not [`Clone`].
impl Clone for Mailer {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            mailgun: Mailgun {
                api_key: self.mailgun.api_key.clone(),
                domain: self.mailgun.domain.clone(),
            },
        }
    }
}

pub fn mailer() -> Result<Arc<Mailer>, AppError> {
    if let Some(AppState { mailer, .. }) = use_context::<AppState>() {
        Ok(Arc::clone(&mailer))
    } else {
        Err(AppError::State(
            "unable to get the mailer from context".to_string(),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct EmailSender {
    pub name: String,
    pub email: Email,
}

impl From<EmailSender> for EmailAddress {
    fn from(sender: EmailSender) -> Self {
        EmailAddress::builder()
            .name(Some(sender.name))
            .address(sender.email.inner())
            .build()
    }
}

pub static MAILER: LazyLock<Mailer> = LazyLock::new(|| {
    let api_key = env::var("MAILGUN_API_KEY")
        .expect("expected MAILGUN_API_KEY environment variable to be present");

    let domain = env::var("MAILGUN_DOMAIN")
        .expect("expected MAILGUN_DOMAIN environment variable to be present");

    let sender_name = env::var("MAILGUN_SENDER_NAME")
        .expect("expected MAILGUN_SENDER_NAME environment variable to be present");

    let sender_email: Email = env::var("MAILGUN_SENDER_EMAIL")
        .expect("expected MAILGUN_SENDER_EMAIL environment variable to be present")
        .try_into()
        .expect("expected the MAILGUN_SENDER_EMAIL environment variable to be a valid email");

    Mailer {
        sender: EmailSender {
            name: sender_name,
            email: sender_email,
        },
        mailgun: Mailgun { api_key, domain },
    }
});

pub enum EmailType {
    Welcome,
}

pub fn email_sign_up(surgeon: &Surgeon) -> Message {
    let name = if let Some(full_name) = surgeon.full_name.clone() {
        full_name
    } else {
        surgeon.email.inner()
    };

    Message::builder()
        .to(vec![
            EmailAddress::builder()
                .name(surgeon.full_name.clone())
                .address(surgeon.email.inner())
                .build(),
        ])
        .subject("the subject")
        .text(format!("Thanks for signing up, {name}!",))
        .build()
}

pub struct MailParts {
    region: MailgunRegion,
    sender: Email,
    message: Message,
    attachments: Option<Vec<Attachment>>,
}

impl Default for MailParts {
    fn default() -> Self {
        Self {
            region: MailgunRegion::US,
            sender: Email::new("no-reply@viceye.au").unwrap(),
            message: Default::default(),
            attachments: None,
        }
    }
}

impl MailParts {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn region(mut self, region: MailgunRegion) -> Self {
        self.region = region;

        self
    }

    pub fn sender(mut self, sender: Email) -> Self {
        self.sender = sender;

        self
    }

    pub fn message(mut self, message: Message) -> Self {
        self.message = message;

        self
    }

    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        self.attachments = Some(attachments);

        self
    }
}

pub async fn transactional_email(
    surgeon: &Surgeon,
    email_type: EmailType,
) -> Result<SendResponse, AppError> {
    let mailer = mailer()?;

    transactional_email_with_mailer(surgeon, email_type, mailer).await
}

// Factoring out this function provides a way to supply our own [`Mailer`] in tests. In prod, it is
// assumed that our main function will run and the Mailer will be retrieved from the [`AppState`] in
// context.
#[doc(hidden)]
pub(crate) async fn transactional_email_with_mailer(
    surgeon: &Surgeon,
    email_type: EmailType,
    mailer: Arc<Mailer>,
) -> Result<SendResponse, AppError> {
    let message = match email_type {
        EmailType::Welcome => email_sign_up(surgeon),
    };

    let sender: EmailAddress = mailer.sender.clone().into();

    let result = mailer
        .mailgun
        .async_send(MailgunRegion::US, &sender, message, None)
        .await;

    match result {
        Ok(response) => Ok(response),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {}
