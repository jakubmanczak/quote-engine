use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use tower_cookies::Cookies;

use crate::{
    omnierror::OmniError,
    state::SharedState,
    user::{
        auth::{
            cookie::{clear_session_token_cookie, set_session_token_cookie},
            error::AuthError::{ClearSessionBearerOnly, NonAsciiHeaderCharacters},
            session::Session,
            SESSION_COOKIE_NAME,
        },
        User,
    },
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/clear", post(clear))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LoginData {
    #[serde(alias = "handle")]
    login: String,
    #[serde(alias = "password")]
    passw: String,
}
async fn login(
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(data): Json<LoginData>,
) -> Response {
    let user = match User::auth_via_credentials(&data.login, &data.passw, &state.dbpool).await {
        Ok(u) => u,
        Err(e) => return e.respond(),
    };

    let (_, token) = match Session::create(&user.id, &state.dbpool).await {
        Ok(s) => s,
        Err(e) => return e.respond(),
    };

    set_session_token_cookie(&token, cookies);
    (StatusCode::CREATED, token).into_response()
}

const TOO_MANY_TOKENS: &str = "Please provide one token at a time.";
const NO_TOKENS: &str = "Please provide a token.";
const SUCCESS: &str = "Logged out - session destroyed.";

async fn clear(headers: HeaderMap, cookies: Cookies, State(state): State<SharedState>) -> Response {
    let header = match headers.get(AUTHORIZATION) {
        Some(h) => match h.to_str() {
            Ok(s) => Some(s.to_string()),
            Err(_) => return OmniError::from(NonAsciiHeaderCharacters).respond(),
        },
        None => None,
    };
    let cookie = match cookies.get(SESSION_COOKIE_NAME) {
        Some(c) => {
            let c = c.value().to_string();
            match &header {
                Some(h) => match h == &c {
                    true => None,
                    false => Some(c),
                },
                None => Some(c),
            }
        }
        None => None,
    };

    match (header, cookie) {
        (Some(_), Some(_)) => (StatusCode::BAD_REQUEST, TOO_MANY_TOKENS).into_response(),
        (None, None) => (StatusCode::BAD_REQUEST, NO_TOKENS).into_response(),
        (None, Some(c)) => auth_clear_and_respond(&c, cookies, &state.dbpool).await,
        (Some(h), None) => {
            let (scheme, data) = match h.split_once(' ') {
                Some((s, d)) => (s, d),
                None => return OmniError::from(NonAsciiHeaderCharacters).respond(),
            };
            match scheme {
                "Bearer" => auth_clear_and_respond(data, cookies, &state.dbpool).await,
                _ => OmniError::from(ClearSessionBearerOnly).respond(),
            }
        }
    }
}

async fn auth_clear_and_respond(token: &str, cookies: Cookies, pool: &PgPool) -> Response {
    clear_session_token_cookie(cookies);
    match Session::get_by_token(token, pool).await {
        Ok(s) => match s.destroy(pool).await {
            Ok(_) => (StatusCode::OK, SUCCESS).into_response(),
            Err(e) => e.respond(),
        },
        Err(e) => e.respond(),
    }
}
