use crate::db::users::{get_user_data, GetUserDataInput};
use crate::models::User;
use crate::{db::get_conn, error::Error};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use base64::{prelude::BASE64_STANDARD, Engine};
use sqlite::State;
use tracing::info;

const NO_AUTH_HEADER_FOUND: &str = "No Authorization header found";
const NO_AUTH_SCHEME_DATA: &str = "Could not get scheme, data from Authorization header";
const NO_BASIC_COLON_SPLIT: &str = "Could not split Authorization header Basic data colon-wise.";
const NO_USER_IN_DATABASE: &str = "Could not find matching user";
const NO_PARSE_PHC_STRING: &str = "Could not parse PHC string as password hash";
const NO_PASSWORD_MATCH: &str = "Password incorrect.";
const UNSUPPORTED_AUTH_SCHEME: &str = "Unsupported authorization scheme";

pub fn authenticate(headers: &HeaderMap) -> Result<User, Error> {
    let authstr = match headers.get(AUTHORIZATION) {
        Some(header) => String::from_utf8(header.as_bytes().to_vec())?,
        None => return Err(Error::RequestAuthError(NO_AUTH_HEADER_FOUND.to_string())),
    };
    let (scheme, data) = match authstr.split_once(' ') {
        Some(parts) => parts,
        None => return Err(Error::RequestAuthError(NO_AUTH_SCHEME_DATA.to_string())),
    };

    match scheme {
        "Basic" => {
            let (user, password) =
                match String::from_utf8(BASE64_STANDARD.decode(data)?)?.split_once(':') {
                    Some((user, password)) => (user.to_string(), password.to_string()),
                    None => return Err(Error::RequestAuthError(NO_BASIC_COLON_SPLIT.to_string())),
                };
            let hashstr: String = {
                let conn = get_conn();
                let q = "SELECT pass FROM users WHERE name = :name";
                let mut statement = conn.prepare(q).unwrap();
                statement.bind((":name", user.as_str())).unwrap();

                match statement.next() {
                    Ok(State::Row) => statement.read("pass").unwrap(),
                    Ok(State::Done) => {
                        return Err(Error::RequestAuthError(NO_USER_IN_DATABASE.to_string()))
                    }
                    Err(e) => return Err(Error::SqliteError(e)),
                }
            };
            let argon = Argon2::default();
            let hash = match PasswordHash::new(hashstr.as_str()) {
                Ok(h) => h,
                Err(e) => {
                    info!("Could not parse database PHC string as password hash: {e}");
                    return Err(Error::RequestAuthError(NO_PARSE_PHC_STRING.to_string()));
                }
            };

            match argon.verify_password(password.as_bytes(), &hash).is_ok() {
                true => return get_user_data(GetUserDataInput::Name(user)),
                false => return Err(Error::RequestAuthError(NO_PASSWORD_MATCH.to_string())),
            }
        }
        _ => Err(Error::RequestAuthError(UNSUPPORTED_AUTH_SCHEME.to_string())),
    }
}
