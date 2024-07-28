use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

use crate::permissions::USER_PERMISSIONS;

pub fn exported_routes() -> Router {
    Router::new().route("/permissions", get(permissions))
}

async fn permissions() -> Response {
    Json(USER_PERMISSIONS).into_response()
}
