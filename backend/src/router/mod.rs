use crate::state::SharedState;
use axum::{routing::get, Router};
use tower_cookies::CookieManagerLayer;

mod auth;
mod infra;
mod users;

pub fn init(state: SharedState) -> Router {
    Router::new()
        .route("/", get(|| async { () }))
        .merge(infra::routes())
        .merge(auth::routes())
        .merge(users::routes())
        .with_state(state)
        .layer(CookieManagerLayer::new())
}
