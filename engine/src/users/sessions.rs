// use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Serialize;
use ulid::Ulid;

#[derive(Serialize)]
pub struct UserSession {
    pub id: Ulid,
    pub userid: Ulid,
    #[serde(skip_serializing)]
    pub token: String,
    pub issued: i64,
    pub expiry: i64,
    pub lastaccess: i64,
    // token - don't even store it.
    // #[serde(with = "ts_seconds")]
    // pub issued: DateTime<Utc>,
    // #[serde(with = "ts_seconds")]
    // pub expiry: DateTime<Utc>,
    // #[serde(with = "ts_seconds")]
    // pub lastaccess: DateTime<Utc>,
}
