use serde::{Deserialize, Serialize};

use crate::{models::User, permissions::UserPermission};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: u64,
    pub actor: String,
    pub subject: String,
    pub action: LogEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LogEvent {
    UserCreatedBySystem(User),
    UserCreated(User),
    UserDeleted(User),
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
}
