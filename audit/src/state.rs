use std::sync::Arc;

use axum_login::AuthnBackend;
use axum_macros::FromRef;
use chrono::DateTime;
use chrono::Utc;
use http::header::AUTHORIZATION;
use http::header::USER_AGENT;
use leptos::prelude::LeptosOptions;
use oauth2::AuthorizationCode;
use oauth2::CsrfToken;
use oauth2::TokenResponse;
use oauth2::url::Url;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::query_as;

use crate::auth::BasicClientSet;
use crate::auth::Credentials;
use crate::auth::UserInfo;
use crate::db::db;
use crate::error::AppError;
use crate::mail::Mailer;
use crate::model::Axis;
use crate::model::Email;
use crate::model::Focus;
use crate::model::Formula;
use crate::model::Main;
use crate::model::QuerySurgeon;
use crate::model::SiaPower;
use crate::model::Surgeon;
use crate::model::ToricPower;

// `derive(FromRef)` is needed to make use of `leptos_axum`'s `extract_with_state()`
// Some fields don't need to be wrapped in `Arc` because they use `Arc` internally.
#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub oauth_client: Arc<BasicClientSet>,
    pub http_client: oauth2::reqwest::Client,
    pub db: Pool<Postgres>,
    pub leptos_options: LeptosOptions,
    pub mailer: Arc<Mailer>,
    // TODO: the Surgeon was removed here, because they are available via an axum extractor on
    // AuthSession::inner.user, and we want a single source of truth, so we will need to send the
    // surgeon back to the frontend and make it available via context somewhere in the protected/
    // route
}

impl AuthnBackend for AppState {
    type Credentials = Credentials;
    type Error = AppError;
    type User = Surgeon;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // Ensure that the CSRF state has not been tampered with
        if creds.old_state.secret() != creds.new_state.secret() {
            return Ok(None);
        };

        // Use the code to get a token
        let access_token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(creds.code))
            .request_async(&self.http_client)
            .await?
            .access_token()
            .clone()
            .into_secret();

        let google = self
            .http_client
            .get("https://openidconnect.googleapis.com/v1/userinfo")
            .header(USER_AGENT.as_str(), "axum-login")
            .header(AUTHORIZATION.as_str(), format!("Bearer {access_token}"))
            .send()
            .await?
            .json::<UserInfo>()
            .await?
            .email;

        let surgeon: Surgeon = query_as!(
            QuerySurgeon,
            r#"
with updated_surgeon as (
    update surgeon
    set access_token = $1
    where google = $2

    returning
        id,
        default_site_id,
        default_iol_id,
        terms,
        default_formula,
        default_main,
        default_sia_power,
        default_sia_axis_right,
        default_sia_axis_left,
        default_custom_constant,
        access_token,
        email,
        google,
        full_name,
        preferred_name
)

select
    s.id,
    s.access_token,

    s.email as "email: Email",
    s.google as "google: Email",
    s.terms as "terms: DateTime<Utc>",
    s.full_name,
    s.preferred_name,

    t.name as default_site_name,
    
    i.model as default_iol_model,
    i.name as default_iol_name,
    i.company as default_iol_company,
    i.focus as "default_iol_focus: Focus",
    i.toric as "default_iol_toric: ToricPower",
    
    s.default_formula as "default_formula: Formula",
    s.default_custom_constant,
    s.default_main as "default_main: Main",

    s.default_sia_power as "default_sia_power: SiaPower",
    s.default_sia_axis_right as "default_sia_axis_right: Axis",
    s.default_sia_axis_left as "default_sia_axis_left: Axis"

from updated_surgeon s
left join site t on t.id = s.default_site_id
left join iol i on i.id = s.default_iol_id
limit 1;
            "#,
            access_token,
            google as Email
        )
        .fetch_one(&db().await?)
        .await?
        .into();

        Ok(Some(surgeon))
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let surgeon: Option<Surgeon> = query_as!(
            QuerySurgeon,
            r#"
select
    s.id,
    s.access_token,

    s.email as "email: Email",
    s.google as "google: Email",
    s.terms as "terms: DateTime<Utc>",
    s.full_name,
    s.preferred_name,

    t.name as default_site_name,
    
    i.model as default_iol_model,
    i.name as default_iol_name,
    i.company as default_iol_company,
    i.focus as "default_iol_focus: Focus",
    i.toric as "default_iol_toric: ToricPower",
    
    s.default_formula as "default_formula: Formula",
    s.default_custom_constant,
    s.default_main as "default_main: Main",

    s.default_sia_power as "default_sia_power: SiaPower",
    s.default_sia_axis_right as "default_sia_axis_right: Axis",
    s.default_sia_axis_left as "default_sia_axis_left: Axis"

from surgeon s
left join site t on t.id = s.default_site_id
left join iol i on i.id = s.default_iol_id
where s.id = $1
limit 1;
            "#,
            user_id
        )
        .fetch_optional(&db().await?)
        .await?
        .map(|qs| qs.into());

        Ok(surgeon)
    }
}

impl AppState {
    pub fn builder() -> AppStateBuilder { AppStateBuilder::default() }

    pub fn authorize_url(&self) -> (Url, CsrfToken) {
        Arc::clone(&self.oauth_client)
            .authorize_url(CsrfToken::new_random)
            .url()
    }
}

#[derive(Debug, Default)]
pub struct AppStateBuilder {
    oauth_client: Option<BasicClientSet>,
    db: Option<Pool<Postgres>>,
    http_client: Option<oauth2::reqwest::Client>,
    leptos_options: Option<LeptosOptions>,
    mailer: Option<Mailer>,
    surgeon: Option<Surgeon>,
}

impl AppStateBuilder {
    pub fn oauth_client(mut self, oauth_client: BasicClientSet) -> Self {
        self.oauth_client = Some(oauth_client);

        self
    }

    pub fn db(mut self, db: Pool<Postgres>) -> Self {
        self.db = Some(db);

        self
    }

    pub fn http_client(mut self, http_client: oauth2::reqwest::Client) -> Self {
        self.http_client = Some(http_client);

        self
    }

    pub fn leptos_options(mut self, leptos_options: LeptosOptions) -> Self {
        self.leptos_options = Some(leptos_options);

        self
    }

    pub fn mailer(mut self, mailer: Mailer) -> Self {
        self.mailer = Some(mailer);

        self
    }

    pub fn surgeon(mut self, surgeon: Surgeon) -> Self {
        self.surgeon = Some(surgeon);

        self
    }

    pub fn build(self) -> Result<AppState, AppError> {
        if let Self {
            oauth_client: Some(oauth_client),
            http_client: Some(http_client),
            db: Some(db),
            leptos_options: Some(leptos_options),
            mailer: Some(mailer),
            surgeon,
        } = self
        {
            let state = AppState {
                oauth_client: Arc::new(oauth_client),
                http_client,
                db,
                leptos_options,
                mailer: Arc::new(mailer),
            };

            Ok(state)
        } else {
            Err(AppError::State(format!(
                r#"
some required fields in `AppState` have not been set - the supplied fields were:

oauth_client: {:?}

http_client: {:?}

db: {:?}

leptos_options: {:?}

mailer: {:?}
                "#,
                self.oauth_client, self.http_client, self.db, self.leptos_options, self.mailer,
            )))
        }
    }
}
