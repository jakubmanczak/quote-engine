use sqlite::{Connection, State};
use std::env;
use tables::TABLES;
use tracing::{error, trace};

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
}
