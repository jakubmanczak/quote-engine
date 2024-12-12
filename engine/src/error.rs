use crate::auth::error::AuthenticationError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum OmniError {
    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("{0}")]
    UlidDecodeError(#[from] ulid::DecodeError),
    #[error("{0}")]
    AuthError(#[from] AuthenticationError),
}

impl OmniError {
    pub fn log_and_respond(self) -> Response {
        use OmniError::*;
        match self {
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
            AuthError(e) => {
                const ERRTEXT: &str = "Authentication error";
                match e.suggested_status_code() {
                    StatusCode::INTERNAL_SERVER_ERROR => error!("{ERRTEXT}: {e}"),
                    _ => (),
                };
                (e.suggested_status_code(), format!("{ERRTEXT}: {e}")).into_response()
            }
        }
    }
}
