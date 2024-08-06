use core::fmt;
use std::{fmt::Display, string::FromUtf8Error};

#[derive(Debug)]
pub enum Error {
    // quotes-engine specific
    RequestAuthError(String),
    GetUserDataError(String),

    // imported
    DecodeError(base64::DecodeError),
    FromUtf8Error(FromUtf8Error),
    JsonWebTokenError(jsonwebtoken::errors::Error),

    // SQLite
    SqliteError(sqlite::Error),
    NoRowsError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RequestAuthError(err) => write!(f, "RequestAuthError: {}", err),
            Error::GetUserDataError(err) => write!(f, "GetUserDataError: {}", err),
            //
            Error::DecodeError(err) => write!(f, "{}", err),
            Error::FromUtf8Error(err) => write!(f, "{}", err),
            Error::JsonWebTokenError(err) => write!(f, "{}", err),
            //
            Error::SqliteError(err) => write!(f, "{}", err),
            Error::NoRowsError(err) => write!(f, "NoRowsError: {}", err),
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

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Error {
        Error::JsonWebTokenError(err)
    }
}

impl From<sqlite::Error> for Error {
    fn from(err: sqlite::Error) -> Error {
        Error::SqliteError(err)
    }
}
