use std::env;

use sqlite::Connection;
use tracing::{error, trace};

mod tables;

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

pub fn execute_migration_queries() {
    use tables::*;

    let conn = get_conn();
    for table in [USERS, LOGS, AUTHORS, LINES, QUOTES] {
        conn.execute(table).unwrap();
    }
}
