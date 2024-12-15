use crate::{error::OmniError, users::User};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::http::{header::AUTHORIZATION, HeaderMap};
use base64::{prelude::BASE64_STANDARD, Engine};
use error::AuthenticationError;
use sqlx::{Pool, Sqlite};
use tower_cookies::{Cookie, Cookies};

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
                    let user = auth_via_session(data.to_string(), pool).await?;
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

async fn auth_via_b64_credentials(data: String, pool: &Pool<Sqlite>) -> Result<User, OmniError> {
    let (usr, pwd) = match String::from_utf8(BASE64_STANDARD.decode(data)?)?.split_once(":") {
        Some((usr, pwd)) => (usr.to_string(), pwd.to_string()),
        None => return Err(AuthenticationError::NoBasicAuthColonSplit.into()),
    };
    auth_via_credentials(usr, pwd, pool).await
}

pub async fn auth_via_credentials(
    username: String,
    password: String,
    pool: &Pool<Sqlite>,
) -> Result<User, OmniError> {
    let hashpass = match sqlx::query!("SELECT pass FROM users WHERE name = ?", username)
        .fetch_optional(pool)
        .await
    {
        Ok(a) => match a {
            Some(a) => a.pass,
            None => return Err(AuthenticationError::InvalidCredentials)?,
        },
        Err(e) => return Err(e)?,
    };
    let argon = Argon2::default();
    let hash = match PasswordHash::new(hashpass.as_str()) {
        Ok(h) => h,
        Err(e) => return Err(AuthenticationError::NoParsePHC)?,
    };

    match argon.verify_password(password.as_bytes(), &hash).is_ok() {
        true => match User::get_by_username(username.as_str(), pool).await {
            Ok(optuser) => match optuser {
                Some(user) => Ok(user),
                None => Err(AuthenticationError::InvalidCredentials)?,
            },
            Err(e) => Err(e)?,
        },
        false => Err(AuthenticationError::InvalidCredentials)?,
    }
}

pub async fn auth_via_session(_data: String, _pool: &Pool<Sqlite>) -> Result<User, OmniError> {
    todo!()
}
