use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use tower_cookies::Cookies;

use crate::{
    state::SharedState,
    user::{auth::session::Session, User},
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/infra/all-users", get(all_users))
        .route("/infra/all-sessions", get(all_sessions))
}

async fn all_users(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.is_infradmin() {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    }

    match User::get_all(&state.dbpool).await {
        Ok(users) => Json(users).into_response(),
        Err(e) => e.respond(),
    }
}

async fn all_sessions(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.is_infradmin() {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    match Session::get_all(&state.dbpool).await {
        Ok(sessions) => Json(sessions).into_response(),
        Err(e) => e.respond(),
    }
}
