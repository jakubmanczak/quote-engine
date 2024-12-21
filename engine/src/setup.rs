use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{
    env::{self, VarError},
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};
use tokio::net::TcpListener;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

pub async fn init_database_pool() -> Pool<Sqlite> {
    let url = env::var("DATABASE_URL").unwrap();
    match sqlx::Sqlite::database_exists(&url).await {
        Ok(exists) => {
            if !exists {
                match sqlx::Sqlite::create_database(&url).await {
                    Ok(_) => info!("db missing; created a blank database file"),
                    Err(e) => {
                        error!("could not create blank db: {e}");
                        panic!();
                    }
                }
            }
        }
        Err(e) => {
            error!("could not check for db existence: {e}");
            panic!();
        }
    };

    match SqlitePoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_millis(100))
        .connect(&url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            error!("could not connect to database: {e}");
            panic!();
        }
    }
}

pub fn initialise_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed!");
    info!("quote-engine says hello -> tracing initialised!");
}

pub fn initialise_dotenv() {
    match dotenvy::dotenv() {
        Ok(_) => info!("loaded .env"),
        Err(e) => {
            if e.not_found() {
                info!(".env file not found; skipping...");
            } else {
                error!("error while loading .env: {e}");
            }
        }
    };
}

pub fn verify_secret_presence() {
    match std::env::var("SECRET") {
        Ok(_) => (),
        Err(e) => {
            error!("error encountered while checking for SECRET: {e}");
            panic!();
        }
    }
}

pub fn verify_dburl_presence() {
    match std::env::var("DATABASE_URL") {
        Ok(_) => (),
        Err(e) => {
            error!("error encountered while checking for DATABASE_URL: {e}");
            panic!();
        }
    }
}

pub async fn initialise_listener() -> TcpListener {
    let addr = get_socket_addr();
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("error creating a listener: {e}");
            panic!();
        }
    };
    report_listener_socket_addr(&listener);
    listener
}

fn get_port() -> u16 {
    const DEFAULT_PORT: u16 = 2024;
    match std::env::var("PORT") {
        Ok(portstr) => match portstr.parse() {
            Ok(num) => num,
            Err(e) => {
                error!("error parsing env var PORT from str to u16: {e}");
                panic!();
            }
        },
        Err(e) => {
            match e {
                VarError::NotPresent => info!("PORT not found in env; using default"),
                _ => info!("PORT environment variable error: {e}"),
            }
            return DEFAULT_PORT;
        }
    }
}

pub fn get_socket_addr() -> SocketAddrV4 {
    SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), get_port())
}

pub fn report_listener_socket_addr(listener: &TcpListener) {
    let addr = match listener.local_addr() {
        Ok(addr) => addr,
        Err(e) => {
            error!("error while getting listener socket address: {e}");
            panic!();
        }
    };
    info!("now listening on {}", addr.to_string());
}
