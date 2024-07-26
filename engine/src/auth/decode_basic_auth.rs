use super::AuthBasic;
use crate::error::Error;
use base64::{prelude::BASE64_STANDARD, Engine};

pub fn decode_basic_auth(data: String) -> Result<AuthBasic, Error> {
    let decoded_bytes = BASE64_STANDARD.decode(data)?;
    let decoded_string = String::from_utf8(decoded_bytes)?;

    let decodedparts: Vec<&str> = decoded_string.split(':').collect();
    if decodedparts.len() != 2 {
        return Err(Error::BasicAuthError(String::from(
            "Decoded base64 did not have two parts separated by \":\"",
        )));
    }

    return Ok(AuthBasic {
        user: decodedparts[0].to_string(),
        pass: decodedparts[1].to_string(),
    });
}
