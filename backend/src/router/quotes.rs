use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    quotes::Quote,
    state::SharedState,
    user::{attributes::UserAttribute as UA, User},
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/quotes", post(post_new))
        .route("/quotes/all", get(get_all))
        .route("/quotes/{id}", get(get_by_id))
}

async fn get_by_id(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Response {
    match Quote::get_by_id(&id, &state.dbpool).await {
        Ok(opt) => match opt {
            Some(q) => {
                match q.clearance == 0 {
                    true => (),
                    false => match User::authenticate(&headers, cookies, &state.dbpool).await {
                        Ok(u) => match u.clearance >= q.clearance {
                            true => (),
                            false => return StatusCode::FORBIDDEN.into_response(),
                        },
                        Err(e) => return e.respond(),
                    },
                }
                Json(q).into_response()
            }
            None => StatusCode::NOT_FOUND.into_response(),
        },
        Err(e) => e.respond(),
    }
}

// NOTE: this is resource intensive in production
// throw this out completely
async fn get_all(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::TheEverythingPermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    match Quote::get_all(&state.dbpool).await {
        Ok(vec) => Json(vec).into_response(),
        Err(e) => e.respond(),
    }
}

const BAD_CLEARANCE: &str = "The quote must have appropriate clearance in regard to its submitter.";
const NO_LINES: &str = "The quote must have quote lines.";

async fn post_new(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(quote): Json<Quote>,
) -> Response {
    let u = match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::QuotesCreatePermission) {
            true => u,
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    if quote.lines.is_empty() {
        return (StatusCode::BAD_REQUEST, NO_LINES).into_response();
    }
    if quote.clearance > u.clearance {
        return (StatusCode::BAD_REQUEST, BAD_CLEARANCE).into_response();
    }

    match Quote::create(quote, &state.dbpool).await {
        Ok(q) => Json(q).into_response(),
        Err(e) => e.respond(),
    }
}
