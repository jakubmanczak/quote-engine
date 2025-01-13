use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::omnierror::OmniError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Author {
    #[serde(skip_deserializing)]
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,
    pub fullname: String,
    pub codename: String,
}

#[derive(Serialize)]
pub struct ExtendedAuthor {
    pub author: Author,
    pub quote_count: u32,
    pub line_count: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorPatch {
    pub fullname: Option<String>,
    pub codename: Option<String>,
}

impl Author {
    pub async fn get_by_id(id: &Uuid, pool: &PgPool) -> Result<Option<Author>, OmniError> {
        match sqlx::query_as!(
            Author,
            "SELECT id, fullname, codename FROM authors WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await
        {
            Ok(opt) => Ok(opt),
            Err(e) => Err(OmniError::from(e)),
        }
    }
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Author>, OmniError> {
        match sqlx::query_as!(Author, "SELECT id, fullname, codename FROM authors")
            .fetch_all(pool)
            .await
        {
            Ok(authors) => Ok(authors),
            Err(e) => Err(OmniError::from(e)),
        }
    }
    pub async fn create(author: Author, pool: &PgPool) -> Result<Author, OmniError> {
        match sqlx::query!(
            "INSERT INTO authors (id, fullname, codename) VALUES ($1, $2, $3)",
            &author.id,
            &author.fullname,
            &author.codename
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok(author),
            Err(e) => {
                error!("err: {e}");
                Err(OmniError::from(e))
            }
        }
    }
    pub async fn patch(self, patch: AuthorPatch, pool: &PgPool) -> Result<Author, OmniError> {
        let author = Author {
            id: self.id,
            fullname: patch.fullname.unwrap_or(self.fullname),
            codename: patch.codename.unwrap_or(self.codename),
        };
        match sqlx::query!(
            "UPDATE authors SET fullname = $1, codename = $2 WHERE id = $3",
            &author.fullname,
            &author.codename,
            self.id
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok(author),
            Err(e) => {
                error!("err: {e}");
                Err(OmniError::from(e))
            }
        }
    }
    pub async fn destroy(self, pool: &PgPool) -> Result<(), OmniError> {
        match sqlx::query!("DELETE FROM authors WHERE id = $1", self.id)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(OmniError::from(e)),
        }
    }
}

impl ExtendedAuthor {
    pub async fn get_by_id(id: &Uuid, pool: &PgPool) -> Result<Option<ExtendedAuthor>, OmniError> {
        match sqlx::query!(
            r#"
            SELECT
                authors.id, authors.fullname, authors.codename,
                COUNT(DISTINCT lines.id) AS quote_count,
                COUNT(lines.id) as line_count
            FROM authors LEFT JOIN lines ON authors.id = lines.author_id
            WHERE authors.id = $1 GROUP BY authors.id
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        {
            Ok(opt) => Ok(opt.map(|rec| ExtendedAuthor {
                author: Author {
                    id: rec.id,
                    fullname: rec.fullname,
                    codename: rec.codename,
                },
                quote_count: rec.quote_count.unwrap_or(0) as u32,
                line_count: rec.line_count.unwrap_or(0) as u32,
            })),
            Err(e) => Err(OmniError::from(e)),
        }
    }
    pub async fn get_all(pool: &PgPool) -> Result<Vec<ExtendedAuthor>, OmniError> {
        match sqlx::query!(
            r#"
            SELECT
                authors.id, authors.fullname, authors.codename,
                COUNT(DISTINCT lines.id) AS quote_count,
                COUNT(lines.id) as line_count
            FROM authors LEFT JOIN lines ON authors.id = lines.author_id
            GROUP BY authors.id
            ORDER BY authors.fullname
            "#
        )
        .fetch_all(pool)
        .await
        {
            Ok(opt) => Ok(opt
                .into_iter()
                .map(|rec| ExtendedAuthor {
                    author: Author {
                        id: rec.id,
                        fullname: rec.fullname,
                        codename: rec.codename,
                    },
                    quote_count: rec.quote_count.unwrap_or(0) as u32,
                    line_count: rec.line_count.unwrap_or(0) as u32,
                })
                .collect()),
            Err(e) => Err(OmniError::from(e)),
        }
    }
}
