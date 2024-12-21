use crate::{authors::AuthorPatch, users::patch::UserPatch};

pub trait AllNoneChecker {
    fn all_none(&self) -> bool;
}

impl AllNoneChecker for UserPatch {
    fn all_none(&self) -> bool {
        self.name.is_none() && self.clearance.is_none()
    }
}

impl AllNoneChecker for AuthorPatch {
    fn all_none(&self) -> bool {
        self.name.is_none() && self.obfname.is_none()
    }
}
