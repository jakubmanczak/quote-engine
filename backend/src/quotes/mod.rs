use authors::Author;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::omnierror::OmniError;

pub mod authors;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quote {
    #[serde(skip_deserializing)]
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,
    pub lines: Vec<QuoteLine>,
    #[serde(skip_deserializing)]
    pub authors: HashMap<Uuid, Author>,

    pub context: Option<String>,
    pub timestamp: NaiveDateTime,
    pub clearance: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuoteLine {
    #[serde(skip_deserializing)]
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,
    pub content: String,
    pub author_id: Uuid,
}

impl Quote {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Quote>, OmniError> {
        // TODO: fetching everything at once is bad at scale. do pagination.
        // TODO: is the quote metadata repeated in every row, or is it cleverly
        // allocated in such a way that it doesn't pose a problem?
        match sqlx::query!(
            r#"
                SELECT
                    quotes.id AS quote_id, quotes.timestamp AS timestamp,
                    quotes.context AS context, quotes.clearance AS clearance,
                    lines.id AS line_id, lines.content AS line_content,
                    authors.id AS author_id, authors.fullname AS author_fullname,
                    authors.codename AS author_codename
                FROM quotes
                LEFT JOIN lines ON quotes.id = lines.quote_id
                LEFT JOIN authors ON lines.author_id = authors.id
                ORDER BY quotes.id DESC, lines.position ASC
            "#
        )
        .fetch_all(pool)
        .await
        {
            Ok(recs) => {
                let mut qvec: Vec<Quote> = vec![];
                let mut q: Quote;
                if let Some(rec) = recs.first() {
                    q = Quote {
                        id: rec.quote_id,
                        clearance: rec.clearance as u8,
                        timestamp: rec.timestamp,
                        context: rec.context.clone(),
                        authors: HashMap::new(),
                        lines: Vec::new(),
                    };
                } else {
                    return Ok(vec![]);
                }
                for rec in recs {
                    match q.id == rec.quote_id {
                        true => {
                            q.lines.push(QuoteLine {
                                id: rec.line_id,
                                content: rec.line_content,
                                author_id: rec.author_id,
                            });
                            if !q.authors.contains_key(&rec.author_id) {
                                q.authors.insert(
                                    rec.author_id,
                                    Author {
                                        id: rec.author_id,
                                        fullname: rec.author_fullname,
                                        codename: rec.author_codename,
                                    },
                                );
                            }
                        }
                        false => {
                            // q is done, push it and start a new one
                            qvec.push(q);
                            q = Quote {
                                id: rec.quote_id,
                                clearance: rec.clearance as u8,
                                timestamp: rec.timestamp,
                                context: rec.context.clone(),
                                authors: HashMap::new(),
                                lines: Vec::new(),
                            };
                            q.lines.push(QuoteLine {
                                id: rec.line_id,
                                content: rec.line_content,
                                author_id: rec.author_id,
                            });
                            if !q.authors.contains_key(&rec.author_id) {
                                q.authors.insert(
                                    rec.author_id,
                                    Author {
                                        id: rec.author_id,
                                        fullname: rec.author_fullname,
                                        codename: rec.author_codename,
                                    },
                                );
                            }
                        }
                    }
                }

                Ok(qvec)
            }
            Err(e) => Err(e)?,
        }
    }
    pub async fn get_by_id(id: &Uuid, pool: &PgPool) -> Result<Option<Quote>, OmniError> {
        match sqlx::query!(
            r#"
                SELECT
                    quotes.id AS quote_id, quotes.timestamp AS timestamp,
                    quotes.context AS context, quotes.clearance AS clearance,
                    lines.id AS line_id, lines.content AS line_content,
                    authors.id AS author_id, authors.fullname AS author_fullname,
                    authors.codename AS author_codename
                FROM quotes
                LEFT JOIN lines ON quotes.id = lines.quote_id
                LEFT JOIN authors ON lines.author_id = authors.id
                WHERE quotes.id = $1
                ORDER BY quotes.id DESC, lines.position ASC
            "#,
            id
        )
        .fetch_all(pool)
        .await
        {
            Ok(recs) => {
                let mut q: Quote;
                if let Some(rec) = recs.first() {
                    q = Quote {
                        id: rec.quote_id,
                        clearance: rec.clearance as u8,
                        timestamp: rec.timestamp,
                        context: rec.context.clone(),
                        authors: HashMap::new(),
                        lines: Vec::new(),
                    };
                } else {
                    return Ok(None);
                }
                for rec in recs {
                    q.lines.push(QuoteLine {
                        id: rec.line_id,
                        content: rec.line_content,
                        author_id: rec.author_id,
                    });
                    if !q.authors.contains_key(&rec.author_id) {
                        q.authors.insert(
                            rec.author_id,
                            Author {
                                id: rec.author_id,
                                fullname: rec.author_fullname,
                                codename: rec.author_codename,
                            },
                        );
                    }
                }

                Ok(Some(q))
            }
            Err(e) => Err(e)?,
        }
    }
    pub async fn create(quote: Quote, pool: &PgPool) -> Result<Quote, OmniError> {
        let mut tr = pool.begin().await?;

        match sqlx::query!(
            "INSERT INTO quotes(id, context, clearance, timestamp) VALUES ($1, $2, $3, $4)",
            quote.id,
            quote.context,
            quote.clearance as i16,
            quote.timestamp
        )
        .execute(&mut *tr)
        .await
        {
            Ok(_) => (),
            Err(e) => {
                tr.rollback().await?;
                return Err(e)?;
            }
        }

        for (index, line) in quote.lines.iter().enumerate() {
            match sqlx::query!(
                "INSERT INTO lines(id, quote_id, author_id, content, position) VALUES ($1, $2, $3, $4, $5)",
                line.id,
                quote.id,
                line.author_id,
                line.content,
                index as i32
            )
            .execute(&mut *tr)
            .await
            {
                Ok(_) => (),
                Err(e) => {
                    tr.rollback().await?;
                    return Err(e)?;
                }
            }
        }

        tr.commit().await?;
        Ok(quote)
    }
}
