#[cfg(feature = "ssr")] use crate::state::AppState;
use crate::{
    state::StatePoisonedError,
    surgeon::{Email, FormSurgeon, Surgeon},
};
use leptos::prelude::{ServerFnError, expect_context, server};
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

    let client = expect_context::<AppState>().db.get_cloned()?;

    let surgeon = client
        .query_required_single::<Surgeon, _>(query, &())
        .await?;
    dbg!(&surgeon);

    // todo: add the surgeon to global state here, and then load it as a resource and provide it in
    // the client

    Ok(())
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
        unreachable!()
    };

    let client = expect_context::<AppState>()
        .db
        .get_cloned()
        .unwrap_or(gel_tokio::create_client().await?);

    // todo: fix this query: The auth_token is a JWT and you are trying to compare it to
    // identity.id which is a uuid. What is the real check you want?
    let query = format!(
        r#"
with
    identity := (select global ext::auth::ClientTokenIdentity),
    signed_in := (select identity.id = <str>"{auth_token}"),

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

    let surgeon = client
        .query_required_single::<Option<Surgeon>, _>(query, &())
        .await?;

    if surgeon.is_some() {
        Ok(surgeon)
    } else {
        redirect("/signin");
        Ok(None)
    }
}

#[cfg(test)]
mod tests {}
