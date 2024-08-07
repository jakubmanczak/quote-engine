use crate::{
    auth::{authenticate, JsonWebTokenClaims, AUTH_COOKIE_NAME},
    db::{
        get_conn,
        users::{get_user_data, GetUserDataInput},
    },
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use sqlite::State;
use std::env;
use tower_cookies::{cookie::SameSite::Strict, Cookie, Cookies};
use tracing::info;

pub fn exported_routes() -> Router {
    Router::new()
        .route("/auth/check", get(check_login))
        .route("/users/self", get(check_login))
        .route("/auth/login", post(login))
        .route("/auth/clear", get(clear_auth))
}

async fn check_login(headers: HeaderMap, cookies: Cookies) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    return Json(actor).into_response();
}

async fn clear_auth(cookies: Cookies) -> Response {
    let c = Cookie::build(AUTH_COOKIE_NAME)
        .removal()
        .http_only(true)
        .path("/")
        .build();
    cookies.add(c);

    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
struct UserLoginCredentials {
    username: String,
    password: String,
}
async fn login(cookies: Cookies, Json(body): Json<UserLoginCredentials>) -> Response {
    const INCORRECT_CREDENTIALS: &str = "Incorrect credentials.";
    let hashstr: String = {
        let conn = get_conn();
        let q = "SELECT pass FROM users WHERE name = :name";
        let mut statement = conn.prepare(q).unwrap();
        statement.bind((":name", body.username.as_str())).unwrap();

        match statement.next() {
            Ok(State::Row) => statement.read("pass").unwrap(),
            Ok(State::Done) => {
                return (StatusCode::UNAUTHORIZED, INCORRECT_CREDENTIALS).into_response()
            }
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    };
    let argon = Argon2::default();
    let hash = match PasswordHash::new(hashstr.as_str()) {
        Ok(h) => h,
        Err(e) => {
            info!("Could not parse database PHC string as password hash: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not parse phc string from db",
            )
                .into_response();
        }
    };

    let actor = match argon
        .verify_password(body.password.as_bytes(), &hash)
        .is_ok()
    {
        true => match get_user_data(GetUserDataInput::Name(body.username)) {
            Ok(actor) => actor,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
        false => return (StatusCode::UNAUTHORIZED, INCORRECT_CREDENTIALS).into_response(),
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
        .path("/")
        .same_site(Strict)
        // .secure(true)
        .build();

    cookies.add(c);
    return token.into_response();
}
