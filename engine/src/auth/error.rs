use axum::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error("Invalid credentials.")]
    InvalidCredentials,
    #[error("No authorization data: provide cookie or header.")]
    NoAuthProvided,
    #[error("Bad AUTHORIZATION header: could not parse scheme/data.")]
    NoHeaderAuthSchemeData,
    #[error("Non-ASCII characters found in AUTHORIZATION header.")]
    NonAsciiHeaderCharacters,
    #[error("Could not split AUTHORIZATION header Basic data colon-wise.")]
    NoBasicAuthColonSplit,
    #[error("Unsupported authorization scheme.")]
    UnsupportedAuthScheme,
    #[error("Could not parse password PHC string")]
    NoParsePHC,
    #[error("Session expired.")]
    SessionExpired,
    #[error("Unable to represent expiry date as an i64, please time travel backwards.")]
    UnableToCreateExpiry,
    #[error("Can't remove sessions from non Bearer scheme headers.")]
    SessionRemoveNonBearerHeader,
}

impl AuthenticationError {
    pub fn suggested_status_code(&self) -> StatusCode {
        use AuthenticationError::*;
        match self {
            NoAuthProvided | InvalidCredentials | SessionExpired => StatusCode::UNAUTHORIZED,
            NoParsePHC | UnableToCreateExpiry => StatusCode::INTERNAL_SERVER_ERROR,
            NoHeaderAuthSchemeData
            | NonAsciiHeaderCharacters
            | NoBasicAuthColonSplit
            | UnsupportedAuthScheme
            | SessionRemoveNonBearerHeader => StatusCode::BAD_REQUEST,
        }
    }
}
