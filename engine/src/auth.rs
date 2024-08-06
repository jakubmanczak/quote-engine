use crate::db::users::{get_user_data, GetUserDataInput};
use crate::models::User;
use crate::{db::get_conn, error::Error};
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

const NO_AUTH_HEADER_FOUND: &str = "No Authorization header found";
const NO_AUTH_SCHEME_DATA: &str = "Could not get scheme, data from Authorization header";
const NO_BASIC_COLON_SPLIT: &str = "Could not split Authorization header Basic data colon-wise.";
const NO_USER_IN_DATABASE: &str = "Could not find matching user";
const NO_PARSE_PHC_STRING: &str = "Could not parse PHC string as password hash";
const NO_PASSWORD_MATCH: &str = "Password incorrect.";
const UNSUPPORTED_AUTH_SCHEME: &str = "Unsupported authorization scheme";
// const NO_COOKIE: &str = "No quoteauth cookie found.";
const NEITHER_HEADER_NOR_COOKIE: &str = "Neither an Authorization header nor a cookie were found.";
const COOKIE_NO_SECURE: &str = "qauth cookie is not marked as Secure";
const COOKIE_NO_HTTPONLY: &str = "quath cookie is not marked as HttpOnly";
const COOKIE_NO_SECURE_NO_HTTPONLY: &str = "quath cookie is not marked as Secure or HttpOnly";

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonWebTokenClaims {
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
}

pub fn authenticate(headers: &HeaderMap, cookies: Cookies) -> Result<User, Error> {
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
        (false, false) => Err(Error::RequestAuthError(NEITHER_HEADER_NOR_COOKIE.into())),
    }
}

pub fn authenticate_via_jwt(jwt: String) -> Result<User, Error> {
    let token = decode::<JsonWebTokenClaims>(
        &jwt,
        &DecodingKey::from_secret(env::var("SECRET").unwrap().as_bytes()),
        &Validation::default(),
    )?;

    let user = get_user_data(GetUserDataInput::Id(token.claims.sub))?;

    Ok(user)
}

pub fn authenticate_via_basicauth(headers: &HeaderMap) -> Result<User, Error> {
    let authstr = match headers.get(AUTHORIZATION) {
        Some(header) => String::from_utf8(header.as_bytes().to_vec())?,
        None => return Err(Error::RequestAuthError(NO_AUTH_HEADER_FOUND.into())),
    };
    let (scheme, data) = match authstr.split_once(' ') {
        Some(parts) => parts,
        None => return Err(Error::RequestAuthError(NO_AUTH_SCHEME_DATA.into())),
    };

    match scheme {
        "Basic" => {
            let (user, password) =
                match String::from_utf8(BASE64_STANDARD.decode(data)?)?.split_once(':') {
                    Some((user, password)) => (user.to_string(), password.to_string()),
                    None => return Err(Error::RequestAuthError(NO_BASIC_COLON_SPLIT.into())),
                };
            let hashstr: String = {
                let conn = get_conn();
                let q = "SELECT pass FROM users WHERE name = :name";
                let mut statement = conn.prepare(q).unwrap();
                statement.bind((":name", user.as_str())).unwrap();

                match statement.next() {
                    Ok(State::Row) => statement.read("pass").unwrap(),
                    Ok(State::Done) => {
                        return Err(Error::RequestAuthError(NO_USER_IN_DATABASE.into()))
                    }
                    Err(e) => return Err(Error::SqliteError(e)),
                }
            };
            let argon = Argon2::default();
            let hash = match PasswordHash::new(hashstr.as_str()) {
                Ok(h) => h,
                Err(e) => {
                    info!("Could not parse database PHC string as password hash: {e}");
                    return Err(Error::RequestAuthError(NO_PARSE_PHC_STRING.into()));
                }
            };

            match argon.verify_password(password.as_bytes(), &hash).is_ok() {
                true => return get_user_data(GetUserDataInput::Name(user)),
                false => return Err(Error::RequestAuthError(NO_PASSWORD_MATCH.into())),
            }
        }
        _ => Err(Error::RequestAuthError(UNSUPPORTED_AUTH_SCHEME.into())),
    }
}
