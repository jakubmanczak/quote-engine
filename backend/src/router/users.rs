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
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::UsersManualCreatePermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    if let Err(e) = User::is_valid_handle(&user.handle) {
        return Err(e)?;
    }
    if let Err(e) = User::is_valid_password(&user.password) {
        return Err(e)?;
    }

    let nu = User::create(
        User::new_incomplete(user.handle),
        &user.password,
        &state.dbpool,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(nu)).into_response())
}

async fn get_user_by_id(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::UsersInspectPermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    match User::get_by_id(&id, &state.dbpool).await? {
        Some(user) => Ok(Json(user).into_response()),
        None => Ok((StatusCode::BAD_REQUEST, "No such user found.").into_response()),
    }
}

async fn patch_user(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
    Json(patch): Json<UserPatch>,
) -> Result<Response, OmniError> {
    let actor = User::authenticate(&headers, cookies, &state.dbpool).await?;
    let target = match User::get_by_id(&id, &state.dbpool).await? {
        Some(u) => u,
        None => return Ok((StatusCode::BAD_REQUEST, "No such user found.").into_response()),
    };

    if actor.id == target.id {
        if patch.handle.is_some() && !actor.has_permission(UA::UsersChangeOwnHandlePermission) {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }
    } else {
        if actor.clearance <= target.clearance {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }
        if patch.handle.is_some() && !actor.has_permission(UA::UsersManageHandlesPermission) {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }
    }

    if let Some(clearance) = patch.clearance {
        if !actor.has_permission(UA::UsersManageClearancesPermission) {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }
        if clearance > actor.clearance && !actor.has_permission(UA::TheEverythingPermission) {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }
    }

    if let Some(handle) = &patch.handle {
        if let Err(e) = User::is_valid_handle(handle) {
            return Err(e)?;
        }
    }

    Ok(Json(target.patch(patch, &state.dbpool).await?).into_response())
}

const DELCLEAR: &str = "Cannot delete a user with higher clearance.";
const DELADMIN: &str = "Cannot delete the infrastructure administrator.";

async fn delete_user(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
) -> Result<Response, OmniError> {
    let u = User::authenticate(&headers, cookies, &state.dbpool).await?;
    if !u.has_permission(UA::UsersDeletePermission) {
        return Ok(StatusCode::FORBIDDEN.into_response());
    }

    match User::get_by_id(&id, &state.dbpool).await? {
        Some(target) => {
            if target.is_infradmin() {
                return Ok((StatusCode::FORBIDDEN, DELADMIN).into_response());
            }
            if target.clearance >= u.clearance {
                return Ok((StatusCode::FORBIDDEN, DELCLEAR).into_response());
            }
            target.destroy(&state.dbpool).await?;
            Ok(StatusCode::NO_CONTENT.into_response())
        }
        None => Ok((StatusCode::BAD_REQUEST, "No such user found.").into_response()),
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
) -> Result<Response, OmniError> {
    let actor = User::authenticate(&headers, cookies, &state.dbpool).await?;
    let target = match User::get_by_id(&id, &state.dbpool).await? {
        Some(u) => u,
        None => return Ok((StatusCode::BAD_REQUEST, "No such user found.").into_response()),
    };

    if target.id == actor.id {
        if !actor.has_permission(UA::UsersChangeOwnPasswordPermission) {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }
    } else {
        if !actor.has_permission(UA::UsersManagePasswordsPermission) {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }
        if actor.clearance <= target.clearance {
            return Ok(StatusCode::FORBIDDEN.into_response());
        }
    }

    if let Err(e) = User::is_valid_password(&pass.password) {
        return Err(e)?;
    }

    target.patch_password(&pass.password, &state.dbpool).await?;
    Ok((StatusCode::OK, "Password updated.").into_response())
}

async fn all_user_attributes() -> Response {
    Json(UA::VARIANTS).into_response()
}
