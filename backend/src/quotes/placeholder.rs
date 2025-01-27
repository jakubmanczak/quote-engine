use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use uuid::Uuid;

use super::{authors::Author, Quote, QuoteLine};

pub fn return_placeholder_random_public_quote() -> Quote {
    let mut authors = HashMap::new();
    authors.insert(
        Uuid::nil(),
        Author {
            id: Uuid::nil(),
            fullname: String::from("Pixieline"),
            codename: String::from("The QuoteEngine Pixie"),
        },
    );
    Quote {
        id: Uuid::nil(),
        clearance: 0,
        context: Some(String::from("About the lack of a public quote.")),
        timestamp: NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 1, 27).unwrap(),
            NaiveTime::from_hms_opt(0, 24, 0).unwrap(),
        ),
        authors,
        lines: vec![
            QuoteLine {
                id: Uuid::nil(),
                content: String::from("Hey, you have no public quotes in your database yet!"),
                author_id: Uuid::nil(),
            },
            QuoteLine {
                id: Uuid::nil(),
                content: String::from("You should add some."),
                author_id: Uuid::nil(),
            },
        ],
    }
}
