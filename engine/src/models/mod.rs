use crate::permissions::UserPermission;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use ulid::Ulid;

pub const DEFAULT_COLOR: &str = "28166f";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Ulid,
    pub name: String,
    pub color: String,
    pub picture: String,
    pub perms: Vec<UserPermission>,
}

#[derive(Debug)]
pub struct Quote {
    pub id: Ulid,
    pub timestamp: i64,
    pub context: Option<String>,
    // assembled from db
    pub lines: Vec<Line>,
    pub authors: Vec<Author>,
}

#[derive(Debug)]
pub struct Line {
    pub id: Ulid,
    pub content: String,
    pub position: u8,
    pub quote: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Author {
    pub id: Ulid,
    pub name: String,
    pub obfname: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub limit: NonZeroU32,
    pub page: NonZeroU32,
}
