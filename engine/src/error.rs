use core::fmt;
use std::{fmt::Display, string::FromUtf8Error};

#[derive(Debug)]
pub enum Error {
    // quotes-engine specific
    HeaderParseError(String),
    BasicAuthError(String),

    // imported
    DecodeError(base64::DecodeError),
    FromUtf8Error(FromUtf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HeaderParseError(err) => write!(f, "HeaderParseError: {}", err),
            Error::BasicAuthError(err) => write!(f, "BasicAuthError: {}", err),
            //
            Error::DecodeError(err) => write!(f, "{}", err),
            Error::FromUtf8Error(err) => write!(f, "{}", err),
        }
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Error {
        Error::DecodeError(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::FromUtf8Error(err)
    }
}
