use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/live", get(health))
        .route("/health", get(health))
}

// 200 OK
async fn health() -> () {}
