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
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::AuthorsInspectPermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    }

    let vec = match Author::get_all(&state.dbpool).await {
        Ok(authors) => authors,
        Err(e) => return e.respond(),
    };
    Json(vec).into_response()
}

async fn get_all_extended(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::AuthorsInspectPermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    }

    let vec = match ExtendedAuthor::get_all(&state.dbpool).await {
        Ok(authors) => authors,
        Err(e) => return e.respond(),
    };
    Json(vec).into_response()
}

async fn post_handler(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(author): Json<Author>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::AuthorsCreatePermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    }

    match Author::create(author, &state.dbpool).await {
        Ok(author) => (StatusCode::CREATED, Json(author)).into_response(),
        Err(e) => return e.respond(),
    }
}

async fn by_id_handler(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::AuthorsInspectPermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    match Author::get_by_id(&id, &state.dbpool).await {
        Ok(author) => match author {
            Some(author) => Json(author).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        Err(e) => return e.respond(),
    }
}

async fn by_id_extended_handler(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::AuthorsInspectPermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    match ExtendedAuthor::get_by_id(&id, &state.dbpool).await {
        Ok(author) => match author {
            Some(author) => Json(author).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        Err(e) => return e.respond(),
    }
}

async fn patch_handler(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
    Json(patch): Json<AuthorPatch>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::AuthorsModifyPermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    match Author::get_by_id(&id, &state.dbpool).await {
        Ok(author) => match author {
            Some(author) => match author.patch(patch, &state.dbpool).await {
                Ok(author) => Json(author).into_response(),
                Err(e) => return e.respond(),
            },
            None => return StatusCode::NOT_FOUND.into_response(),
        },
        Err(e) => return e.respond(),
    }
}

async fn delete_handler(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::AuthorsDeletePermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    match Author::get_by_id(&id, &state.dbpool).await {
        Ok(author) => match author {
            Some(author) => match author.destroy(&state.dbpool).await {
                Ok(_) => StatusCode::NO_CONTENT.into_response(),
                Err(e) => return e.respond(),
            },
            None => return StatusCode::NOT_FOUND.into_response(),
        },
        Err(e) => return e.respond(),
    }
}
