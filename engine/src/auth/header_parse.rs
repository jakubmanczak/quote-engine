use crate::error::Error;
use axum::http::{header::AUTHORIZATION, HeaderMap};
use tracing::error;

pub fn parse_auth_header_info(headers: &HeaderMap) -> Result<Vec<String>, Error> {
    let header = match headers.contains_key(AUTHORIZATION) {
        true => match String::from_utf8(headers.get(AUTHORIZATION).unwrap().as_bytes().to_vec()) {
            Ok(authstr) => authstr,
            Err(e) => {
                let msg = "failed to parse string from AUTHORIZATION header";
                error!(
                    "{msg} -> {e} -> offending header: {:?}",
                    headers.get(AUTHORIZATION).unwrap()
                );
                return Err(Error::from(e));
            }
        },
        false => return Ok(Vec::new()),
    };

    let headerparts: Vec<String> = header.split(' ').map(|s| s.to_string()).collect();

    match headerparts.len() {
        2 => return Ok(headerparts),
        _ => {
            return Err(Error::HeaderParseError(format!(
                "headerparts.len is not 2 => {header}"
            )))
        }
    }
}
