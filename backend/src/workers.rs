use std::time::Duration;

use sysinfo::System;
use tokio::{spawn, time::sleep};
use tracing::info;

use crate::state::{SharedState, SystemInfo};

pub fn init(state: SharedState) {
    info!("Spawning thread workers...");
    spawn(async { system_health_diagnostics(state).await });
}

async fn system_health_diagnostics(state: SharedState) {
    info!("System health diagnostics thread worker ready!");
    let mut system = System::new();
    system.refresh_cpu_all();
    loop {
        system.refresh_memory();
        system.refresh_cpu_usage();
        let sysinfo = SystemInfo {
            cpu_used: system.global_cpu_usage(),
            mem_used: system.used_memory(),
            mem_total: system.total_memory(),
            swap_used: system.used_swap(),
            swap_total: system.total_swap(),
        };

        let mut lock = state.sysinfo.write().await;
        *lock = sysinfo;
        drop(lock);

        sleep(Duration::from_millis(250)).await;
    }
}
