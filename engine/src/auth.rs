use crate::db::get_conn;
use crate::db::users::{get_user_data, GetUserDataInput};
use crate::models::User;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use base64::{prelude::BASE64_STANDARD, Engine};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlite::State;
use std::env;
use tower_cookies::Cookies;
use tracing::info;

pub const AUTH_COOKIE_NAME: &str = "qauth";

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("Invalid credentials.")]
    InvalidCredentials,
    #[error("No authorization data: provide cookie or header.")]
    NoAuthProvided,
    #[error("Bad AUTHORIZATION header: could not parse scheme/data.")]
    NoHeaderAuthSchemeData,
    #[error("Non-ASCII characters found in AUTHORIZATION header.")]
    NonAsciiHeaderCharacters,
    #[error("Could not split Authorization header Basic data colon-wise.")]
    NoBasicAuthColonSplit,
    #[error("Unsupported authorization scheme.")]
    UnsupportedAuthScheme,
    #[error("Could not parse password PHC string.")]
    NoParsePHC,
    #[error("Database error.")]
    DatabaseError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonWebTokenClaims {
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
}

pub fn authenticate(headers: &HeaderMap, cookies: Cookies) -> Result<User, anyhow::Error> {
    let auth_cookie = cookies
        .get(AUTH_COOKIE_NAME)
        .is_some_and(|c| !c.value().is_empty());
    let auth_header = headers.get(AUTHORIZATION).is_some();

    match (auth_header, auth_cookie) {
        (true, _) => authenticate_via_basicauth(headers),
        (false, true) => {
            let cookie = match cookies.get(AUTH_COOKIE_NAME) {
                Some(c) => c.value().to_string(),
                None => unreachable!(),
                // TODO: this doesn't work
                // Some(c) => match (c.http_only(), c.secure()) {
                //     (Some(true), Some(true)) => c.value().to_string(),
                //     (Some(false) | None, Some(true)) => {
                //         return Err(Error::RequestAuthError(COOKIE_NO_HTTPONLY.into()))
                //     }
                //     (Some(true), Some(false) | None) => {
                //         return Err(Error::RequestAuthError(COOKIE_NO_SECURE.into()))
                //     }
                //     (Some(false) | None, Some(false) | None) => {
                //         return Err(Error::RequestAuthError(COOKIE_NO_SECURE_NO_HTTPONLY.into()))
                //     }
                // },
            };

            authenticate_via_jwt(cookie)
        }
        (false, false) => Err(AuthenticationError::NoAuthProvided)?,
    }
}

pub fn authenticate_via_jwt(jwt: String) -> Result<User, anyhow::Error> {
    let token = decode::<JsonWebTokenClaims>(
        &jwt,
        &DecodingKey::from_secret(env::var("SECRET").unwrap().as_bytes()),
        &Validation::default(),
    )?;

    let user = get_user_data(GetUserDataInput::Id(token.claims.sub))?;

    Ok(user)
}

pub fn authenticate_via_basicauth(headers: &HeaderMap) -> Result<User, anyhow::Error> {
    let authstr = match headers.get(AUTHORIZATION) {
        Some(header) => String::from_utf8(header.as_bytes().to_vec())?,
        None => unreachable!(),
    };
    let (scheme, data) = match authstr.split_once(' ') {
        Some(parts) => parts,
        None => return Err(AuthenticationError::NoHeaderAuthSchemeData)?,
    };

    match scheme {
        "Basic" => {
            let (user, password) =
                match String::from_utf8(BASE64_STANDARD.decode(data)?)?.split_once(':') {
                    Some((user, password)) => (user.to_string(), password.to_string()),
                    None => return Err(AuthenticationError::NoBasicAuthColonSplit)?,
                };
            let hashstr: String = {
                let conn = get_conn();
                let q = "SELECT pass FROM users WHERE name = :name";
                let mut statement = conn.prepare(q).unwrap();
                statement.bind((":name", user.as_str())).unwrap();

                match statement.next() {
                    Ok(State::Row) => statement.read("pass").unwrap(),
                    Ok(State::Done) => return Err(AuthenticationError::InvalidCredentials)?,
                    Err(e) => return Err(AuthenticationError::DatabaseError)?,
                }
            };
            let argon = Argon2::default();
            let hash = match PasswordHash::new(hashstr.as_str()) {
                Ok(h) => h,
                Err(e) => {
                    info!("Could not parse database PHC string as password hash: {e}");
                    return Err(AuthenticationError::NoParsePHC)?;
                }
            };

            match argon.verify_password(password.as_bytes(), &hash).is_ok() {
                true => return Ok(get_user_data(GetUserDataInput::Name(user))?),
                false => return Err(AuthenticationError::InvalidCredentials)?,
            }
        }
        _ => Err(AuthenticationError::UnsupportedAuthScheme)?,
    }
}
