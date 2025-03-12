#[cfg(feature = "ssr")] use crate::auth::AuthToken;
#[cfg(feature = "ssr")] use crate::state::AppState;
use crate::{
    sia::Sia,
    state::StatePoisonedError,
    surgeon::{Email, FormSurgeon, SurgeonSia},
};
use leptos::prelude::{ServerFnError, expect_context, server};
#[cfg(feature = "ssr")] use leptos_axum::ResponseOptions;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
#[cfg(feature = "ssr")]
pub enum DbError {
    #[error("Gel error: {0:?}")]
    Gel(gel_tokio::Error),
    #[error("The DB operation couldn't be completed due to poisoned state: {0:?}")]
    State(StatePoisonedError),
}

#[cfg(feature = "ssr")]
impl From<gel_tokio::Error> for DbError {
    fn from(err: gel_tokio::Error) -> Self {
        Self::Gel(err)
    }
}

#[cfg(feature = "ssr")]
impl From<StatePoisonedError> for DbError {
    fn from(err: StatePoisonedError) -> Self {
        Self::State(err)
    }
}

#[server]
pub async fn insert_surgeon(surgeon: FormSurgeon) -> Result<Uuid, ServerFnError> {
    let FormSurgeon {
        email,
        first_name,
        last_name,
        default_site,
        sia_right_power,
        sia_right_axis,
        sia_left_power,
        sia_left_axis,
    } = surgeon;

    let email = Email::new(&email)?.inner();

    let (first_name, last_name, default_site) = (
        first_name.unwrap_or("{}".to_string()),
        last_name.unwrap_or("{}".to_string()),
        default_site.unwrap_or("{}".to_string()),
    );

    let (sia_right_power, sia_left_power) = (
        (sia_right_power * 100.0) as i32,
        (sia_left_power * 100.0) as i32,
    );

    let sia = format!(
        "(select (insert SurgeonSia {{ right := (select (insert Sia {{ power := {sia_right_power}, axis := {sia_right_axis} }} )), left := (select (insert Sia {{ power := {sia_left_power}, axis := {sia_left_axis} }} )) }} ))"
    );

    let identity = if let Some(header) = expect_context::<ResponseOptions>()
        .0
        .read()
        .headers
        .get("gel-auth-token")
    {
        let auth_token: AuthToken = serde_json::from_str(header.to_str()?)?;
        &auth_token.identity_id.to_string()
    } else {
        return Err(ServerFnError::Deserialization(
            "unable to get the auth token from the cookie".to_string(),
        ));
    };

    let query = format!(
        "insert Surgeon {{
            identity := {identity}
            email := {email},  
            first_name := {first_name},
            last_name := {last_name},
            default_site := {default_site},
            sia := {sia}
        }} unless conflict on .email;"
    );

    let client = expect_context::<AppState>().db.get_cloned()?;
    // todo: handle an error on the insert immediately, rather than bubbling it up.
    // The main reason for failure would be a duplicate email.
    let surgeon_id = client.query_required_single::<Uuid, _>(query, &()).await?;
    dbg!(surgeon_id);
    Ok(surgeon_id)
}

#[cfg(test)]
mod tests {}
