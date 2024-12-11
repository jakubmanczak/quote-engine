use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use sqlx::{Pool, Sqlite};

mod users;

pub fn all() -> Router<Pool<Sqlite>> {
    Router::new()
        // HEALTH CHECKS
        .route("/", get(twohundred))
        .route("/health", get(twohundred))
        .route("/live", get(twohundred))
        // MEME
        .route("/teapot", get(teapot))
        // ACTUAL ROUTES
        .merge(users::routes())
}

async fn twohundred() -> Response {
    StatusCode::OK.into_response()
}

async fn teapot() -> Response {
    StatusCode::IM_A_TEAPOT.into_response()
}
