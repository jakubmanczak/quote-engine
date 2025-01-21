use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, patch, post},
    Json, Router,
};
use serde::Deserialize;
use strum::VariantArray;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    omnierror::OmniError,
    state::SharedState,
    user::{attributes::UserAttribute as UA, patch::UserPatch, User},
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/users", post(create_user_manually))
        .route(
            "/users/{id}",
            get(get_user_by_id).patch(patch_user).delete(delete_user),
        )
        .route("/users/{id}/change-password", patch(change_password))
        .route("/users/user-attributes", get(all_user_attributes))
}

#[derive(Deserialize)]
struct ManualUserCreation {
    handle: String,
    password: String,
}
async fn create_user_manually(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(user): Json<ManualUserCreation>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::UsersManualCreatePermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    if let Err(e) = User::is_valid_handle(&user.handle) {
        return OmniError::from(e).respond();
    }
    if let Err(e) = User::is_valid_password(&user.password) {
        return OmniError::from(e).respond();
    }

    match User::create(
        User::new_incomplete(user.handle),
        &user.password,
        &state.dbpool,
    )
    .await
    {
        Ok(u) => Json(u).into_response(),
        Err(e) => e.respond(),
    }
}

async fn get_user_by_id(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::UsersInspectPermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    match User::get_by_id(&id, &state.dbpool).await {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => (StatusCode::BAD_REQUEST, "No such user found.").into_response(),
        Err(e) => e.respond(),
    }
}

async fn patch_user(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
    Json(patch): Json<UserPatch>,
) -> Response {
    let actor = match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => u,
        Err(e) => return e.respond(),
    };
    let target = match User::get_by_id(&id, &state.dbpool).await {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::BAD_REQUEST, "No such user found.").into_response(),
        Err(e) => return e.respond(),
    };

    if actor.id == target.id {
        if patch.handle.is_some() && !actor.has_permission(UA::UsersChangeOwnHandlePermission) {
            return StatusCode::FORBIDDEN.into_response();
        }
    } else {
        if actor.clearance <= target.clearance {
            return StatusCode::FORBIDDEN.into_response();
        }
        if patch.handle.is_some() && !actor.has_permission(UA::UsersManageHandlesPermission) {
            return StatusCode::FORBIDDEN.into_response();
        }
    }

    if let Some(clearance) = patch.clearance {
        if !actor.has_permission(UA::UsersManageClearancesPermission) {
            return StatusCode::FORBIDDEN.into_response();
        }
        if clearance > actor.clearance && !actor.has_permission(UA::TheEverythingPermission) {
            return StatusCode::FORBIDDEN.into_response();
        }
    }

    if let Some(handle) = &patch.handle {
        if let Err(e) = User::is_valid_handle(handle) {
            return OmniError::from(e).respond();
        }
    }

    match target.patch(patch, &state.dbpool).await {
        Ok(u) => Json(u).into_response(),
        Err(e) => e.respond(),
    }
}

async fn delete_user(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Response {
    match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => match u.has_permission(UA::UsersDeletePermission) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        },
        Err(e) => return e.respond(),
    };

    match User::get_by_id(&id, &state.dbpool).await {
        Ok(Some(user)) => match user.destroy(&state.dbpool).await {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(e) => e.respond(),
        },
        Ok(None) => (StatusCode::BAD_REQUEST, "No such user found.").into_response(),
        Err(e) => e.respond(),
    }
}

#[derive(Deserialize)]
struct ChangePassword {
    password: String,
}
async fn change_password(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
    Json(pass): Json<ChangePassword>,
) -> Response {
    let actor = match User::authenticate(&headers, cookies, &state.dbpool).await {
        Ok(u) => u,
        Err(e) => return e.respond(),
    };
    let target = match User::get_by_id(&id, &state.dbpool).await {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::BAD_REQUEST, "No such user found.").into_response(),
        Err(e) => return e.respond(),
    };

    if target.id == actor.id {
        if !actor.has_permission(UA::UsersChangeOwnPasswordPermission) {
            return StatusCode::FORBIDDEN.into_response();
        }
    } else {
        if !actor.has_permission(UA::UsersManagePasswordsPermission) {
            return StatusCode::FORBIDDEN.into_response();
        }
        if actor.clearance <= target.clearance {
            return StatusCode::FORBIDDEN.into_response();
        }
    }

    if let Err(e) = User::is_valid_password(&pass.password) {
        return OmniError::from(e).respond();
    }

    match target.patch_password(&pass.password, &state.dbpool).await {
        Ok(_) => (StatusCode::OK, "Password updated.").into_response(),
        Err(e) => e.respond(),
    }
}

async fn all_user_attributes() -> Response {
    Json(UA::VARIANTS).into_response()
}
