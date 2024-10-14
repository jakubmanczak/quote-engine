use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use strum::VariantArray;

use crate::permissions::UserPermission;

pub fn exported_routes() -> Router {
    Router::new().route("/permissions", get(permissions))
}

async fn permissions() -> Response {
    Json(UserPermission::VARIANTS).into_response()
}
