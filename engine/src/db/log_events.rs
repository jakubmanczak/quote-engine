use crate::{models::User, permissions::UserPermission};

pub enum LogEvents {
    UserCreatedBySystem(User),
    UserCreated(LogUserInfo),
    UserDeleted(LogUserInfo),
    UserMutated(LogUserMutatedInfo),
}
pub struct LogUserInfo {
    pub actor: User,
    pub subject: User,
}

pub struct LogUserMutatedInfo {
    pub actor: User,
    pub subject: User,
    pub patched: UserMutation,
}

#[derive(Debug)]
pub struct UserMutation {
    pub name: Option<String>,
    pub color: Option<String>,
    pub picture: Option<String>,
    pub perms: Option<Vec<UserPermission>>,
}

impl LogEvents {
    pub fn get_string(event: LogEvents) -> String {
        use LogEvents::*;
        match event {
            UserCreatedBySystem(subject) => {
                format!("{:?} was created by system", subject)
            }
            UserCreated(info) => {
                format!(
                    "{:?} was created by {}({})",
                    info.subject, info.actor.name, info.actor.id
                )
            }
            UserDeleted(info) => {
                format!(
                    "{:?} was deleted by {}({})",
                    info.subject, info.actor.name, info.actor.id
                )
            }
            UserMutated(info) => {
                format!(
                    "{}({}) was modified {:?} by {}({})",
                    info.subject.name,
                    info.subject.id,
                    info.patched,
                    info.actor.name,
                    info.actor.id
                )
            }
        }
    }
}
