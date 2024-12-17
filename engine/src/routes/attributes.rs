use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::{Pool, Sqlite};
use strum::VariantArray;

use crate::users::attributes::UserAttribute;

pub fn routes() -> Router<Pool<Sqlite>> {
    Router::new().route("/user-attributes", get(handler))
}

#[derive(Serialize)]
struct AttributeMeta {
    attribute: UserAttribute,
    bit: u8,
}
async fn handler() -> Response {
    let a = UserAttribute::VARIANTS
        .into_iter()
        .map(|a| AttributeMeta {
            attribute: a.to_owned(),
            bit: a.get_bit_offset(),
        })
        .collect::<Vec<AttributeMeta>>();

    Json(a).into_response()
}
