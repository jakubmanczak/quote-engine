use crate::state::SharedState;
use axum::{routing::get, Router};
use tower_cookies::CookieManagerLayer;

mod auth;
mod authors;
mod health;
mod infra;
mod quotes;
mod users;

pub fn init(state: SharedState) -> Router {
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
}
