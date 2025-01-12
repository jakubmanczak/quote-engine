use tokio::net::TcpListener;
use tracing::{error, info, warn, Level};
use tracing_subscriber::EnvFilter;

const DEFAULT_LOG_LEVEL: Level = Level::INFO;

const SETUP_DONE: &str = "Quote Engine ready! Spinning up listener...";

const SECRET_OKAY: &str = "Cryptographic SECRET is set.";
const SECRET_ERROR: &str = "Cryptographic SECRET could not be read. Is it valid UTF-8?";
const SECRET_UNSET: &str = "Cryptographic SECRET is not set. This may lead to increased predictability in token generation.";
const DB_URL_UNSET: &str = "DATABASE_URL must be set.";
const DB_URL_ERROR: &str = "DATABASE_URL could not be read. Is it valid UTF-8?";

pub fn signal_readiness() {
    info!("{}", SETUP_DONE);
}

pub fn init_tracing_and_dotenv() {
    let mut dotenvy_error = None;
    let mut dotenvy_present = true;

    match dotenvy::dotenv() {
        Ok(_) => (),
        Err(e) => match e.not_found() {
            true => dotenvy_present = false,
            false => dotenvy_error = Some(e),
        },
    }

    let filter = EnvFilter::builder()
        .with_default_directive(DEFAULT_LOG_LEVEL.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();
    info!("Logging initialised");

    if !dotenvy_present {
        warn!("No .env file found; skipping...");
    }
    if let Some(e) = dotenvy_error {
        error!("Error loading/reading .env: {e}");
    }
}

pub async fn init_listener() -> TcpListener {
    match TcpListener::bind("0.0.0.0:2025").await {
        Ok(listener) => {
            match listener.local_addr() {
                Ok(addr) => info!("Bound to: {addr}"),
                Err(e) => error!("Failed to get listener local address: {e}"),
            }
            listener
        }
        Err(e) => {
            error!("Failed to bind to port: {e}");
            panic!();
        }
    }
}

pub fn verify_required_env_vars() {
    match std::env::var("SECRET") {
        Ok(var) => match var.is_empty() {
            true => warn!("{}", SECRET_UNSET),
            false => info!("{}", SECRET_OKAY),
        },
        Err(e) => match e {
            std::env::VarError::NotPresent => warn!("{}", SECRET_UNSET),
            std::env::VarError::NotUnicode(_) => warn!("{}", SECRET_ERROR),
        },
    }

    match std::env::var("DATABASE_URL") {
        Ok(var) => match var.is_empty() {
            false => (),
            true => {
                warn!("{}", DB_URL_UNSET);
                panic!();
            }
        },
        Err(e) => match e {
            std::env::VarError::NotPresent => {
                warn!("{}", DB_URL_UNSET);
                panic!();
            }
            std::env::VarError::NotUnicode(_) => warn!("{}", DB_URL_ERROR),
        },
    }
}

pub mod servertest {
    use std::time::Duration;

    use tokio::{spawn, time::sleep};
    use tracing::{info, warn};

    pub fn test_connectivity() {
        spawn(async {
            let cl = reqwest::Client::new();
            let mut iter = 1;
            loop {
                match cl.get("http://localhost:2025/").send().await {
                    Ok(resp) => match resp.status().is_success() {
                        true => {
                            info!("Health check passed.");
                            break;
                        }
                        false => (),
                    },
                    Err(_) => (),
                };
                match iter {
                    1..=10 => sleep(Duration::from_secs(1)).await,
                    11..=20 => sleep(Duration::from_secs(29)).await,
                    _ => {
                        warn!("Could not connect after 20 tries (5mins). Aborting checks...");
                        break;
                    }
                };
                iter += 1;
            }
        });
    }
}
