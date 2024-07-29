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
}

pub const USER_PERMISSIONS: [UserPermission; 6] = [
    Everything,
    MutateOwnUser,
    CreateUsers,
    DeleteUsers,
    MutateUsers,
    MutateUsersPermissions,
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
    // pub fn check_permission_via_bits(perm: &UserPermission, bits: &u32) -> bool {
    //     for p in [&Everything, perm] {
    //         if bits & UserPermission::get_bit_from_permission(p) > 0 {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
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
            Everything => 0b1,
            MutateOwnUser => 0b10,
            CreateUsers => 0b100,
            DeleteUsers => 0b1000,
            MutateUsers => 0b10000,
            MutateUsersPermissions => 0b100000,
        }
    }
}
