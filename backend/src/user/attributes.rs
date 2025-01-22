use serde::Serialize;
use strum::VariantArray;
use UserAttribute as A;

#[derive(VariantArray, Serialize)]
pub enum UserAttribute {
    TheEverythingPermission,
    UsersInspectPermission,
    UsersChangeOwnHandlePermission,
    UsersChangeOwnPasswordPermission,
    UsersManageHandlesPermission,
    UsersManagePasswordsPermission,
    UsersManageClearancesPermission,
    UsersManageAttributesPermission,
    UsersManualCreatePermission,
    UsersDeletePermission,
    LogsInspectPermission,
    AuthorsInspectPermission,
    AuthorsCreatePermission,
    AuthorsModifyPermission,
    AuthorsDeletePermission,
    QuotesCreatePermission,

    DisplayCoquetteAvatar,
    DisplayProfileCardFlower,
}

const DEFAULT_ATTRIBUTES: [UserAttribute; 7] = [
    A::LogsInspectPermission,
    A::AuthorsInspectPermission,
    A::AuthorsCreatePermission,
    A::AuthorsModifyPermission,
    A::UsersInspectPermission,
    A::UsersChangeOwnHandlePermission,
    A::UsersChangeOwnPasswordPermission,
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
        match self {
            A::TheEverythingPermission => 0,
            A::UsersInspectPermission => 1,
            A::UsersChangeOwnHandlePermission => 2,
            A::UsersChangeOwnPasswordPermission => 3,
            A::UsersManageHandlesPermission => 4,
            A::UsersManagePasswordsPermission => 5,
            A::UsersManageClearancesPermission => 6,
            A::UsersManageAttributesPermission => 7,
            A::UsersManualCreatePermission => 8,
            A::UsersDeletePermission => 9,
            // 0b1 << 10-15
            A::LogsInspectPermission => 16,
            // 0b1 << 17-19
            A::AuthorsInspectPermission => 20,
            A::AuthorsCreatePermission => 21,
            A::AuthorsModifyPermission => 22,
            // 0b1 << 23-24
            A::AuthorsDeletePermission => 25,
            // 0b1 << 26-31
            A::QuotesCreatePermission => 32,
            // 0b1 << 33-60
            A::DisplayCoquetteAvatar => 61,
            A::DisplayProfileCardFlower => 62,
            // 0b1 << 63
        }
    }
    pub fn get_bit(&self) -> u64 {
        0b1 << self.get_bit_offset()
    }
}
