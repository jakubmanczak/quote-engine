use crate::oldlogs::LogEvent;
use chrono::Utc;
use sqlite::{Connection, State};
use std::env;
use tracing::{error, info, trace};
use ulid::Ulid;

mod create_default_admin;
mod tables;

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

    match sqlite::open(path) {
        Ok(conn) => conn,
        Err(e) => {
            error!("error establishing sqlite db conn: {e}");
            panic!();
        }
    }
}

pub fn push_log(event: LogEvent) {
    todo!();
    // TODO: update to new log structure
    let string = LogEvent::get_string(event);
    let ulid = Ulid::new().to_string();
    let timestamp = Utc::now().timestamp();
    info!("{}", string);

    let conn = get_conn();
    let q = "INSERT INTO logs VALUES (:id, :timestamp, :content)";
    let mut statement = conn.prepare(q).unwrap();
    statement.bind((":id", ulid.as_str())).unwrap();
    statement.bind((":timestamp", timestamp)).unwrap();
    statement.bind((":content", string.as_str())).unwrap();

    match statement.next() {
        Ok(_) => (),
        Err(e) => error!("Could not push log to database: {e}"),
    }
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
    use tables::*;

    let conn = get_conn();
    for table in [USERS, LOGS, AUTHORS, LINES, QUOTES] {
        conn.execute(table).unwrap();
    }
}
