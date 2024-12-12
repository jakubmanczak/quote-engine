use crate::{
    auth::{error::AuthenticationError::NonAsciiHeaderCharacters, AUTH_COOKIE_NAME},
    error::OmniError,
};
use axum::{
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Sqlite};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/auth/clear", get(auth_clear))
        .route("/auth/login", post(auth_login))
}

pub const AUTH_SESSION_1_CLEAR_RESPONSE: &str = "Provided session cleared.";
pub const AUTH_SESSION_X_CLEAR_RESPONSE: &str = "Provided sessions cleared.";
pub const AUTH_SESSION_0_CLEAR_RESPONSE: &str = "No sessions to clear provided.";

async fn auth_clear(headers: HeaderMap, cookies: Cookies) -> Response {
    let header = match headers.get(AUTHORIZATION) {
        Some(h) => match h.to_str() {
            Ok(s) => Some(s.to_owned()),
            Err(_) => return OmniError::AuthError(NonAsciiHeaderCharacters).log_and_respond(),
        },
        None => None,
    };
    let cookie = match cookies.get(AUTH_COOKIE_NAME) {
        Some(c) => Some(c.to_string()),
        None => None,
    };

    match (header, cookie) {
        (Some(header), Some(cookie)) => {
            // TODO: handle failures in removing sessions nicely
            if header != cookie {
                remove_session(header);
            }
            remove_session(cookie);
            cookies.add(Cookie::build(AUTH_COOKIE_NAME).removal().path("/").build());
            (StatusCode::OK, AUTH_SESSION_X_CLEAR_RESPONSE).into_response()
        }
        (Some(header), None) => {
            remove_session(header);
            (StatusCode::OK, AUTH_SESSION_1_CLEAR_RESPONSE).into_response()
        }
        (None, Some(cookie)) => {
            remove_session(cookie);
            cookies.add(Cookie::build(AUTH_COOKIE_NAME).removal().path("/").build());
            (StatusCode::OK, AUTH_SESSION_1_CLEAR_RESPONSE).into_response()
        }
        (None, None) => (StatusCode::OK, AUTH_SESSION_0_CLEAR_RESPONSE).into_response(),
    }
}

async fn auth_login() -> Response {
    todo!()
}

// TODO: implement sessions and actually make this a useful function in src/auth/mod.rs!
fn remove_session(a: String) {}
