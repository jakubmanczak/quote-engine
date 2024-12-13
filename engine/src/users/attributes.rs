use serde::Serialize;
use strum::VariantArray;
use UserAttribute::*;

#[derive(Serialize, VariantArray, Clone, Debug)]
pub enum UserAttribute {
    TheEverythingPermission,
    UserCreateManuallyPermission,
    UserCreateInvitePermission,
    UserDeletePermission,
    LogsInspectPermission,
    AuthorCreatePermission,
    AuthorModifyPermission,
    AuthorDeletePermission,
}

const DEFAULT_ATTRIBUTES: [UserAttribute; 3] = [
    LogsInspectPermission,
    AuthorCreatePermission,
    AuthorModifyPermission,
];

pub fn default_attributes_u64() -> u64 {
    let mut u64 = 0;
    for attr in DEFAULT_ATTRIBUTES {
        u64 |= attr.get_bit();
    }
    u64
}

impl UserAttribute {
    pub fn get_bit(&self) -> u64 {
        use UserAttribute::*;
        match self {
            TheEverythingPermission => 0b1 << 0,
            UserCreateManuallyPermission => 0b1 << 1,
            UserCreateInvitePermission => 0b1 << 2,
            // 0b1 << 3-7
            UserDeletePermission => 0b1 << 8,
            // 0b1 << 9
            LogsInspectPermission => 0b1 << 10,
            // 0b1 << 11-19
            AuthorCreatePermission => 0b1 << 21,
            AuthorModifyPermission => 0b1 << 22,
            // 0b1 << 23-24
            AuthorDeletePermission => 0b1 << 25,
        }
    }
}
