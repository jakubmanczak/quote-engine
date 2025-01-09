use sqlx::{Pool, Postgres};
use tracing::{error, info};

use crate::user::infradmin::guarantee_infradmin_exists;

pub async fn establish_connections() -> Pool<Postgres> {
    info!("Now attempting database connection.");
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = match Pool::connect(&db_url).await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to establish connection pool: {e}");
            panic!();
        }
    };
    info!("Connection with the database successful.");

    guarantee_infradmin_exists(&pool).await;

    pool
}
