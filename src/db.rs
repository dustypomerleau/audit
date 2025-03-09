use crate::{
    auth::{AuthError, AuthToken},
    sia::Sia,
    state::{AppState, StatePoisonedError},
    surgeon::{Surgeon, SurgeonSia},
};
use axum::response::IntoResponse;
use leptos::prelude::{ServerFnError, expect_context, server};
use leptos_axum::ResponseOptions;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Gel error: {0:?}")]
    Gel(gel_tokio::Error),
    #[error("The DB operation couldn't be completed due to poisoned state: {0:?}")]
    State(StatePoisonedError),
}

impl From<gel_tokio::Error> for DbError {
    fn from(err: gel_tokio::Error) -> Self {
        Self::Gel(err)
    }
}

impl From<StatePoisonedError> for DbError {
    fn from(err: StatePoisonedError) -> Self {
        Self::State(err)
    }
}

#[server]
pub async fn insert_surgeon(surgeon: Surgeon) -> Result<Uuid, ServerFnError> {
    let Surgeon {
        email,
        first_name,
        last_name,
        default_site,
        sia,
        ..
    } = surgeon;

    let (first_name, last_name, default_site) = (
        first_name.unwrap_or("{}".to_string()),
        last_name.unwrap_or("{}".to_string()),
        default_site.unwrap_or("{}".to_string()),
    );

    let sia = match sia {
        Some(SurgeonSia {
            right:
                Sia {
                    power: right_power,
                    axis: right_axis,
                },
            left:
                Sia {
                    power: left_power,
                    axis: left_axis,
                },
        }) => {
            format!(
                "(select (insert SurgeonSia {{
                    right := (select (insert Sia {{ power := {right_power}, axis := {right_axis} }} )),
                    left := (select (insert Sia {{ power := {left_power}, axis := {left_axis} }} ))
                }} ))"
            )
        }

        None => "{}".to_string(),
    };

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
            identity := {identity},
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
    Ok(surgeon_id)
}

#[cfg(test)]
mod tests {}
