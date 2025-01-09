use axum::http::{header::AUTHORIZATION, HeaderMap};
use base64::{prelude::BASE64_STANDARD, Engine};
use sqlx::PgPool;
use tower_cookies::Cookies;

use crate::{
    omnierror::OmniError,
    user::{auth::cookie::set_session_token_cookie, User},
};

use super::{error::AuthError, password::verify_password, session::Session, SESSION_COOKIE_NAME};

impl User {
    pub async fn authenticate(
        headers: &HeaderMap,
        cookies: Cookies,
        pool: &PgPool,
    ) -> Result<User, OmniError> {
        let cookie = match cookies.get(SESSION_COOKIE_NAME) {
            Some(c) => match c.value().is_empty() {
                true => None,
                false => Some(c.value().to_string()),
            },
            None => None,
        };
        let header = match headers.get(AUTHORIZATION) {
            Some(h) => match h.to_str() {
                Ok(s) => Some(s.to_string()),
                Err(_) => return Err(AuthError::NonAsciiHeaderCharacters)?,
            },
            None => None,
        };

        match (cookie, header) {
            (None, None) => Err(AuthError::NoCredentials)?,
            (_, Some(header)) => {
                let (scheme, data) = match header.split_once(' ') {
                    Some((s, d)) => (s, d),
                    None => return Err(AuthError::BadHeaderAuthSchemeData)?,
                };
                match scheme {
                    "Basic" => User::auth_via_credentials_b64(data, pool).await,
                    "Bearer" => User::auth_via_session(data, cookies, pool).await,
                    _ => Err(AuthError::UnsupportedHeaderAuthScheme)?,
                }
            }
            (Some(cookie), None) => User::auth_via_session(&cookie, cookies, pool).await,
        }
    }
    async fn auth_via_credentials_b64(credentials: &str, pool: &PgPool) -> Result<User, OmniError> {
        let (login, passw) =
            match String::from_utf8(BASE64_STANDARD.decode(credentials)?)?.split_once(":") {
                Some((l, p)) => (l.to_string(), p.to_string()),
                None => return Err(AuthError::NoBasicAuthColonSplit)?,
            };
        User::auth_via_credentials(&login, &passw, pool).await
    }
    pub async fn auth_via_credentials(
        login: &str,
        passw: &str,
        pool: &PgPool,
    ) -> Result<User, OmniError> {
        let hash = match sqlx::query!("SELECT password_hash FROM users WHERE handle = $1", login)
            .fetch_optional(pool)
            .await?
        {
            Some(rec) => rec.password_hash,
            None => return Err(AuthError::InvalidCredentials)?,
        };
        match verify_password(passw, &hash) {
            Ok(true) => match User::get_by_handle(login, pool).await {
                Ok(Some(u)) => Ok(u),
                Ok(None) => Err(AuthError::InvalidCredentials)?,
                Err(e) => Err(e)?,
            },
            Ok(false) => Err(AuthError::InvalidCredentials)?,
            Err(e) => Err(e)?,
        }
    }
    async fn auth_via_session(
        token: &str,
        cookies: Cookies,
        pool: &PgPool,
    ) -> Result<User, OmniError> {
        let s = Session::get_by_token(token, pool).await?;
        match s.is_expired() {
            true => Err(AuthError::SessionExpired)?,
            false => {
                let user = match User::get_by_id(&s.user_id, pool).await? {
                    Some(u) => u,
                    None => return Err(AuthError::InvalidCredentials)?,
                };
                s.prolong_and_mark_access(pool).await?;
                set_session_token_cookie(token, cookies);
                Ok(user)
            }
        }
    }
}
