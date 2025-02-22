use crate::surgeon::Surgeon;
use axum_macros::FromRef;
use gel_tokio::Client;
use leptos::config::LeptosOptions;
use std::sync::{Arc, RwLock};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("The lock is poisoned and couldn't be written or read")]
pub struct StatePoisonedError(pub String);

// `derive(FromRef)` is needed to make use of `leptos_axum`'s `extract_with_state()` in
// handlers
#[derive(FromRef, Clone, Debug)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: Arc<RwLock<Client>>,
    pub surgeon: Arc<RwLock<Option<Surgeon>>>,
}
