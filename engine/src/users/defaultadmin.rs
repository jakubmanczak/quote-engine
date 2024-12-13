use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::rngs::OsRng;
use sqlx::{Pool, Sqlite};
use tracing::{error, info};
use ulid::Ulid;

pub async fn check_lack_of_account(pool: &Pool<Sqlite>) {
    let id = Ulid::nil().to_string();
    match sqlx::query!(
        "SELECT id, name, clearance, attributes, color, picture FROM users WHERE id = ?",
        id
    )
    .fetch_optional(pool)
    .await
    {
        Ok(record) => {
            if record.is_none() {
                info!("admin account not present in database; adding...");

                let password = b"admin";
                let salt = SaltString::generate(&mut OsRng);
                let argon = Argon2::default();
                let hash = match argon.hash_password(password, &salt) {
                    Ok(hash) => hash.to_string(),
                    Err(e) => {
                        error!("could not hash default admin password; {e}");
                        panic!();
                    }
                };

                match sqlx::query!(
                    "INSERT INTO users (id, name, pass, clearance, attributes, color, picture) VALUES (?, 'admin', ?, 255, 1, '#000000', '')",
                    id, hash
                ).execute(pool).await {
                    Ok(_) => {
                        info!("admin account added to database");
                    }
                    Err(e) => {
                        error!("could not add admin account to database; {e}");
                        panic!();
                    }
                }
            }
        }
        Err(e) => {
            error!("could not check for users; could the database be corrupt?");
            error!("^^ error: {e}");
            panic!();
        }
    }
}
