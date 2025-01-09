use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

use crate::user::{auth::error::AuthError, validity::ValidityError};

#[derive(thiserror::Error, Debug)]
pub enum OmniError {
    #[error("{0}")]
    AuthError(#[from] AuthError),
    #[error("{0}")]
    UserValidityError(#[from] ValidityError),

    #[error("sqlx::Error => {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("base64::DecodeError => {0}")]
    B64DecodeError(#[from] base64::DecodeError),
    #[error("std::string::FromUtf8Error => {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("passwordhash error: {0}")]
    PassHashError(String),
}

impl OmniError {
    pub fn respond(&self) -> Response {
        const ISE: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
        const BAD: StatusCode = StatusCode::BAD_REQUEST;
        use OmniError as E;
        match self {
            E::AuthError(e) => (e.status_code(), e.to_string()).into_response(),
            E::UserValidityError(e) => (BAD, e.to_string()).into_response(),
            E::SqlxError(e) => {
                use sqlx::Error as SE;
                match e {
                    SE::RowNotFound => (ISE, "SQLx Error; No rows returned").into_response(),
                    _ => (ISE, "SQLx Error; Logged").into_response(),
                }
            }
            E::B64DecodeError(_) => (ISE, "Base64Decode Error").into_response(),
            E::FromUtf8Error(_) => (ISE, "FromUtf8 Error").into_response(),
            E::PassHashError(_) => (ISE, "PasswordHash Error").into_response(),
        }
    }
}

impl From<argon2::password_hash::Error> for OmniError {
    fn from(e: argon2::password_hash::Error) -> Self {
        OmniError::PassHashError(e.to_string())
    }
}
