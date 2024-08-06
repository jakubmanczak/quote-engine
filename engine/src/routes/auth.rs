use crate::auth::{authenticate, authenticate_via_basicauth, JsonWebTokenClaims, AUTH_COOKIE_NAME};
use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;
use tower_cookies::{Cookie, Cookies};

pub fn exported_routes() -> Router {
    Router::new()
        .route("/auth/check", get(check_login))
        .route("/users/self", get(check_login))
        .route("/auth/login", get(login_handler))
}

async fn check_login(headers: HeaderMap, cookies: Cookies) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    return Json(actor).into_response();
}

async fn login_handler(headers: HeaderMap, cookies: Cookies) -> Response {
    let actor = match authenticate_via_basicauth(&headers) {
        Ok(u) => u,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    let claims = JsonWebTokenClaims {
        exp: (Utc::now() + Duration::weeks(2)).timestamp(),
        iat: Utc::now().timestamp(),
        sub: actor.id,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("SECRET").unwrap().as_bytes()),
    ) {
        Ok(token) => token,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let c = Cookie::build((AUTH_COOKIE_NAME, token.clone()))
        .max_age(tower_cookies::cookie::time::Duration::weeks(2))
        .http_only(true)
        .secure(true)
        .build();

    cookies.add(c);
    return token.into_response();
}
