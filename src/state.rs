use crate::model::Surgeon;
use axum_macros::FromRef;
use gel_tokio::Client;
use leptos::prelude::LeptosOptions;
use std::sync::{Arc, RwLock};

// `derive(FromRef)` is needed to make use of `leptos_axum`'s `extract_with_state()`
#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: Arc<RwLock<Client>>,
    pub surgeon: Arc<RwLock<Option<Surgeon>>>,
}
