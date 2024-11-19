use crate::{auth::AuthenticationError, db::users::GetUserDataError, logs::LogError};

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
