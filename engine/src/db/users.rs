use super::get_conn;
use crate::permissions::UserPermission;
use crate::{error::Error, models::User};
use sqlite::{State, Statement};
use strum::Display;
use tracing::error;
use ulid::Ulid;

#[derive(Debug, Clone, Display)]
pub enum GetUserDataInput {
    Id(String),
    Name(String),
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserDataError {
    #[error("Could not get u32 from i64 (reading permission bits)")]
    I64ToU32ConversionFault,
    #[error("No user \"{0}\" found")]
    NoSuchUserFound(String),
    #[error("Sqlite error")]
    SqliteError,
    #[error("Ulid from string conversion fault: {0}")]
    UlidReadFault(String),
}

pub fn get_user_data(data: GetUserDataInput) -> Result<User, Error> {
    let conn = get_conn();
    let mut st: Statement;
    match data.clone() {
        GetUserDataInput::Id(s) => {
            let q = "SELECT * FROM users WHERE id = :id";
            st = conn.prepare(q).unwrap();
            st.bind((":id", s.as_str())).unwrap();
        }
        GetUserDataInput::Name(s) => {
            let q = "SELECT * FROM users WHERE name = :name";
            st = conn.prepare(q).unwrap();
            st.bind((":name", s.as_str())).unwrap();
        }
    }

    match st.next() {
        Ok(State::Row) => {
            return Ok(User {
                id: match Ulid::from_string(st.read::<String, _>("id").unwrap().as_str()) {
                    Ok(id) => id,
                    Err(e) => return Err(GetUserDataError::UlidReadFault(e.to_string()))?,
                },
                name: st.read("name").unwrap(),
                color: st.read("color").unwrap(),
                picture: st.read("picture").unwrap(),
                perms: UserPermission::get_permissions_from_bits(
                    match u32::try_from(st.read::<i64, _>("permissions").unwrap()) {
                        Ok(u) => u,
                        Err(e) => return Err(GetUserDataError::I64ToU32ConversionFault)?,
                    },
                ),
            });
        }
        Ok(State::Done) => return Err(GetUserDataError::NoSuchUserFound(data.to_string()))?,
        Err(e) => {
            return Err(GetUserDataError::SqliteError)?;
        }
    }
}
