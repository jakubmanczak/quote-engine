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
    auth::authenticate,
    error::OmniError,
    users::{
        patch::{UserPatch, UserPatchError},
        User,
    },
};

pub fn routes() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/users", get(get_users).post(post_new_user_manually))
        .route("/users/:id", get(get_user_by_id).patch(patch_user))
        .route("/users/self", get(get_user_self))
        .route("/users/self/sessions", get(get_self_sessions))
        .route("/users/count", get(get_users_count))
}

async fn get_user_by_id(
    Path(id): Path<Ulid>,
    headers: HeaderMap,
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
) -> Response {
    match authenticate(&headers, cookies, &pool).await {
        Ok(_) => (),
        Err(e) => return e.log_and_respond(),
    };

    match User::get_by_id(id, &pool).await {
        Ok(user) => match user {
            Some(user) => Json(user).into_response(),
            None => (StatusCode::BAD_REQUEST, "No such user found").into_response(),
        },
        Err(e) => e.log_and_respond(),
    }
}

async fn get_users(
    headers: HeaderMap,
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
) -> Response {
    match authenticate(&headers, cookies, &pool).await {
        Ok(_) => (),
        Err(e) => return e.log_and_respond(),
    };

    match User::get_all(&pool).await {
        Ok(vec) => (StatusCode::OK, Json(vec)).into_response(),
        Err(e) => e.log_and_respond(),
    }
}

async fn get_user_self(
    headers: HeaderMap,
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
) -> Response {
    match authenticate(&headers, cookies, &pool).await {
        Ok(user) => Json(user).into_response(),
        Err(e) => e.log_and_respond(),
    }
}

async fn get_self_sessions(
    headers: HeaderMap,
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
) -> Response {
    let user = match authenticate(&headers, cookies, &pool).await {
        Ok(user) => user,
        Err(e) => return e.log_and_respond(),
    };
    match user.get_sessions(&pool).await {
        Ok(vec) => Json(vec).into_response(),
        Err(e) => e.log_and_respond(),
    }
}

async fn patch_user(
    headers: HeaderMap,
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<Ulid>,
    Json(patch): Json<UserPatch>,
) -> Response {
    if !patch.is_valid() {
        return OmniError::from(UserPatchError::NoFields).log_and_respond();
    }
    let target = match User::get_by_id(id, &pool).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return OmniError::from(UserPatchError::IncorrectTarget).log_and_respond(),
        },
        Err(e) => return e.log_and_respond(),
    };
    // TODO: check update permissions, apart from just clearance
    let actor = match authenticate(&headers, cookies, &pool).await {
        Ok(actor) => actor,
        Err(e) => return e.log_and_respond(),
    };
    if target.id != actor.id && target.clearance >= actor.clearance {
        return OmniError::from(UserPatchError::InsufficientClearance).log_and_respond();
    }

    match target.patch(patch, &pool).await {
        Ok(user) => Json(user).into_response(),
        Err(e) => e.log_and_respond(),
    }
}

async fn get_users_count(
    headers: HeaderMap,
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
) -> Response {
    match authenticate(&headers, cookies, &pool).await {
        Ok(_) => (),
        Err(e) => return e.log_and_respond(),
    };

    match User::get_db_count(&pool).await {
        Ok(count) => (StatusCode::OK, format!("{}", count)).into_response(),
        Err(e) => return e.log_and_respond(),
    }
}

async fn post_new_user_manually(Json(body): Json<User>) -> Response {
    (StatusCode::OK, Json(body)).into_response()
}
