use std::sync::Arc;

use crate::database;
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct SharedState {
    pub dbpool: PgPool,
    pub sysinfo: Arc<RwLock<SystemInfo>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SystemInfo {
    pub cpu_used: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
}

pub async fn init() -> SharedState {
    SharedState {
        dbpool: database::establish_connections().await,
        sysinfo: Arc::new(RwLock::new(SystemInfo::default())),
    }
}
