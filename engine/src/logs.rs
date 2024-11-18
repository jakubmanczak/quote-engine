use crate::{
    db::get_conn,
    models::{Author, User},
    permissions::UserPermission,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use ulid::Ulid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogEntry {
    pub id: Ulid,
    pub timestamp: i64,
    pub actor: Ulid,
    pub subject: Ulid,
    pub action: LogEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LogEvent {
    UserCreatedBySystem(User),
    UserCreated(User),
    UserDeleted(User),
    UserPasswordUpdated,
    UserNameUpdated {
        old_name: String,
        new_name: String,
    },
    UserColorUpdated {
        old_color: String,
        new_color: String,
    },
    UserPictureUpdated {
        old_picture: String,
        new_picture: String,
    },
    UserPermissionsUpdated {
        old_perms: Vec<UserPermission>,
        new_perms: Vec<UserPermission>,
    },
    AuthorCreated(Author),
    AuthorDeleted(Author),
    AuthorUpdated {
        old: Author,
        new: Author,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum LogError {
    #[error("Could not serve variant as string")]
    LogVariantSerializeError,
}

impl LogEvent {
    pub fn as_json(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string(self)?)
    }
    pub fn variant_as_str(&self) -> Result<String, anyhow::Error> {
        let json = serde_json::to_string(self)?;
        match json.contains("{") {
            true => {
                let split = json.splitn(3, "\"").collect::<Vec<&str>>()[1].to_string();
                Ok(split)
            }
            false => {
                let one = match json.strip_prefix("\"") {
                    Some(str) => str,
                    None => {
                        error!("Could not serve variant as string");
                        return Err(LogError::LogVariantSerializeError)?;
                    }
                };
                let two = match one.strip_suffix("\"") {
                    Some(str) => str,
                    None => {
                        error!("Could not serve variant as string");
                        return Err(LogError::LogVariantSerializeError)?;
                    }
                };

                Ok(two.to_string())
            }
        }
    }
}

pub fn push_log(event: LogEntry) {
    let action = match event.action.variant_as_str() {
        Ok(str) => str,
        Err(e) => {
            error!("Could not push log! Could not push log!");
            error!("{:?}", event);
            error!("{e}");
            return;
        }
    };
    let details = match event.action.as_json() {
        Ok(str) => str,
        Err(e) => {
            error!("Could not push log! Could not push log!");
            error!("{:?}", event);
            error!("{e}");
            return;
        }
    };

    info!("{:#?}", event);

    let conn = get_conn();
    let q = "INSERT INTO logs VALUES (:id, :timestamp, :actor, :subject, :action, :details)";
    let mut st = conn.prepare(q).unwrap();
    st.bind((":id", event.id.to_string().as_str())).unwrap();
    st.bind((":timestamp", event.timestamp)).unwrap();
    st.bind((":actor", event.actor.to_string().as_str()))
        .unwrap();
    st.bind((":subject", event.subject.to_string().as_str()))
        .unwrap();
    st.bind((":action", action.as_str())).unwrap();
    st.bind((":details", details.as_str())).unwrap();

    match st.next() {
        Ok(_) => (),
        Err(e) => error!("Could not push log to database: {e}"),
    }
}
