use axum::{routing::get, Router};
pub fn routes() -> Router {
    Router::new()
        .route("/", get(healthcheck))
        .route("/live", get(healthcheck))
        .route("/health", get(healthcheck))
}

// 200 OK
async fn healthcheck() -> () {}
