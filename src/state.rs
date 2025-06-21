use crate::model::Surgeon;
use axum_macros::FromRef;
use gel_tokio::Client;
use leptos::prelude::LeptosOptions;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, PoisonError, RwLock};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Error, Serialize)]
#[error("The lock is poisoned and couldn't be written or read")]
pub struct StatePoisonedError(pub String);

impl<T> From<PoisonError<T>> for StatePoisonedError {
    fn from(err: PoisonError<T>) -> Self {
        Self(format!("{err:?}"))
    }
}

// `derive(FromRef)` is needed to make use of `leptos_axum`'s `extract_with_state()`
#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: Arc<RwLock<Client>>,
    pub surgeon: Arc<RwLock<Option<Surgeon>>>,
}
