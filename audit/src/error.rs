use std::sync::PoisonError;

#[cfg(feature = "ssr")] use axum::response::IntoResponse;
#[cfg(feature = "ssr")] use axum::response::Response;
#[cfg(feature = "ssr")] use axum_login::tower_sessions::session_store;
use leptos::prelude::FromServerFnError;
use leptos::prelude::ServerFnErrorErr;
use leptos::server_fn::codec::JsonEncoding;
use oauth2::ErrorResponse;
use oauth2::RequestTokenError;
use serde::Deserialize;
use serde::Serialize;
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

#[cfg(feature = "ssr")]
impl From<chrono::format::ParseError> for AppError {
    fn from(err: chrono::format::ParseError) -> Self { Self::Server(err.to_string()) }
}

#[cfg(feature = "ssr")]
impl From<mailgun_rs::SendError> for AppError {
    fn from(err: mailgun_rs::SendError) -> Self { Self::Server(err.to_string()) }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(err: PoisonError<T>) -> Self { Self::State(err.to_string()) }
}

#[cfg(feature = "ssr")]
impl<RE, T> From<oauth2::RequestTokenError<RE, T>> for AppError
where
    RE: core::error::Error + 'static,
    T: ErrorResponse + 'static,
{
    fn from(err: RequestTokenError<RE, T>) -> Self { Self::Server(err.to_string()) }
}

#[cfg(feature = "ssr")]
impl From<oauth2::reqwest::Error> for AppError {
    fn from(err: oauth2::reqwest::Error) -> Self { Self::Server(err.to_string()) }
}

#[cfg(feature = "ssr")]
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self { Self::Server(err.to_string()) }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self { Self::Serde(err.to_string()) }
}

impl From<ServerFnErrorErr> for AppError {
    fn from(err: ServerFnErrorErr) -> Self { Self::Server(err.to_string()) }
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self { Self::Db(err.to_string()) }
}

#[cfg(feature = "ssr")]
impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(err: sqlx::migrate::MigrateError) -> Self { Self::Db(err.to_string()) }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self { Self::Server(err.to_string()) }
}

#[cfg(feature = "ssr")]
impl From<session_store::Error> for AppError {
    fn from(err: session_store::Error) -> Self { Self::Server(err.to_string()) }
}
#[cfg(feature = "ssr")]
impl From<AppError> for session_store::Error {
    fn from(err: AppError) -> Self { Self::Backend(err.to_string()) }
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(err: ServerFnErrorErr) -> Self { Self::Server(err.to_string()) }
}

#[cfg(feature = "ssr")]
impl IntoResponse for AppError {
    fn into_response(self) -> Response { self.to_string().into_response() }
}
