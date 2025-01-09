use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use tower_cookies::Cookies;

use crate::{state::SharedState, user::User};

pub fn routes() -> Router<SharedState> {
    Router::new().route("/health", get(health))
}

async fn health(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Response {
    // infosec: only show system health to actual users
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(_) => (),
        Err(e) => return e.respond(),
    };

    let sysinfo = state.sysinfo.read().await;
    Json(&*sysinfo).into_response()
}
