use axum::{routing::get, Router};

mod auth;
mod authors;
mod logs;
mod permissions;
mod quotes;
mod users;
mod lines;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(healthcheck))
        .route("/live", get(healthcheck))
        .route("/health", get(healthcheck))
        .merge(users::exported_routes())
        .merge(permissions::exported_routes())
        .merge(logs::exported_routes())
        .merge(auth::exported_routes())
        .merge(quotes::exported_routes())
        .merge(lines::exported_routes())
        .merge(authors::exported_routes())
}

// 200 OK
async fn healthcheck() -> () {}
