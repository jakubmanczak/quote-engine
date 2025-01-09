use attributes::{default_attributes_u64, UserAttribute};
use serde::Serialize;
use uuid::Uuid;

pub mod attributes;
pub mod auth;
pub mod infradmin;
pub mod patch;
pub mod queries;
pub mod validity;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: Uuid,
    pub handle: String,
    pub clearance: u8,
    attributes: u64,
}

impl User {
    pub fn has_attribute(&self, attr: UserAttribute) -> bool {
        self.attributes & attr.get_bit() != 0
    }
    pub fn has_permission(&self, attr: UserAttribute) -> bool {
        (self.attributes & attr.get_bit() != 0)
            || (self.attributes & UserAttribute::TheEverythingPermission.get_bit() != 0)
    }
    /// This only creates a user local to the scope, it does not save it to the database.
    /// Call `User::create` to add a user to the database.
    pub fn new_incomplete(handle: String) -> User {
        User {
            id: Uuid::now_v7(),
            handle,
            clearance: 1,
            attributes: default_attributes_u64(),
        }
    }
}
