use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use tower_cookies::Cookies;

use crate::{
    omnierror::OmniError,
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
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.is_infradmin() {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    Ok(Json(User::get_all(&state.dbpool).await?).into_response())
}

async fn all_sessions(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.is_infradmin() {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    Ok(Json(Session::get_all(&state.dbpool).await?).into_response())
}
