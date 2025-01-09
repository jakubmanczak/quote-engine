use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    omnierror::OmniError,
    user::auth::{
        self,
        crypto::{generate_token, hash_token},
    },
};

use super::error::AuthError;

#[derive(Serialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub issued: DateTime<Utc>,
    pub expiry: DateTime<Utc>,
    pub last_access: Option<DateTime<Utc>>,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        self.expiry <= Utc::now()
    }

    pub async fn get_by_id(id: &Uuid, pool: &PgPool) -> Result<Session, OmniError> {
        match sqlx::query_as!(
            Session,
            "SELECT id, user_id, issued, expiry, last_access FROM sessions WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await
        {
            Ok(s) => match s {
                Some(s) => Ok(s),
                // infosec: don't leak session info
                None => Err(AuthError::SessionExpired)?,
            },
            Err(e) => return Err(e)?,
        }
    }
    pub async fn get_by_token(token: &str, pool: &PgPool) -> Result<Session, OmniError> {
        let hashed_token = hash_token(token);
        match sqlx::query_as!(
            Session,
            "SELECT id, user_id, issued, expiry, last_access FROM sessions WHERE token = $1",
            &hashed_token
        )
        .fetch_optional(pool)
        .await
        {
            Ok(s) => match s {
                Some(s) => Ok(s),
                // infosec: don't leak session info
                None => Err(AuthError::SessionExpired)?,
            },
            Err(e) => return Err(e)?,
        }
    }
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Session>, OmniError> {
        match sqlx::query_as!(
            Session,
            "SELECT id, user_id, issued, expiry, last_access FROM sessions"
        )
        .fetch_all(pool)
        .await
        {
            Ok(s) => Ok(s),
            Err(e) => return Err(e)?,
        }
    }
    /// Ok(..) returns both the Session and the unhashed token as a String in a tuple
    pub async fn create(user_id: &Uuid, pool: &PgPool) -> Result<(Session, String), OmniError> {
        let id = Uuid::now_v7();
        let token = generate_token();
        let hashed_token = hash_token(&token);
        match sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions(id, token, user_id) VALUES ($1, $2, $3)
            RETURNING id, user_id, issued, expiry, last_access
            "#,
            &id,
            &hashed_token,
            user_id
        )
        .fetch_one(pool)
        .await
        {
            Ok(s) => Ok((s, token)),
            Err(e) => return Err(e)?,
        }
    }
    pub async fn destroy(self, pool: &PgPool) -> Result<(), OmniError> {
        match sqlx::query!("DELETE FROM sessions WHERE id = $1", self.id)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => return Err(e)?,
        }
    }

    /// Prolongs session expiry and updates last_access - to be called on every request
    pub async fn prolong_and_mark_access(self, pool: &PgPool) -> Result<Session, OmniError> {
        let last_access = Some(Utc::now());
        let expiry = Utc::now() + auth::SESSION_DURATION;
        match sqlx::query!(
            "UPDATE sessions SET expiry = $1, last_access = $2 WHERE id = $3",
            expiry,
            last_access,
            &self.id
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok(Session {
                expiry,
                last_access,
                ..self
            }),
            Err(e) => return Err(e)?,
        }
    }
}
