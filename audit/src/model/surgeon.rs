use std::fmt::Debug;

use axum_login::AuthUser;
use chrono::DateTime;
use chrono::Utc;
use garde::Validate;
use leptos::prelude::ServerFnError;
use leptos::prelude::server;
#[cfg(feature = "ssr")] use leptos::prelude::use_context;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::Axis;
use crate::model::Focus;
use crate::model::Formula;
use crate::model::Iol;
use crate::model::Main;
use crate::model::Sia;
use crate::model::SiaPower;
use crate::model::ToricPower;
#[cfg(feature = "ssr")] use crate::state::AppState;

/// A [`garde`]-checked valid email [`String`].
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Validate)]
#[garde(transparent)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Email(#[garde(email)] String);

impl TryFrom<String> for Email {
    type Error = AppError;

    fn try_from(email: String) -> Result<Self, Self::Error> { Email::new(email.as_str()) }
}

impl Email {
    pub fn new(email: &str) -> Result<Self, AppError> {
        let email = Self(email.to_string());

        match email.validate() {
            Ok(_) => Ok(email),
            Err(e) => Err(AppError::Bounds(format!("invalid email: {e}"))),
        }
    }

    pub fn inner(&self) -> String { self.0.clone() }

    pub fn into_inner(self) -> String { self.0 }
}

/// A surgeon's default [`Sia`] for right and left eyes
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SurgeonSia {
    pub right: Sia,
    pub left: Sia,
}

/// A proto-[`Surgeon`] representing the surgeon's form input at sign-up.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormSurgeon {
    pub email: String,
    pub full_name: Option<String>,
    pub preferred_name: Option<String>,
    pub default_site: Option<String>,
    pub default_iol: Option<String>,
    pub default_formula: Option<String>,
    pub custom_constant: Option<String>,
    pub main: f32,
    pub sia_power: f32,
    pub sia_right_axis: i32,
    pub sia_left_axis: i32,
}

#[cfg(feature = "ssr")]
pub struct QuerySurgeon {
    pub id: Uuid,
    pub access_token: Option<String>,

    pub email: Email,
    pub google: Email,
    pub terms: Option<DateTime<Utc>>,
    pub full_name: Option<String>,
    pub preferred_name: Option<String>,

    pub default_site_name: Option<String>,

    pub default_iol_model: Option<String>,
    pub default_iol_name: Option<String>,
    pub default_iol_company: Option<String>,
    pub default_iol_focus: Option<Focus>,
    pub default_iol_toric: Option<ToricPower>,

    pub default_formula: Option<Formula>,
    pub default_custom_constant: bool,
    pub default_main: Main,

    pub default_sia_power: SiaPower,
    pub default_sia_axis_right: Axis,
    pub default_sia_axis_left: Axis,
}

impl Debug for QuerySurgeon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuerySurgeon")
            .field("access_token", &"[redacted]")
            .field("id", &"[redacted]")
            .field("email", &self.email)
            .field("google", &self.google)
            .field("terms", &self.terms)
            .field("full_name", &self.full_name)
            .field("preferred_name", &self.preferred_name)
            .field("default_site_name", &self.default_site_name)
            .field("default_iol_model", &self.default_iol_model)
            .field("default_iol_name", &self.default_iol_name)
            .field("default_iol_company", &self.default_iol_company)
            .field("default_iol_focus", &self.default_iol_focus)
            .field("default_iol_toric", &self.default_iol_toric)
            .field("default_formula", &self.default_formula)
            .field("default_custom_constant", &self.default_custom_constant)
            .field("default_main", &self.default_main)
            .field("default_sia_power", &self.default_sia_power)
            .field("default_sia_axis_right", &self.default_sia_axis_right)
            .field("default_sia_axis_left", &self.default_sia_axis_left)
            .finish()
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Site {
    pub name: String,
}

/// A unique surgeon
#[derive(Clone, Default, Deserialize, PartialEq, Serialize)]
pub struct Surgeon {
    // Never send the id to the frontend.
    #[cfg(feature = "ssr")]
    #[serde(skip_serializing)]
    pub id: Uuid,

    // Never send the access token to the frontend.
    #[cfg(feature = "ssr")]
    #[serde(skip_serializing)]
    pub access_token: String,

    pub email: Email,
    pub google: Email,
    pub terms: Option<DateTime<Utc>>,
    pub full_name: Option<String>,
    pub preferred_name: Option<String>,
    pub defaults: SurgeonDefaults,
    pub sia: SurgeonSia,
}

impl AuthUser for Surgeon {
    type Id = Uuid;

    fn id(&self) -> Self::Id { self.id }

    fn session_auth_hash(&self) -> &[u8] { self.access_token.as_bytes() }
}

impl Debug for Surgeon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Surgeon")
            .field("id", &"[redacted]")
            .field("access_token", &"[redacted]")
            .field("email", &self.email)
            .field("google", &self.google)
            .field("terms", &self.terms)
            .field("full_name", &self.full_name)
            .field("preferred_name", &self.preferred_name)
            .field("defaults", &self.defaults)
            .field("sia", &self.sia)
            .finish()
    }
}

impl From<QuerySurgeon> for Surgeon {
    fn from(qs: QuerySurgeon) -> Self {
        let QuerySurgeon {
            id,
            access_token,
            email,
            google,
            terms,
            full_name,
            preferred_name,
            default_site_name,
            default_iol_model,
            default_iol_name,
            default_iol_company,
            default_iol_focus,
            default_iol_toric,
            default_formula,
            default_custom_constant,
            default_main,
            default_sia_power,
            default_sia_axis_right,
            default_sia_axis_left,
        } = qs;

        let access_token = access_token.unwrap_or("no access token".to_string());
        let site = default_site_name.map(|name| Site { name });

        let iol = if let (Some(model), Some(focus)) = (default_iol_model, default_iol_focus) {
            Some(Iol {
                model,
                name: default_iol_name,
                company: default_iol_company,
                focus,
                toric: default_iol_toric,
            })
        } else {
            None
        };

        let defaults = SurgeonDefaults {
            site,
            iol,
            formula: default_formula,
            custom_constant: default_custom_constant,
            main: default_main,
        };

        let sia = SurgeonSia {
            right: Sia {
                power: default_sia_power,
                axis: default_sia_axis_right,
            },
            left: Sia {
                power: default_sia_power,
                axis: default_sia_axis_left,
            },
        };

        Surgeon {
            id,
            access_token,
            email,
            google,
            terms,
            full_name,
            preferred_name,
            defaults,
            sia,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SurgeonDefaults {
    pub site: Option<Site>,
    pub iol: Option<Iol>,
    pub formula: Option<Formula>,
    pub custom_constant: bool,
    pub main: Main,
}

/// Return the current [`Surgeon`] from global server context by cloning its value. In practice,
/// this function should rarely be needed, as accessing a protected route will call
/// [`get_authorized_surgeon`](crate::auth::get_authorized_surgeon), which is then provided as
/// client-side context.
#[server]
pub async fn get_current_surgeon() -> Result<Option<Surgeon>, AppError> {
    let surgeon = use_context::<AppState>()
        .ok_or_else(|| AppError::State("AppState not present in context".to_string()))?
        .surgeon
        .get_cloned()?;

    Ok(surgeon)
}

/// Set the value of the current [`Surgeon`] in global server context. using `Option<Surgeon>` as
/// the input parameter allows clearing the value by setting [`None`].
#[server]
pub async fn set_current_surgeon(surgeon: Option<Surgeon>) -> Result<(), ServerFnError> {
    use_context::<AppState>()
        .ok_or_else(|| AppError::State("AppState not present in context".to_string()))?
        .surgeon
        .set(surgeon)?;

    Ok(())
}

#[cfg(test)]
mod tests {}
