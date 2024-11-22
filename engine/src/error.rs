use crate::{auth::AuthenticationError, db::users::GetUserDataError, logs::LogError};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    AuthenticationError(#[from] AuthenticationError),
    #[error("GetUserDataError: {0}")]
    GetUserDataError(#[from] GetUserDataError),
    #[error("Base64DecodeError: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("LogError: {0}")]
    LogError(#[from] LogError),
}

impl Error {
    pub fn log_and_response(self) -> Response {
        match self {
            crate::error::Error::AuthenticationError(ref e) => {
                match e.suggested_status_code() {
                    StatusCode::INTERNAL_SERVER_ERROR => error!("{}", self),
                    _ => (),
                };
                (e.suggested_status_code(), e.to_string()).into_response()
            }
            _ => {
                error!("{}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error logged serverside").into_response()
            }
        }
    }
}
