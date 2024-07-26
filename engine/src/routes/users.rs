use crate::{
    auth::{get_auth_from_header, validate::validate_basic_auth, AuthType},
    db::get_conn,
    models::User,
};
use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use sqlite::State;
use tracing::error;

const NO_USERS: &str = "NO USERS FOUND.";

pub fn exported_routes() -> Router {
    Router::new().route("/users", get(get_users))
    // .route("/users", post(post_users))
}

async fn get_users(headers: HeaderMap) -> Response {
    match get_auth_from_header(&headers) {
        Some(auth) => match auth {
            AuthType::Basic(auth) => match validate_basic_auth(auth) {
                true => (),
                false => return StatusCode::UNAUTHORIZED.into_response(),
            },
            _ => return StatusCode::UNAUTHORIZED.into_response(),
        },
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let conn = get_conn();
    let query = "SELECT id, name FROM users";
    let mut statement = conn.prepare(query).unwrap();

    let mut users: Vec<User> = Vec::new();
    loop {
        match statement.next() {
            Ok(State::Row) => users.push(User {
                id: statement.read::<String, _>("id").unwrap(),
                name: statement.read::<String, _>("name").unwrap(),
            }),
            Ok(State::Done) => match users.is_empty() {
                true => return (StatusCode::NOT_FOUND, NO_USERS).into_response(),
                false => return Json(users).into_response(),
            },
            Err(e) => {
                error!("Error in GET /users: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }
}

// async fn post_users() -> Response {}
