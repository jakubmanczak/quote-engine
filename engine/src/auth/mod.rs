use crate::{error::OmniError, users::User};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::http::{header::AUTHORIZATION, HeaderMap};
use base64::{
    prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD},
    Engine,
};
use chrono::{Duration, Utc};
use error::AuthenticationError;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sqlx::{Pool, Sqlite};
use tower_cookies::{Cookie, Cookies};
use ulid::Ulid;

pub mod error;

pub const AUTH_COOKIE_NAME: &str = "qauth";
const AUTH_SESSION_LENGTH: Duration = Duration::weeks(2);

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
        Err(_) => return Err(AuthenticationError::NoParsePHC)?,
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

pub async fn auth_via_session(data: String, pool: &Pool<Sqlite>) -> Result<User, OmniError> {
    let now = Utc::now().timestamp();
    let (sessionid, userid, expiry) = match sqlx::query!(
        "SELECT id, user, expiry FROM sessions WHERE token = ?",
        data
    )
    .fetch_optional(pool)
    .await
    {
        Ok(row) => match row {
            Some(record) => (
                Ulid::from_string(&record.id)?,
                Ulid::from_string(&record.user)?,
                record.expiry,
            ),
            None => return Err(AuthenticationError::SessionExpired)?,
        },
        Err(e) => return Err(e)?,
    };

    if now >= expiry {
        return Err(AuthenticationError::SessionExpired)?;
    }

    let expiry = match Utc::now().checked_add_signed(AUTH_SESSION_LENGTH) {
        Some(e) => e.timestamp(),
        None => return Err(AuthenticationError::UnableToCreateExpiry)?,
    };
    let sessionid = sessionid.to_string();

    match sqlx::query!(
        "UPDATE sessions SET expiry = ? WHERE id = ?",
        expiry,
        sessionid
    )
    .execute(pool)
    .await
    {
        Ok(_) => (),
        Err(e) => return Err(e)?,
    };

    match User::get_by_id(userid, pool).await {
        Ok(useropt) => match useropt {
            Some(user) => Ok(user),
            None => Err(AuthenticationError::SessionExpired)?,
        },
        Err(e) => Err(e)?,
    }
}

pub async fn create_session(userid: Ulid, pool: &Pool<Sqlite>) -> Result<String, OmniError> {
    let token = {
        let secret = std::env::var("SECRET").unwrap();
        let seed = {
            let mut seed = [0u8; 32];
            let secret = secret.as_bytes();
            let timestamp = Utc::now().timestamp().to_ne_bytes();
            let mut entropy = [0u8; 32];
            StdRng::from_entropy().fill(&mut entropy);
            for (i, &byte) in secret.iter().enumerate() {
                seed[i % 32] ^= byte;
            }
            for (i, &byte) in timestamp.iter().enumerate() {
                seed[i % 32] ^= byte;
            }
            for (i, &byte) in entropy.iter().enumerate() {
                seed[i % 32] ^= byte;
            }
            seed
        };
        let mut rng = StdRng::from_seed(seed);
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        BASE64_URL_SAFE_NO_PAD.encode(bytes)
    };
    let dbtoken = token.clone();

    let session = Ulid::new();
    let sessionid = session.to_string();
    let now = Utc::now();
    let issued = now.timestamp();
    let lastaccess = now.timestamp();
    let userid = userid.to_string();
    let expiry = match now.checked_add_signed(AUTH_SESSION_LENGTH) {
        Some(e) => e.timestamp(),
        None => return Err(AuthenticationError::UnableToCreateExpiry)?,
    };

    match sqlx::query!(
        "INSERT INTO sessions VALUES (?, ?, ?, ?, ?, ?)",
        sessionid,
        dbtoken,
        userid,
        issued,
        expiry,
        lastaccess
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(token),
        Err(e) => Err(e)?,
    }
}

pub async fn destroy_session(token: String, pool: &Pool<Sqlite>) -> Result<(), OmniError> {
    match sqlx::query!("DELETE FROM sessions WHERE token = ?", token)
        .execute(pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e)?,
    }
}
