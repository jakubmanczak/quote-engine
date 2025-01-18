use authors::Author;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use uuid::Uuid;

pub mod authors;

pub struct Quote {
    pub id: Uuid,
    pub lines: Vec<QuoteLine>,
    pub authors: HashMap<Uuid, Author>,

    pub context: Option<String>,
    pub timestamp: NaiveDateTime,
    pub clearance: u8,
}

pub struct QuoteLine {
    pub id: Uuid,
    pub content: String,
    pub author_id: Uuid,
    pub position: u8,
}
