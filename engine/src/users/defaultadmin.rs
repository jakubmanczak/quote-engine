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
        Ok(record) => match record {
            Some(record) => {
                // sick, do nothing i guess!
            }
            None => {
                info!("admin account not present in database; adding...");
            }
        },
        Err(e) => {
            error!("could not check for users; could the database be corrupt?");
            error!("^^ error: {e}");
            panic!();
        }
    }
}
