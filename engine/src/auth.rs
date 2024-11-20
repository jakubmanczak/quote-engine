use crate::db::get_conn;
use crate::db::users::{get_user_data, GetUserDataInput};
use crate::error::Error;
use crate::models::User;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, StatusCode};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::{Duration, Utc};
use rand::Rng;
use sqlite::State;
use tower_cookies::{Cookie, Cookies};
use ulid::Ulid;

pub const AUTH_COOKIE_NAME: &str = "qauth";
pub const AUTH_SESSION_LENGTH: Duration = Duration::weeks(2);

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
    #[error("Session expired.")]
    SessionExpired,
    #[error("Unable to represent expiry date in i64.")]
    UnableToCreateExpiry,
}

impl AuthenticationError {
    pub fn suggested_status_code(&self) -> StatusCode {
        use AuthenticationError::*;
        match self {
            InvalidCredentials | SessionExpired => StatusCode::UNAUTHORIZED,
            NoParsePHC | DatabaseError | UnableToCreateExpiry => StatusCode::INTERNAL_SERVER_ERROR,
            NoAuthProvided
            | NoHeaderAuthSchemeData
            | NonAsciiHeaderCharacters
            | NoBasicAuthColonSplit
            | UnsupportedAuthScheme => StatusCode::BAD_REQUEST,
        }
    }
}

pub fn authenticate(headers: &HeaderMap, cookies: Cookies) -> Result<User, Error> {
    let cookie = match cookies.get(AUTH_COOKIE_NAME) {
        Some(cookie) => match !cookie.value().is_empty() {
            true => Some(cookie.value().to_string()),
            false => None,
        },
        None => None,
    };
    let header = match headers.get(AUTHORIZATION) {
        Some(header) => Some(match header.to_str() {
            Ok(str) => str.to_string(),
            Err(_) => return Err(AuthenticationError::NonAsciiHeaderCharacters)?,
        }),
        None => None,
    };

    match (cookie, header) {
        (_, Some(header)) => {
            let (scheme, data) = match header.split_once(' ') {
                Some((a, b)) => (a, b),
                None => return Err(AuthenticationError::NoHeaderAuthSchemeData)?,
            };
            match scheme {
                "Basic" => validate_user_base64_credentials(data.to_string()),
                "Bearer" => {
                    let user = validate_user_session(data.to_string())?;
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
                _ => return Err(AuthenticationError::UnsupportedAuthScheme)?,
            }
        }
        (Some(cookie), None) => {
            let user = validate_user_session(cookie.clone())?;
            let c = Cookie::build((AUTH_COOKIE_NAME, cookie))
                .max_age(tower_cookies::cookie::time::Duration::weeks(2))
                .http_only(true)
                .path("/")
                .same_site(tower_cookies::cookie::SameSite::Strict)
                .secure(true)
                .build();
            cookies.add(c);
            return Ok(user);
        }
        (None, None) => Err(AuthenticationError::NoAuthProvided)?,
    }
}

pub fn validate_user_base64_credentials(credentials: String) -> Result<User, Error> {
    let (usr, pwd) = match String::from_utf8(BASE64_STANDARD.decode(credentials)?)?.split_once(":")
    {
        Some((user, password)) => (user.to_string(), password.to_string()),
        None => return Err(AuthenticationError::NoBasicAuthColonSplit)?,
    };

    validate_user_credentials(usr, pwd)
}

pub fn validate_user_credentials(username: String, password: String) -> Result<User, Error> {
    let hashstr: String = {
        let conn = get_conn();
        let q = "SELECT pass FROM users WHERE name = :name";
        let mut statement = conn.prepare(q).unwrap();
        statement.bind((":name", username.as_str())).unwrap();

        match statement.next() {
            Ok(State::Row) => statement.read("pass").unwrap(),
            Ok(State::Done) => return Err(AuthenticationError::InvalidCredentials)?,
            Err(e) => return Err(AuthenticationError::DatabaseError)?,
        }
    };
    let argon = Argon2::default();
    let hash = match PasswordHash::new(hashstr.as_str()) {
        Ok(h) => h,
        Err(e) => return Err(AuthenticationError::NoParsePHC)?,
    };

    match argon.verify_password(password.as_bytes(), &hash).is_ok() {
        true => get_user_data(GetUserDataInput::Name(username)),
        false => Err(AuthenticationError::InvalidCredentials)?,
    }
}

pub fn validate_user_session(token: String) -> Result<User, Error> {
    let now = Utc::now().timestamp();
    let (id, userid, expiry) = {
        let conn = get_conn();
        let q = "SELECT id, user, expiry FROM sessions WHERE token = :token";
        let mut st = conn.prepare(q).unwrap();
        st.bind((":token", token.as_str())).unwrap();

        let id: String;
        let userid: String;
        let expiry: i64;
        match st.next() {
            Ok(State::Row) => {
                id = st.read("id").unwrap();
                expiry = st.read("expiry").unwrap();
                userid = st.read("user").unwrap();
                if now >= expiry {
                    return Err(AuthenticationError::SessionExpired)?;
                }
                (id, userid, expiry)
            }
            Ok(State::Done) => return Err(AuthenticationError::SessionExpired)?,
            Err(e) => return Err(AuthenticationError::DatabaseError)?,
        }
    };

    let expiry = match Utc::now().checked_add_signed(AUTH_SESSION_LENGTH) {
        Some(a) => a.timestamp(),
        None => return Err(AuthenticationError::UnableToCreateExpiry)?,
    };

    {
        let conn = get_conn();
        let q = "UPDATE sessions SET expiry = :expiry WHERE id = :id";
        let mut st = conn.prepare(q).unwrap();
        st.bind((":expiry", expiry)).unwrap();
        st.bind((":id", id.as_str())).unwrap();

        match st.next() {
            Ok(_) => (),
            Err(e) => return Err(AuthenticationError::DatabaseError)?,
        }
    };

    get_user_data(GetUserDataInput::Id(userid))
}

pub fn create_user_session(user: Ulid) -> Result<String, Error> {
    let id = Ulid::new();
    let now = Utc::now();
    let issued = now.timestamp();
    let expiry = match now.checked_add_signed(AUTH_SESSION_LENGTH) {
        Some(a) => a.timestamp(),
        None => return Err(AuthenticationError::UnableToCreateExpiry)?,
    };

    let mut rng = rand::thread_rng();
    let token: [u8; 32] = rng.gen();
    let token = String::from(BASE64_STANDARD.encode(token));

    let conn = get_conn();
    let q = "INSERT INTO sessions VALUES (:id, :token, :user, :issued, :expiry, :lastaccess)";
    let mut statement = conn.prepare(q).unwrap();
    statement.bind((":id", id.to_string().as_str())).unwrap();
    statement.bind((":token", token.as_str())).unwrap();
    statement
        .bind((":user", user.to_string().as_str()))
        .unwrap();
    statement.bind((":issued", issued)).unwrap();
    statement.bind((":expiry", expiry)).unwrap();
    statement.bind((":lastaccess", issued)).unwrap();

    match statement.next() {
        Ok(_) => Ok(token),
        Err(e) => return Err(AuthenticationError::DatabaseError)?,
    }
}

pub fn destroy_user_session(token: String) -> Result<(), Error> {
    let conn = get_conn();
    let q = "DELETE FROM sessions WHERE token = :token";
    let mut st = conn.prepare(q).unwrap();
    st.bind((":token", token.as_str())).unwrap();

    match st.next() {
        Ok(_) => Ok(()),
        Err(_) => Err(AuthenticationError::DatabaseError)?,
    }
}
