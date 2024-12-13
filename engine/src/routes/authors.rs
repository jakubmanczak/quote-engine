use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch, post},
    Json, Router,
};
use sqlx::{Pool, Sqlite};
use ulid::Ulid;

use crate::authors::{Author, AuthorUpdate};

pub fn routes() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/authors", get(get_authors))
        .route("/authors", post(post_author))
        .route("/authors/:id", get(get_author_by_id))
        .route("/authors/:id", patch(patch_author_by_id))
        .route("/authors/count", get(get_authors_count))
}

async fn get_author_by_id(Path(id): Path<Ulid>, State(pool): State<Pool<Sqlite>>) -> Response {
    match Author::get_by_id(id, &pool).await {
        Ok(author) => match author {
            Some(author) => Json(author).into_response(),
            None => (StatusCode::NOT_FOUND, "No such author found").into_response(),
        },
        Err(e) => e.log_and_respond(),
    }
}

async fn get_authors(State(pool): State<Pool<Sqlite>>) -> Response {
    match Author::get_all(&pool).await {
        Ok(vec) => Json(vec).into_response(),
        Err(e) => return e.log_and_respond(),
    }
}

async fn get_authors_count(State(pool): State<Pool<Sqlite>>) -> Response {
    match Author::get_db_count(&pool).await {
        Ok(count) => (StatusCode::OK, format!("{}", count)).into_response(),
        Err(e) => return e.log_and_respond(),
    }
}

async fn post_author(State(pool): State<Pool<Sqlite>>, Json(body): Json<Author>) -> Response {
    match Author::post(body, &pool).await {
        Ok(author) => (StatusCode::CREATED, Json(author)).into_response(),
        Err(e) => e.log_and_respond(),
    }
}

async fn patch_author_by_id(
    Path(id): Path<Ulid>,
    State(pool): State<Pool<Sqlite>>,
    Json(body): Json<AuthorUpdate>,
) -> Response {
    match Author::patch(id, body, &pool).await {
        Ok(author) => match author {
            Some(author) => (StatusCode::OK, Json(author)).into_response(),
            None => (StatusCode::NOT_FOUND, "No such author found").into_response(),
        },
        Err(e) => e.log_and_respond(),
    }
}
