use crate::{oldlogs::LogEvent, permissions::UserPermission};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

pub const DEFAULT_COLOR: &str = "28166f";
pub const DEFAULT_PICTURE: &str = "https://placewaifu.com/image/64";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub color: String,
    pub picture: String,
    pub perms: Vec<UserPermission>,
}

#[derive(Debug)]
pub struct Quote {
    pub id: String,
    pub context: String,
    pub timestamp: u64,
    // assembled from db
    pub lines: Option<Vec<Line>>,
    pub authors: Option<Vec<Author>>,
}

#[derive(Debug)]
pub struct Line {
    pub id: String,
    pub content: String,
    pub position: u8,
    pub quote: String,
    pub author: String,
}

#[derive(Debug)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub obfname: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub limit: NonZeroU32,
    pub page: NonZeroU32,
}
