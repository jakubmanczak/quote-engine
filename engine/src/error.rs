use crate::auth::error::AuthenticationError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum OmniError {
    #[error("{0}")]
    AuthError(#[from] AuthenticationError),
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("{0}")]
    UlidDecodeError(#[from] ulid::DecodeError),
    #[error("{0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("{0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

impl OmniError {
    pub fn log_and_respond(self) -> Response {
        use OmniError::*;
        match self {
            AuthError(e) => {
                const ERRTEXT: &str = "Authentication error";
                match e.suggested_status_code() {
                    StatusCode::INTERNAL_SERVER_ERROR => error!("{ERRTEXT}: {e}"),
                    _ => (),
                };
                (e.suggested_status_code(), e.to_string()).into_response()
            }
            SqlxError(e) => {
                const ERRTEXT: &str = "Database error";
                error!("{ERRTEXT}: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, ERRTEXT).into_response()
            }
            UlidDecodeError(e) => {
                const ERRTEXT: &str = "Ulid decode error";
                error!("{ERRTEXT}: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, ERRTEXT).into_response()
            }
            Base64DecodeError(e) => {
                const ERRTEXT: &str = "BASE64 decode error";
                error!("{ERRTEXT}: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, ERRTEXT).into_response()
            }
            FromUtf8Error(e) => {
                const ERRTEXT: &str = "UTF-8 decode error";
                error!("{ERRTEXT}: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, ERRTEXT).into_response()
            }
        }
    }
}
