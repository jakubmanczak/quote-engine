use crate::database;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct SharedState {
    pub dbpool: PgPool,
}

pub async fn init() -> SharedState {
    SharedState {
        dbpool: database::establish_connections().await,
    }
}
