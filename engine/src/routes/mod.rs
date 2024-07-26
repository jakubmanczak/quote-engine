use axum::{routing::get, Router};

mod users;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(healthcheck))
        .route("/live", get(healthcheck))
        .route("/health", get(healthcheck))
        .merge(users::exported_routes())
}

// 200 OK
async fn healthcheck() -> () {}
