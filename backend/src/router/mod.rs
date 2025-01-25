use crate::state::SharedState;
use axum::{http::Method, routing::get, Router};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

mod auth;
mod authors;
mod health;
mod infra;
mod quotes;
mod users;

pub fn init(state: SharedState) -> Router {
    let allowed_origins = ["http://localhost:3000"];

    Router::new()
        .route("/", get(|| async { () }))
        .merge(health::routes())
        .merge(infra::routes())
        .merge(auth::routes())
        .merge(users::routes())
        .merge(authors::routes())
        .merge(quotes::routes())
        .with_state(state)
        .layer(CookieManagerLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(allowed_origins.map(|s| s.parse().unwrap()))
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_credentials(true),
        )
}
