#[cfg(feature = "ssr")] use crate::state::AppState;
use crate::{
    state::StatePoisonedError,
    surgeon::{Email, FormSurgeon, Surgeon},
};
#[cfg(feature = "ssr")] use gel_tokio::Client;
use leptos::prelude::{ServerFnError, expect_context, server};
#[cfg(feature = "ssr")] use leptos_axum::{ResponseOptions, redirect};
use thiserror::Error;

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

/// Handles the case where an inserted value is an [`Option`] containing a quoted [`String`]. If
/// the value is `None`, we only interpolate our `{}` with a single set of quotes, as this would be
/// unquoted in EdgeQL,  but if the value is `Some("string")`, we double the quotes, because the
/// value must remain quoted in EdgeQL after interpolation.
fn some_or_empty(value: Option<String>) -> String {
    value.map_or("{}".to_string(), |s| format!(r#""{s}""#))
}

/// Takes a value in whole diopters (D) and returns an integer value of centidiopters for storing
/// in the database.
fn to_cd(diopters: f32) -> i32 {
    (diopters * 100.0) as i32
}

/// Takes an integer value of centidiopters from the database and returns a float representing the
/// value in diopters (D).
fn to_d(centidiopters: i32) -> f32 {
    (centidiopters as f32) / 100.0
}

#[cfg(feature = "ssr")]
pub async fn db() -> Result<Client, DbError> {
    let client = expect_context::<AppState>()
        .db
        .get_cloned()
        .map_err(|err| DbError::State(StatePoisonedError(format!("{err:?}"))))?;

    Ok(client)
}

#[server]
pub async fn is_signed_in() -> Result<bool, ServerFnError> {
    let auth_token = if let Some(auth_token) = expect_context::<ResponseOptions>()
        .0
        .read()
        .headers
        .get("gel-auth-token")
    {
        auth_token.to_str().unwrap_or_default().to_string()
    } else {
        redirect("/signin");
        "redirecting...".to_string()
    };

    let query = format!(r#"select "{auth_token}" = (select global ext::auth::client_token);"#);

    let has_auth_token = db()
        .await?
        .query_required_single::<bool, _>(query, &())
        .await
        // unwrap_or_default() works here, but for such an important check, be explicit.
        .unwrap_or(false);

    Ok(has_auth_token)
}

#[server]
pub async fn get_authorized_surgeon() -> Result<Option<Surgeon>, ServerFnError> {
    let auth_token = if let Some(auth_token) = expect_context::<ResponseOptions>()
        .0
        .read()
        .headers
        .get("gel-auth-token")
    {
        auth_token.to_str().unwrap_or_default().to_string()
    } else {
        redirect("/signin");
        "redirecting...".to_string()
    };

    // In this query, `signed_in` returns a bool that tells us whether the JWT in the
    // `gel-auth-token` cookie matches the JWT stored as a global on the DB client. This is our
    // first check that nothing is fundamentally wrong with the session.
    //
    // Then we check the `Identity` that matches that JWT, which is computed and stored as the
    // global `ext::auth::ClientTokenIdentity`. If there is a `Surgeon` with the same identity, then
    // we return the `Surgeon` object from the DB, so the frontend can share it as context. We also
    // set the `surgeon` value in global server state to the returned `Surgeon`.
    //
    // If there isn't a matching `Surgeon`, then the surgeon still needs to complete the signup
    // flow. We just return an empty set, and respond to that on the frontend with a redirect to
    // the signup form and then the terms.
    let surgeon_query = format!(
        r#"
with
    signed_in := (select global ext::auth::client_token = "{auth_token}"),
    identity := (select global ext::auth::ClientTokenIdentity),

    QuerySurgeon := (select Surgeon {{
        email,
        terms,
        first_name,
        last_name,
        default_site: {{ name }},
        sia: {{
            right: {{ power, axis }},
            left: {{ power, axis }}
        }}
    }} filter .identity = identity)

select QuerySurgeon if signed_in = true else {{}};
        "#
    );

    let client = db().await?;

    let surgeon = client
        .query_required_single::<Option<Surgeon>, _>(surgeon_query, &())
        .await?;

    if surgeon.is_some() {
        expect_context::<AppState>().surgeon.set(surgeon.clone())?;
        Ok(surgeon)
    } else {
        let is_signed_in = client
            .query_required_single::<bool, _>(
                format!(r#"select global ext::auth::client_token = "{auth_token}""#),
                &(),
            )
            .await?;

        if is_signed_in {
            redirect("/signup");
            Ok(None)
        } else {
            redirect("/signin");
            Ok(None)
        }
    }
}

#[server]
pub async fn insert_surgeon(surgeon: FormSurgeon) -> Result<(), ServerFnError> {
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
        some_or_empty(first_name),
        some_or_empty(last_name),
        some_or_empty(default_site),
    );

    let (sia_right_power, sia_left_power) = (to_cd(sia_right_power), to_cd(sia_left_power));

    let query = format!(
        r#"
with QuerySurgeon := (
    insert Surgeon {{
        identity := (select global ext::auth::ClientTokenIdentity),
        email := "{email}",
        first_name := {first_name},
        last_name := {last_name},

        default_site := (select(insert Site {{
            name := {default_site} 
        }} unless conflict on .name else (select Site))),

        sia := (select(insert SurgeonSia {{
            right := (select(insert Sia {{
                power := {sia_right_power}, axis := {sia_right_axis}
            }})),
            left := (select(insert Sia {{
                power := {sia_left_power}, axis := {sia_left_axis}
            }}))
        }}))
    }} unless conflict on .email else (select Surgeon)
)
select QuerySurgeon {{
    email,
    terms,
    first_name,
    last_name,
    default_site: {{ name }},
    sia: {{
        right: {{ power, axis }},
        left: {{ power, axis }}
    }}
}};
        "#
    );

    let surgeon = db()
        .await?
        .query_required_single::<Surgeon, _>(query, &())
        .await?;
    dbg!(&surgeon);

    // todo: add the surgeon to global state here, and then load it as a resource and provide it in
    // the client

    Ok(())
}

#[cfg(test)]
mod tests {}
