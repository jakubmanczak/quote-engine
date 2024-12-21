use crate::{
    auth::{
        auth_via_credentials, create_session, destroy_session,
        error::AuthenticationError::{
            NoHeaderAuthSchemeData, NonAsciiHeaderCharacters, SessionRemoveNonBearerHeader,
        },
        AUTH_COOKIE_NAME,
    },
    error::OmniError,
};
use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
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

async fn auth_clear(
    headers: HeaderMap,
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
) -> Response {
    let header = match headers.get(AUTHORIZATION) {
        Some(h) => match h.to_str() {
            Ok(s) => {
                let s = s.to_string();
                match s.split_once(' ') {
                    Some((scheme, data)) => {
                        if scheme == "Bearer" {
                            Some(data.to_string())
                        } else {
                            return OmniError::AuthError(SessionRemoveNonBearerHeader)
                                .log_and_respond();
                        }
                    }
                    None => return OmniError::AuthError(NoHeaderAuthSchemeData).log_and_respond(),
                }
            }
            Err(_) => return OmniError::AuthError(NonAsciiHeaderCharacters).log_and_respond(),
        },
        None => None,
    };
    let cookie = match cookies.get(AUTH_COOKIE_NAME) {
        Some(c) => Some(c.value().to_string()),
        None => None,
    };

    match (header, cookie) {
        (Some(header), Some(cookie)) => {
            // TODO: handle failures in removing sessions nicely
            if header != cookie {
                match destroy_session(header, &pool).await {
                    Ok(_) => (),
                    Err(e) => return e.log_and_respond(),
                };
            }
            match destroy_session(cookie, &pool).await {
                Ok(_) => (),
                Err(e) => return e.log_and_respond(),
            };
            cookies.add(Cookie::build(AUTH_COOKIE_NAME).removal().path("/").build());
            (StatusCode::OK, AUTH_SESSION_X_CLEAR_RESPONSE).into_response()
        }
        (Some(header), None) => {
            match destroy_session(header, &pool).await {
                Ok(_) => (),
                Err(e) => return e.log_and_respond(),
            };
            (StatusCode::OK, AUTH_SESSION_1_CLEAR_RESPONSE).into_response()
        }
        (None, Some(cookie)) => {
            match destroy_session(cookie, &pool).await {
                Ok(_) => (),
                Err(e) => return e.log_and_respond(),
            };
            cookies.add(Cookie::build(AUTH_COOKIE_NAME).removal().path("/").build());
            (StatusCode::OK, AUTH_SESSION_1_CLEAR_RESPONSE).into_response()
        }
        (None, None) => (StatusCode::OK, AUTH_SESSION_0_CLEAR_RESPONSE).into_response(),
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthLoginFields {
    username: String,
    password: String,
}
async fn auth_login(
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
    Json(body): Json<AuthLoginFields>,
) -> Response {
    let user = match auth_via_credentials(body.username, body.password, &pool).await {
        Ok(user) => user,
        Err(e) => return e.log_and_respond(),
    };

    let token = match create_session(user.id, &pool).await {
        Ok(token) => token,
        Err(e) => return e.log_and_respond(),
    };

    let c = Cookie::build((AUTH_COOKIE_NAME, token.clone()))
        .max_age(tower_cookies::cookie::time::Duration::weeks(2))
        .http_only(true)
        .path("/")
        .same_site(tower_cookies::cookie::SameSite::Strict)
        .secure(true)
        .build();
    cookies.add(c);

    (StatusCode::OK, token).into_response()
}
