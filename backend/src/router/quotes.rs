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
    omnierror::OmniError,
    quotes::{placeholder::return_placeholder_random_public_quote, Quote},
    state::SharedState,
    user::{attributes::UserAttribute as UA, User},
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/quotes", post(post_new))
        .route("/quotes/all", get(get_all))
        .route("/quotes/{id}", get(get_by_id).delete(delete))
        .route("/quotes/randompublic", get(get_random))
}

async fn get_by_id(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    match Quote::get_by_id(&id, &state.dbpool).await? {
        Some(q) => {
            if q.clearance != 0 {
                let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
                if u.clearance < q.clearance {
                    return Ok(StatusCode::FORBIDDEN.into_response());
                }
            }
            Ok(Json(q).into_response())
        }
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

async fn get_random(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    match Quote::get_random_public(&state.dbpool).await? {
        Some(q) => Ok(Json(q).into_response()),
        None => Ok(Json(return_placeholder_random_public_quote()).into_response()),
    }
}

// NOTE: this is resource intensive in production
// it MUST have pagination or streaming
async fn get_all(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::TheEverythingPermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    Ok(Json(Quote::get_all(&state.dbpool).await?).into_response())
}

const BAD_CLEARANCE: &str = "The quote must have appropriate clearance in regard to its submitter.";
const NO_LINES: &str = "The quote must have quote lines.";

async fn post_new(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(quote): Json<Quote>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::QuotesCreatePermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    if quote.lines.is_empty() {
        return Ok((StatusCode::BAD_REQUEST, NO_LINES).into_response());
    }
    if quote.clearance > u.clearance {
        return Ok((StatusCode::FORBIDDEN, BAD_CLEARANCE).into_response());
    }

    let quote = Quote::create(quote, &state.dbpool).await?;
    Ok((StatusCode::CREATED, Json(quote)).into_response())
}

async fn delete(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::QuotesDeletePermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    let q = match Quote::get_by_id(&id, &state.dbpool).await? {
        Some(q) => q,
        None => return Ok(StatusCode::NOT_FOUND.into_response()),
    };
    if q.clearance >= u.clearance {
        return Ok((StatusCode::FORBIDDEN, BAD_CLEARANCE).into_response());
    }

    q.delete(&state.dbpool).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
