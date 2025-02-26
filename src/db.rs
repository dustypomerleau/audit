use crate::{
    sia::Sia,
    state::StatePoisonedError,
    surgeon::{Surgeon, SurgeonSia},
};
use axum::extract::State;
use gel_tokio::Client;
use std::sync::{Arc, RwLock};
use thiserror::Error;

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

pub async fn upsert_surgeon(
    id: &str,
    State(db): State<Arc<RwLock<Client>>>,
) -> Result<Surgeon, DbError> {
    let client = db
        .get_cloned()
        .map_err(|err| StatePoisonedError(format!("{err:?}")))?;

    let query = format!("");
    let surgeon: Surgeon = client.query_required_single(query, &()).await?;

    Ok(surgeon)
}

// todo: you probably want to create the Surgeon with only the email and identity, and then
// after creating it, offer a form view to add the name, site, SIA.

// probably a good place for a macro...
pub async fn insert_surgeon(
    Surgeon {
        email,
        first_name,
        last_name,
        default_site,
        sia,
    }: Surgeon,
) -> Result<(), gel_tokio::Error> {
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

    let query = format!(
        "insert Surgeon {{
            email := {email},  
            first_name := {first_name},
            last_name := {last_name},
            default_site := {default_site},
            sia := {sia}
        }} unless conflict on .email;"
    );

    // create the client and execute the query

    Ok(())
}

#[cfg(test)]
mod tests {}
