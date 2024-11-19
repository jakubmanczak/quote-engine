use crate::auth::{
    authenticate, create_user_session, destroy_user_session, validate_user_credentials,
    AUTH_COOKIE_NAME,
};
use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

pub fn exported_routes() -> Router {
    Router::new()
        .route("/auth/login", post(auth_login))
        .route("/auth/clear", get(auth_clear))
        .route("/auth/check", get(auth_check))
        .route("/users/self", get(auth_check))
}

async fn auth_check(headers: HeaderMap, cookies: Cookies) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => match e {
            crate::error::Error::AuthenticationError(err) => {
                return (err.suggested_status_code(), err.to_string()).into_response()
            }
            _ => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    };
    Json(actor).into_response()
}

async fn auth_clear(headers: HeaderMap, cookies: Cookies) -> Response {
    if cookies.get(AUTH_COOKIE_NAME).is_some() {
        match destroy_user_session(cookies.get(AUTH_COOKIE_NAME).unwrap().value().to_string()) {
            Ok(_) => (),
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }

    let c = Cookie::build(AUTH_COOKIE_NAME).removal().path("/").build();
    cookies.add(c);

    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
struct UserLoginCredentials {
    username: String,
    password: String,
}
async fn auth_login(cookies: Cookies, Json(body): Json<UserLoginCredentials>) -> Response {
    let user = match validate_user_credentials(body.username, body.password) {
        Ok(u) => u,
        Err(e) => match e {
            crate::error::Error::AuthenticationError(err) => {
                return (err.suggested_status_code(), err.to_string()).into_response()
            }
            _ => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    };

    let token = match create_user_session(user.id) {
        Ok(token) => token,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let c = Cookie::build((AUTH_COOKIE_NAME, token))
        .max_age(tower_cookies::cookie::time::Duration::weeks(2))
        .http_only(true)
        .path("/")
        .same_site(tower_cookies::cookie::SameSite::Strict)
        .secure(true)
        .build();
    cookies.add(c);

    return StatusCode::OK.into_response();
}
