use axum::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Session expired")]
    SessionExpired,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("No credentials provided")]
    NoCredentials,

    #[error("Non-ASCII characters found in AUTHORIZATION header")]
    NonAsciiHeaderCharacters,
    #[error("Base64 Basic Auth header is missing login/password colon separator")]
    NoBasicAuthColonSplit,
    #[error("Could not parse header auth scheme/data")]
    BadHeaderAuthSchemeData,
    #[error("Unsupported header auth scheme - use Basic or Bearer")]
    UnsupportedHeaderAuthScheme,
    #[error("Can only clear sessions via Bearer token requests")]
    ClearSessionBearerOnly,
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        use AuthError as E;
        use StatusCode as C;
        match self {
            E::InvalidCredentials | E::NoCredentials | E::SessionExpired => C::UNAUTHORIZED,
            E::NonAsciiHeaderCharacters
            | E::NoBasicAuthColonSplit
            | E::BadHeaderAuthSchemeData
            | E::UnsupportedHeaderAuthScheme
            | E::ClearSessionBearerOnly => C::BAD_REQUEST,
        }
    }
}
