use serde::Deserialize;
use sqlx::PgPool;

use crate::omnierror::OmniError;

use super::{auth::password::hash_password, User};

#[derive(Deserialize)]
pub struct UserPatch {
    pub handle: Option<String>,
    pub clearance: Option<u8>,
    // pub attributes: Option<u64>,
}

impl User {
    pub async fn patch(self, patch: UserPatch, pool: &PgPool) -> Result<User, OmniError> {
        let mut user = self.clone();
        if let Some(handle) = patch.handle {
            if let Err(e) = User::is_valid_handle(&handle) {
                return Err(OmniError::from(e));
            }
            user.handle = handle;
        }
        if let Some(clearance) = patch.clearance {
            user.clearance = clearance;
        }
        // if let Some(attributes) = patch.attributes {
        //     user.attributes = attributes;
        // }

        match sqlx::query!(
            "UPDATE users SET handle = $1, clearance = $2, attributes = $3 WHERE id = $4",
            &user.handle,
            user.clearance as i16,
            user.attributes as i64,
            &user.id
        )
        .execute(pool)
        .await
        {
            Ok(_) => (),
            Err(e) => return Err(e)?,
        }

        Ok(user)
    }
    pub async fn patch_password(&self, password: &str, pool: &PgPool) -> Result<(), OmniError> {
        let hash = hash_password(password)?;
        match sqlx::query!(
            "UPDATE users SET password_hash = $1 WHERE id = $2",
            hash,
            &self.id
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e)?,
        }
    }
}
