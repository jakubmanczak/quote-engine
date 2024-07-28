use axum::{routing::get, Router};

mod permissions;
mod users;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(healthcheck))
        .route("/live", get(healthcheck))
        .route("/health", get(healthcheck))
        .merge(users::exported_routes())
        .merge(permissions::exported_routes())
}

// 200 OK
async fn healthcheck() -> () {}
