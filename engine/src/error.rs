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
        }
    }
}
