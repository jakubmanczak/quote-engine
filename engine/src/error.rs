use crate::{auth::AuthenticationError, db::users::GetUserDataError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    AuthenticationError(AuthenticationError),
    #[error("GetUserDataError: {0}")]
    GetUserDataError(GetUserDataError),
    #[error("Base64DecodeError: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}
