use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::rngs::OsRng;
use sqlx::PgPool;
use uuid::Uuid;

use crate::omnierror::OmniError;

use super::User;

impl User {
    pub async fn get_by_id(id: &Uuid, pool: &PgPool) -> Result<Option<User>, OmniError> {
        match sqlx::query!(
            "SELECT id, handle, clearance, attributes FROM users WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await
        {
            Ok(Some(res)) => Ok(Some(User {
                id: res.id,
                handle: res.handle,
                clearance: res.clearance as u8,
                attributes: res.attributes as u64,
            })),
            Ok(None) => Ok(None),
            Err(err) => Err(err)?,
        }
    }
    pub async fn get_by_handle(handle: &str, pool: &PgPool) -> Result<Option<User>, OmniError> {
        match sqlx::query!(
            "SELECT id, handle, clearance, attributes FROM users WHERE handle = $1",
            handle
        )
        .fetch_optional(pool)
        .await
        {
            Ok(Some(res)) => Ok(Some(User {
                id: res.id,
                handle: res.handle,
                clearance: res.clearance as u8,
                attributes: res.attributes as u64,
            })),
            Ok(None) => Ok(None),
            Err(err) => Err(err)?,
        }
    }
    pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, OmniError> {
        match sqlx::query!("SELECT id, handle, clearance, attributes FROM users")
            .fetch_all(pool)
            .await
        {
            Ok(res) => Ok(res
                .into_iter()
                .map(|row| User {
                    id: row.id,
                    handle: row.handle,
                    clearance: row.clearance as u8,
                    attributes: row.attributes as u64,
                })
                .collect()),
            Err(err) => Err(err)?,
        }
    }
    pub async fn create(user: User, password: &str, pool: &PgPool) -> Result<User, OmniError> {
        let hash = {
            let argon = Argon2::default();
            let salt = SaltString::generate(&mut OsRng);
            match argon.hash_password(password.as_bytes(), &salt) {
                Ok(hash) => hash.to_string(),
                Err(err) => return Err(err)?,
            }
        };
        match sqlx::query!(
            "INSERT INTO users VALUES ($1, $2, $3, $4, $5)",
            user.id,
            user.handle,
            user.clearance as i32,
            user.attributes as i64,
            hash
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok(user),
            Err(err) => Err(err)?,
        }
    }
    pub async fn destroy(self, pool: &PgPool) -> Result<(), OmniError> {
        match sqlx::query!("DELETE FROM users WHERE id = $1", self.id)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err)?,
        }
    }
}
