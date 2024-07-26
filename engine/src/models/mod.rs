use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Quote {
    pub id: String,
    pub context: String,
    pub timestamp: i32,
    // assembled from db
    pub lines: Option<Vec<Line>>,
    pub authors: Option<Vec<Author>>,
}

#[derive(Debug)]
pub struct Line {
    pub id: String,
    pub content: String,
    pub position: u32,
    pub quote: String,
    pub author: String,
}

#[derive(Debug)]
pub struct Author {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Log {
    pub id: String,
    pub content: String,
    pub timestamp: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
}
