use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserPatch {
    pub name: Option<String>,
    pub clearance: Option<u8>,
    // pub attributes: Option<u64>,
}

#[derive(thiserror::Error, Debug)]
pub enum UserPatchError {
    #[error("At least one field must be present.")]
    NoFields,
    #[error("Target user doesn't exist.")]
    IncorrectTarget,
    #[error("You can't patch this user - they have higher clearance.")]
    InsufficientClearance,
    #[error("Can't change root user's clearance.")]
    NoChangeRootClearance,
    // TODO: tajemniczy mysi skręt który przyda nam się później!
    // #[error("Can't change root user's TheEverythingPermission attribute.")]
    // NoChangeRootEverythingPermission,
}

impl UserPatch {
    pub fn is_valid(&self) -> bool {
        self.name.is_some() || self.clearance.is_some()
    }
}
