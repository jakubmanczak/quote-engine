// use sqlx::{Pool, Sqlite};

use axum::http::{header::AUTHORIZATION, HeaderMap};
use error::AuthenticationError;
use sqlx::{Pool, Sqlite};
use tower_cookies::{Cookie, Cookies};

use crate::{error::OmniError, users::User};

pub mod error;

pub const AUTH_COOKIE_NAME: &str = "qauth";

pub async fn authenticate(
    headers: &HeaderMap,
    cookies: Cookies,
    pool: &Pool<Sqlite>,
) -> Result<User, OmniError> {
    let cookie = match cookies.get(AUTH_COOKIE_NAME) {
        Some(cookie) => match cookie.value().is_empty() {
            true => None,
            false => Some(cookie.value().to_string()),
        },
        None => None,
    };
    let header = match headers.get(AUTHORIZATION) {
        Some(header) => match header.to_str() {
            Ok(header) => match header.is_empty() {
                true => None,
                false => Some(header.to_string()),
            },
            Err(_) => return Err(AuthenticationError::NonAsciiHeaderCharacters.into()),
        },
        None => None,
    };

    match (cookie, header) {
        (None, None) => Err(AuthenticationError::NoAuthProvided.into()),
        (_, Some(header)) => {
            let (scheme, data) = match header.split_once(' ') {
                Some((a, b)) => (a, b),
                None => return Err(AuthenticationError::NoHeaderAuthSchemeData.into()),
            };
            match scheme {
                "Basic" => auth_via_b64_credentials(data.to_string(), pool).await,
                "Bearer" => {
                    let user = auth_via_credentials(data.to_string(), pool).await?;
                    let c = Cookie::build((AUTH_COOKIE_NAME, data.to_string()))
                        .max_age(tower_cookies::cookie::time::Duration::weeks(2))
                        .http_only(true)
                        .path("/")
                        .same_site(tower_cookies::cookie::SameSite::Strict)
                        .secure(true)
                        .build();
                    cookies.add(c);
                    return Ok(user);
                }
                _ => Err(AuthenticationError::UnsupportedAuthScheme.into()),
            }
        }
        (Some(cookie), None) => {
            let user = auth_via_session(cookie.clone(), pool).await?;
            let c = Cookie::build((AUTH_COOKIE_NAME, cookie.to_string()))
                .max_age(tower_cookies::cookie::time::Duration::weeks(2))
                .http_only(true)
                .path("/")
                .same_site(tower_cookies::cookie::SameSite::Strict)
                .secure(true)
                .build();
            cookies.add(c);
            return Ok(user);
        }
    }
}

async fn auth_via_b64_credentials(_data: String, _pool: &Pool<Sqlite>) -> Result<User, OmniError> {
    todo!()
}

pub async fn auth_via_credentials(_data: String, _pool: &Pool<Sqlite>) -> Result<User, OmniError> {
    todo!()
}

pub async fn auth_via_session(_data: String, _pool: &Pool<Sqlite>) -> Result<User, OmniError> {
    todo!()
}
