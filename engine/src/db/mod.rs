use sqlite::{Connection, State};
use std::env;
use tables::TABLES;
use tracing::{error, info, trace};

mod create_default_admin;
mod tables;

pub mod queries;
pub mod users;

pub fn get_conn() -> Connection {
    let path = match env::var("DBPATH") {
        Ok(env) => env,
        Err(e) => {
            match e {
                env::VarError::NotPresent => trace!("DBPATH environment variable not found"),
                _ => error!("DBPATH environment variable error: {e}"),
            }
            "quotes.db".to_owned()
        }
    };

    let mut conn = match sqlite::open(path) {
        Ok(conn) => conn,
        Err(e) => {
            error!("error establishing sqlite db conn: {e}");
            panic!();
        }
    };

    match conn.set_busy_timeout(100) {
        Ok(_) => (),
        Err(err) => trace!(
            "Could not set db connection timeout! Requests may err due to db lock ({})",
            err.to_string()
        ),
    };

    conn
}

pub fn check_for_lack_of_account() {
    let conn = get_conn();
    let q = "SELECT * FROM users";
    let mut statement = conn.prepare(q).unwrap();

    match statement.next() {
        Ok(State::Row) => (),
        Ok(State::Done) => create_default_admin::run(),
        Err(e) => {
            error!("Could not check for users while preparing db - could the database be corrupt? ({e})");
            panic!();
        }
    }
}

pub fn execute_migration_queries() {
    let conn = get_conn();
    conn.execute(TABLES).unwrap();
    match conn.execute("PRAGMA journal_mode = WAL;") {
        Ok(_) => info!("WAL mode set"),
        Err(_) => info!("Could not set WAL mode - requests may err!"),
    }
}
