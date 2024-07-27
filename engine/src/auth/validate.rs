use super::AuthBasic;
use crate::db::get_conn;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlite::State;
use tracing::error;

pub fn validate_basic_auth(auth: &AuthBasic) -> bool {
    let conn = get_conn();
    let q = "SELECT pass FROM users WHERE name = :name";
    let mut statement = conn.prepare(q).unwrap();
    statement.bind((":name", auth.user.as_str())).unwrap();

    match statement.next() {
        Ok(State::Row) => {
            let hashstring: String = statement.read("pass").unwrap();
            let argon = Argon2::default();
            let hash = match PasswordHash::new(hashstring.as_str()) {
                Ok(h) => h,
                Err(e) => {
                    error!("Could not parse hash string into hash: {e}");
                    return false;
                }
            };

            argon.verify_password(auth.pass.as_bytes(), &hash).is_ok()
        }
        Ok(State::Done) => return false,
        Err(e) => {
            error!("Could not fetch {}'s password: {e}", auth.user);
            return false;
        }
    }
}
