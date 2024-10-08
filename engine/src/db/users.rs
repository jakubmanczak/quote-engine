use super::get_conn;
use crate::models::User;
use crate::{error::Error, permissions::UserPermission};
use sqlite::{State, Statement};
use tracing::error;
use ulid::Ulid;

#[derive(Debug, Clone)]
pub enum GetUserDataInput {
    Id(String),
    Name(String),
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
                id: Ulid::from_string(st.read::<String, _>("id").unwrap().as_str())?,
                name: st.read("name").unwrap(),
                color: st.read("color").unwrap(),
                picture: st.read("picture").unwrap(),
                perms: UserPermission::get_permissions_from_bits(
                    match u32::try_from(st.read::<i64, _>("permissions").unwrap()) {
                        Ok(u) => u,
                        Err(e) => {
                            let res = format!("Could not get u32 from i64: {e}");
                            return Err(Error::GetUserDataError(res));
                        }
                    },
                ),
            });
        }
        Ok(State::Done) => {
            let res = format!("No user of {:?} found.", data);
            return Err(Error::NoRowsError(res));
        }
        Err(e) => {
            error!("sqlite err in getuserdata: {e}");
            return Err(Error::SqliteError(e));
        }
    }
}
