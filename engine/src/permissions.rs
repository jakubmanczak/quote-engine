use serde::{Deserialize, Serialize};
use UserPermission::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserPermission {
    Everything,
    MutateOwnUser,
    CreateUsers,
    DeleteUsers,
    MutateUsers,
    MutateUsersPermissions,
    MutateUsersPasswords,
    InspectLogs,

    // OTHER ENTITLEMENTS
    DisplayFlower,
}

pub const USER_PERMISSIONS: [UserPermission; 9] = [
    Everything,
    MutateOwnUser,
    CreateUsers,
    DeleteUsers,
    MutateUsers,
    MutateUsersPermissions,
    MutateUsersPasswords,
    InspectLogs,
    DisplayFlower,
];

pub const DEFAULT_PERMISSIONS: [UserPermission; 1] = [MutateOwnUser];

impl UserPermission {
    pub fn check_permission(checked_perm: &UserPermission, perms: &Vec<UserPermission>) -> bool {
        if perms.contains(&Everything) || perms.contains(checked_perm) {
            return true;
        } else {
            return false;
        }
    }
    pub fn get_permissions_from_bits(bits: u32) -> Vec<UserPermission> {
        let mut vec = Vec::new();
        for perm in USER_PERMISSIONS {
            if (bits & UserPermission::get_bit_from_permission(&perm)) > 0 {
                vec.push(perm)
            }
        }
        return vec;
    }
    pub fn get_bits_from_permissions(perms: &Vec<UserPermission>) -> u32 {
        let mut bits: u32 = 0;
        for perm in perms {
            bits |= UserPermission::get_bit_from_permission(&perm);
        }
        return bits;
    }
    pub fn get_bit_from_permission(perm: &UserPermission) -> u32 {
        match perm {
            Everything => 0b1 << 0,
            MutateOwnUser => 0b1 << 1,
            CreateUsers => 0b1 << 2,
            DeleteUsers => 0b1 << 3,
            MutateUsers => 0b1 << 4,
            MutateUsersPermissions => 0b1 << 5,
            MutateUsersPasswords => 0b1 << 6,
            InspectLogs => 0b1 << 7,
            DisplayFlower => 0b1 << 8,
        }
    }
}
