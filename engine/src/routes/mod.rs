use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use sqlx::{Pool, Sqlite};

pub fn all() -> Router<Pool<Sqlite>> {
    Router::new()
        // HEALTH CHECKS
        .route("/", get(twohundred))
        .route("/health", get(twohundred))
        .route("/live", get(twohundred))
}

async fn twohundred() -> Response {
    StatusCode::OK.into_response()
}
