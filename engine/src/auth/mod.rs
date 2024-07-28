use axum::http::HeaderMap;
use decode_basic_auth::decode_basic_auth;
use header_parse::parse_auth_header_info;
use tracing::{error, info};

mod decode_basic_auth;
mod header_parse;

pub mod validate;

#[derive(Debug)]
pub enum AuthType {
    Basic(AuthBasic),
    JWT,
}

#[derive(Debug)]
pub struct AuthBasic {
    pub user: String,
    pub pass: String,
}

pub fn get_auth_from_header(headers: &HeaderMap) -> Option<AuthType> {
    let headerparts = match parse_auth_header_info(headers) {
        Ok(hp) => match hp.is_empty() {
            false => hp,
            true => return None,
        },
        Err(e) => {
            error!("Could not parse auth header info -> {e}");
            return None;
        }
    };

    let (scheme, data) = (&headerparts[0], &headerparts[1]);
    match scheme.as_str() {
        "Basic" => match decode_basic_auth(data.to_string()) {
            Ok(auth) => Some(AuthType::Basic(auth)),
            Err(e) => {
                error!("{e}");
                None
            }
        },
        "Bearer" => {
            info!("Bearer? -> {data}");
            None
        }
        _ => {
            info!("Unknown?! -> {scheme} -> {data}");
            None
        }
    }
}
