use serde::Serialize;
use sqlx::{Pool, Sqlite};
use ulid::Ulid;

use crate::error::OmniError;

#[derive(Serialize)]
pub struct ExtendedAuthor {
    pub id: Ulid,
    pub name: String,
    pub obfname: String,
    pub quotecount: i64,
    pub linecount: i64,
}

impl ExtendedAuthor {
    pub async fn get_by_id(
        id: Ulid,
        pool: &Pool<Sqlite>,
    ) -> Result<Option<ExtendedAuthor>, OmniError> {
        let idstr = id.to_string();
        // match sqlx::query!(
        //     "SELECT authors.id, authors.name, authors.obfname,
        //         COUNT(DISTINCT lines.quote) AS quotecount,
        //         COUNT(lines.id) AS linescount
        //     FROM authors LEFT JOIN lines ON authors.id = lines.author
        //     WHERE authors.id = ? GROUP BY authors.id, authors.name",
        //     idstr
        // )
        // ^^ The above doesn't work :(
        // For some reason, SQLx requires CASTing a String to a String in this case
        match sqlx::query!(
            "SELECT authors.id, authors.name, authors.obfname,
                COUNT(DISTINCT lines.quote) AS quotecount,
                COUNT(lines.id) AS linescount
            FROM authors LEFT JOIN lines ON authors.id = lines.author
            WHERE authors.id = CAST(? AS TEXT) GROUP BY authors.id, authors.name",
            idstr
        )
        .fetch_optional(pool)
        .await?
        {
            Some(rec) => Ok(Some(ExtendedAuthor {
                id: Ulid::from_string(&rec.id)?,
                name: rec.name,
                obfname: rec.obfname,
                quotecount: rec.quotecount,
                linecount: rec.linescount,
            })),
            None => return Ok(None),
        }
    }
}
