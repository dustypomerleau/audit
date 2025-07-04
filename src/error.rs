#[cfg(feature = "ssr")] use axum::response::{IntoResponse, Response};
use leptos::{
    prelude::{FromServerFnError, ServerFnErrorErr},
    server_fn::codec::JsonEncoding,
};
use serde::{Deserialize, Serialize};
use std::sync::PoisonError;
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum AppError {
    #[error("authentication error: {0:?}")]
    Auth(String),
    #[error("out-of-bounds error: {0:?}")]
    Bounds(String),
    #[error("database error: {0:?}")]
    Db(String),
    #[error("(de)serialization error: {0:?}")]
    Serde(String),
    #[error("server error: {0:?}")]
    Server(String),
    #[error("state error: {0:?}")]
    State(String),
    #[error("view error: {0:?}")]
    View(String),
}

impl From<ServerFnErrorErr> for AppError {
    fn from(err: ServerFnErrorErr) -> Self {
        Self::Server(format!("{err:?}"))
    }
}

impl From<chrono::format::ParseError> for AppError {
    fn from(err: chrono::format::ParseError) -> Self {
        Self::Server(format!("{err:?}"))
    }
}

#[cfg(feature = "ssr")]
impl From<gel_tokio::Error> for AppError {
    fn from(err: gel_tokio::Error) -> Self {
        Self::Db(format!("{err:?}"))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::Server(format!("{err:?}"))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(format!("{err:?}"))
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self {
        Self::State(format!("{err:?}"))
    }
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        Self::Server(format!("{err}"))
    }
}

#[cfg(feature = "ssr")]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}
