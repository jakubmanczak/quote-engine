use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use sqlx::{Pool, Sqlite};

mod auth;
mod authors;
mod users;

pub fn all() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/", get(twohundred))
        .route("/health", get(twohundred))
        .route("/live", get(twohundred))
        .route("/teapot", get(teapot))
        .merge(auth::routes())
        .merge(users::routes())
        .merge(authors::routes())
}

async fn twohundred() -> Response {
    StatusCode::OK.into_response()
}

async fn teapot() -> Response {
    StatusCode::IM_A_TEAPOT.into_response()
}
