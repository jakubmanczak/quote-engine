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
    AuthorInspectPermission,
    AuthorCreatePermission,
    AuthorModifyPermission,
    AuthorDeletePermission,
}

const DEFAULT_ATTRIBUTES: [UserAttribute; 4] = [
    LogsInspectPermission,
    AuthorInspectPermission,
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
    pub fn get_bit_offset(&self) -> u8 {
        use UserAttribute::*;
        match self {
            TheEverythingPermission => 0,
            UserCreateManuallyPermission => 1,
            UserCreateInvitePermission => 2,
            // 0b1 << 3-7
            UserDeletePermission => 8,
            // 0b1 << 9
            LogsInspectPermission => 10,
            // 0b1 << 11-19
            AuthorInspectPermission => 20,
            AuthorCreatePermission => 21,
            AuthorModifyPermission => 22,
            // 0b1 << 23-24
            AuthorDeletePermission => 25,
            // 0b1 << 26-63
        }
    }
    pub fn get_bit(&self) -> u64 {
        0b1 << self.get_bit_offset()
    }
}
