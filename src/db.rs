#[cfg(feature = "ssr")] use crate::state::AppState;
use crate::{
    state::StatePoisonedError,
    surgeon::{Email, FormSurgeon, QuerySurgeon, Surgeon},
};
#[cfg(feature = "ssr")] use gel_protocol::value::Value;
use leptos::prelude::{ServerFnError, StorageAccess, expect_context, server};
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

/// Handles the case where an inserted value is an [`Option`] containing a quoted [`String`]. If
/// the value is `None`, we only interpolate our `{}` with a single set of quotes, as this would be
/// unquoted in EdgeQL,  but if the value is `Some("string")`, we double the quotes, because the
/// value must remain quoted in EdgeQL after interpolation.
fn some_or_empty(value: Option<String>) -> String {
    value.map_or("{}".to_string(), |s| format!(r#""{s}""#))
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

    let first_name = some_or_empty(first_name);
    let last_name = some_or_empty(last_name);
    let default_site = some_or_empty(default_site);

    let (sia_right_power, sia_left_power) = (
        (sia_right_power * 100.0) as i32,
        (sia_left_power * 100.0) as i32,
    );

    let query = format!(
        r#"with QuerySurgeon := (
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
        }};"#
    );

    let client = expect_context::<AppState>().db.get_cloned()?;

    let value = client
        .query_required_single::<QuerySurgeon, _>(query, &())
        .await?; // remove `?` to get the details of the error
    dbg!(&value);

    Ok(())
}

#[cfg(test)]
mod tests {}
