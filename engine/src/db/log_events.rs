use crate::models::User;

pub enum LogEvents {
    UserCreatedBySystem(User),
    UserCreated(LogUserInfo),
    UserDeleted(LogUserInfo),
    UserModified(LogUserInfo),
}
pub struct LogUserInfo {
    pub actor: User,
    pub subject: User,
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
            UserModified(info) => {
                format!(
                    "{:?} was modified by {}({})",
                    info.subject, info.actor.name, info.actor.id,
                )
            }
        }
    }
}
