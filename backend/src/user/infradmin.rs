use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::{omnierror::OmniError, user::auth::crypto::generate_short_token};

use super::{attributes::UserAttribute, User};

impl User {
    pub fn is_infradmin(&self) -> bool {
        self.id.is_max()
    }
    fn new_infradmin() -> User {
        User {
            id: Uuid::max(),
            handle: "admin".to_string(),
            clearance: 255,
            attributes: UserAttribute::TheEverythingPermission.get_bit(),
            joindate: chrono::Utc::now(),
        }
    }
}

pub async fn guarantee_infradmin_exists(pool: &PgPool) {
    match sqlx::query!(
        "SELECT id, handle, clearance, attributes FROM users WHERE id = $1",
        Uuid::max()
    )
    .fetch_optional(pool)
    .await
    {
        Ok(Some(u)) => info!("Infradmin account (@{}) found.", u.handle),
        Ok(None) => {
            info!("No infradmin found.");
            let passw = generate_short_token();
            let admin = User::new_infradmin();
            match User::create(admin, &passw, pool).await {
                Ok(_) => {
                    info!("New infradmin account has been created!");
                    info!("Handle: admin; Password: {passw}");
                    info!("Please change these credentials as soon as possible.");
                }
                Err(e) => {
                    let err = OmniError::from(e);
                    error!("Could not create infradmin!");
                    error!("{err}");
                    panic!();
                }
            }
        }
        Err(e) => {
            let err = OmniError::from(e);
            error!("Could not guarantee infradmin exists!");
            error!("{err}");
            panic!();
        }
    }
}
