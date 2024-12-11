use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use sqlx::{Pool, Sqlite};
use ulid::Ulid;

use crate::users::User;

pub fn routes() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(post_new_user_manually))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/count", get(get_users_count))
}

async fn get_user_by_id(Path(id): Path<Ulid>, State(pool): State<Pool<Sqlite>>) -> Response {
    match User::get_by_id(id, &pool).await {
        Ok(user) => match user {
            Some(user) => Json(user).into_response(),
            None => (StatusCode::NOT_FOUND, "No such user found").into_response(),
        },
        Err(e) => e.log_and_respond(),
    }
}

async fn get_users(State(pool): State<Pool<Sqlite>>) -> Response {
    match User::get_all(&pool).await {
        Ok(vec) => (StatusCode::OK, Json(vec)).into_response(),
        Err(e) => e.log_and_respond(),
    }
}

async fn get_users_count(State(pool): State<Pool<Sqlite>>) -> Response {
    match User::get_db_count(&pool).await {
        Ok(count) => (StatusCode::OK, format!("{}", count)).into_response(),
        Err(e) => return e.log_and_respond(),
    }
}

async fn post_new_user_manually(Json(body): Json<User>) -> Response {
    (StatusCode::OK, Json(body)).into_response()
}
