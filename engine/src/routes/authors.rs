use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use sqlx::{Pool, Sqlite};
use tower_cookies::Cookies;
use ulid::Ulid;

use crate::{
    all_none_trait::AllNoneChecker,
    auth::authenticate,
    authors::{extended::ExtendedAuthor, Author, AuthorPatch},
    users::attributes::UserAttribute,
};

pub fn routes() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/authors", get(get_authors).post(post_author))
        .route(
            "/authors/:id",
            get(get_author_by_id)
                .patch(patch_author_by_id)
                .delete(delete_author_by_id),
        )
        .route("/authors/:id/extended", get(get_extended_author))
        .route("/authors/count", get(get_authors_count))
}

async fn get_author_by_id(
    Path(id): Path<Ulid>,
    State(pool): State<Pool<Sqlite>>,
    headers: HeaderMap,
    cookies: Cookies,
) -> Response {
    let user = match authenticate(&headers, cookies, &pool).await {
        Ok(user) => user,
        Err(e) => return e.log_and_respond(),
    };
    if !user.has_permission(UserAttribute::AuthorInspectPermission) {
        return StatusCode::FORBIDDEN.into_response();
    }

    match Author::get_by_id(id, &pool).await {
        Ok(author) => match author {
            Some(author) => Json(author).into_response(),
            None => (StatusCode::BAD_REQUEST, "No such author found").into_response(),
        },
        Err(e) => e.log_and_respond(),
    }
}

async fn get_extended_author(
    Path(id): Path<Ulid>,
    State(pool): State<Pool<Sqlite>>,
    headers: HeaderMap,
    cookies: Cookies,
) -> Response {
    let user = match authenticate(&headers, cookies, &pool).await {
        Ok(user) => user,
        Err(e) => return e.log_and_respond(),
    };
    if !user.has_permission(UserAttribute::AuthorInspectPermission) {
        return StatusCode::FORBIDDEN.into_response();
    }

    match ExtendedAuthor::get_by_id(id, &pool).await {
        Ok(author) => match author {
            Some(author) => Json(author).into_response(),
            None => (StatusCode::BAD_REQUEST, "No such author found").into_response(),
        },
        Err(e) => e.log_and_respond(),
    }
}

async fn get_authors(
    State(pool): State<Pool<Sqlite>>,
    headers: HeaderMap,
    cookies: Cookies,
) -> Response {
    let user = match authenticate(&headers, cookies, &pool).await {
        Ok(user) => user,
        Err(e) => return e.log_and_respond(),
    };
    if !user.has_permission(UserAttribute::AuthorInspectPermission) {
        return StatusCode::FORBIDDEN.into_response();
    }

    match Author::get_all(&pool).await {
        Ok(vec) => Json(vec).into_response(),
        Err(e) => return e.log_and_respond(),
    }
}

async fn get_authors_count(
    State(pool): State<Pool<Sqlite>>,
    headers: HeaderMap,
    cookies: Cookies,
) -> Response {
    let user = match authenticate(&headers, cookies, &pool).await {
        Ok(user) => user,
        Err(e) => return e.log_and_respond(),
    };
    if !user.has_permission(UserAttribute::AuthorInspectPermission) {
        return StatusCode::FORBIDDEN.into_response();
    }

    match Author::get_db_count(&pool).await {
        Ok(count) => (StatusCode::OK, format!("{}", count)).into_response(),
        Err(e) => return e.log_and_respond(),
    }
}

async fn post_author(
    State(pool): State<Pool<Sqlite>>,
    headers: HeaderMap,
    cookies: Cookies,
    Json(body): Json<Author>,
) -> Response {
    let user = match authenticate(&headers, cookies, &pool).await {
        Ok(user) => user,
        Err(e) => return e.log_and_respond(),
    };
    if !user.has_permission(UserAttribute::AuthorCreatePermission) {
        return StatusCode::FORBIDDEN.into_response();
    }

    match Author::post(body, &pool).await {
        Ok(author) => (StatusCode::CREATED, Json(author)).into_response(),
        Err(e) => e.log_and_respond(),
    }
}

async fn patch_author_by_id(
    Path(id): Path<Ulid>,
    State(pool): State<Pool<Sqlite>>,
    headers: HeaderMap,
    cookies: Cookies,
    Json(body): Json<AuthorPatch>,
) -> Response {
    if body.all_none() {
        return (StatusCode::BAD_REQUEST, "No fields to update.").into_response();
    }
    let user = match authenticate(&headers, cookies, &pool).await {
        Ok(user) => user,
        Err(e) => return e.log_and_respond(),
    };
    if !user.has_permission(UserAttribute::AuthorModifyPermission) {
        return StatusCode::FORBIDDEN.into_response();
    }

    let author = match Author::get_by_id(id, &pool).await {
        Ok(a) => match a {
            Some(author) => author,
            None => return (StatusCode::BAD_REQUEST, "No such author found.").into_response(),
        },
        Err(e) => return e.log_and_respond(),
    };

    match author.patch(body, &pool).await {
        Ok(author) => (StatusCode::OK, Json(author)).into_response(),
        Err(e) => e.log_and_respond(),
    }
}

async fn delete_author_by_id(
    Path(id): Path<Ulid>,
    State(pool): State<Pool<Sqlite>>,
    headers: HeaderMap,
    cookies: Cookies,
) -> Response {
    let user = match authenticate(&headers, cookies, &pool).await {
        Ok(user) => user,
        Err(e) => return e.log_and_respond(),
    };
    if !user.has_permission(UserAttribute::AuthorDeletePermission) {
        return StatusCode::FORBIDDEN.into_response();
    }
    let target = match Author::get_by_id(id, &pool).await {
        Ok(author) => match author {
            Some(author) => author,
            None => return (StatusCode::BAD_REQUEST, "No such author found.").into_response(),
        },
        Err(e) => return e.log_and_respond(),
    };
    match target.delete(&pool).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => e.log_and_respond(),
    }
}
