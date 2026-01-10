use std::fmt::Debug;

use axum_login::AuthUser;
use oauth2::CsrfToken;
use oauth2::EndpointNotSet;
use oauth2::EndpointSet;
use oauth2::basic::BasicClient;
use serde::Deserialize;

use crate::model::Email;

#[derive(Clone, Debug)]
pub struct Credentials {
    pub code: String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub email: Email,
}

pub type BasicClientSet =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointSet, EndpointSet>;
