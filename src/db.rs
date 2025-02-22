use crate::{
    sia::Sia,
    surgeon::{Surgeon, SurgeonSia},
};
use axum::extract::State;
use gel_tokio::Client;
use std::sync::{Arc, RwLock};

// what if we just:
// get the id from the auth token
// create a new surgeon with the id
// check to see if that user already has an email
// if they do, populate the `surgeon` in state and redirect to `add`
// if they don't, redirect to the surgeon information form and collect their preferred email
// along with all the other details
pub async fn upsert_surgeon(
    id: &str,
    State(db): State<Arc<RwLock<Client>>>,
) -> Result<Surgeon, gel_tokio::Error> {
    // todo: create the upsert query, then call this in auth::
    let query = format!("");

    let db = Arc::clone(&db);
    let client = db.read().unwrap();
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
