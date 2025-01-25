use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    omnierror::OmniError,
    quotes::authors::{Author, AuthorPatch, ExtendedAuthor},
    state::SharedState,
    user::{attributes::UserAttribute as UA, User},
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/authors", get(get_all).post(post_handler))
        .route(
            "/authors/{id}",
            get(by_id_handler)
                .patch(patch_handler)
                .delete(delete_handler),
        )
        .route("/authors/{id}/extended", get(by_id_extended_handler))
        .route("/authors/extended", get(get_all_extended))
}

async fn get_all(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::AuthorsInspectPermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    let authors = Author::get_all(&state.dbpool).await?;
    Ok(Json(authors).into_response())
}

async fn get_all_extended(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::AuthorsInspectPermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    Ok(Json(ExtendedAuthor::get_all(&state.dbpool).await?).into_response())
}

async fn post_handler(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(author): Json<Author>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::AuthorsCreatePermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    let author = Author::create(author, &state.dbpool).await?;
    Ok((StatusCode::CREATED, Json(author)).into_response())
}

async fn by_id_handler(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::AuthorsInspectPermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    match Author::get_by_id(&id, &state.dbpool).await? {
        Some(author) => Ok(Json(author).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

async fn by_id_extended_handler(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::AuthorsInspectPermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    match ExtendedAuthor::get_by_id(&id, &state.dbpool).await? {
        Some(author) => Ok(Json(author).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

async fn patch_handler(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
    Json(patch): Json<AuthorPatch>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::AuthorsModifyPermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    match Author::get_by_id(&id, &state.dbpool).await? {
        Some(author) => {
            let author = author.patch(patch, &state.dbpool).await?;
            Ok(Json(author).into_response())
        }
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

async fn delete_handler(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::AuthorsDeletePermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    match Author::get_by_id(&id, &state.dbpool).await? {
        Some(author) => {
            author.destroy(&state.dbpool).await?;
            Ok(StatusCode::NO_CONTENT.into_response())
        }
        None => Ok(StatusCode::FORBIDDEN.into_response()),
    }
}
