use std::sync::Arc;
use std::sync::RwLock;

use axum_macros::FromRef;
use leptos::prelude::LeptosOptions;
use sqlx::Pool;
use sqlx::Postgres;

use crate::mail::Mailer;
use crate::model::Surgeon;

// `derive(FromRef)` is needed to make use of `leptos_axum`'s `extract_with_state()`
#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: Pool<Postgres>,
    pub mailer: Arc<Mailer>,
    pub surgeon: Arc<RwLock<Option<Surgeon>>>,
}
