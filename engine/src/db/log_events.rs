use crate::models::User;

pub enum LogEvents {
    UserCreatedBySystem(User),
    UserCreated(LogUserInfo),
    UserDeleted(LogUserInfo),
    UserModified(LogUserInfo),
}
pub struct LogUserInfo {
    actor: User,
    subject: User,
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
                    "User {:?} was created by {}({})",
                    info.subject, info.actor.name, info.actor.id
                )
            }
            UserDeleted(info) => {
                format!(
                    "User {:?} was deleted by {}({})",
                    info.subject, info.actor.name, info.actor.id
                )
            }
            UserModified(info) => {
                format!(
                    "User {:?} was modified by {}({})",
                    info.subject, info.actor.name, info.actor.id,
                )
            }
        }
    }
}
