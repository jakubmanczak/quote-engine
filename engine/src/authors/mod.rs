use crate::error::OmniError;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use ulid::Ulid;

pub mod extended;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Author {
    #[serde(skip_deserializing)]
    #[serde(default = "Ulid::new")]
    pub id: Ulid,
    pub name: String,
    pub obfname: String,
}

#[derive(Deserialize)]
pub struct AuthorPatch {
    pub name: Option<String>,
    pub obfname: Option<String>,
}

impl Author {
    // DATABASE QUERIES BELOW
    pub async fn get_by_id(id: Ulid, pool: &Pool<Sqlite>) -> Result<Option<Author>, OmniError> {
        let idstr = id.to_string();
        match sqlx::query!("SELECT * FROM authors WHERE id = ?", idstr)
            .fetch_optional(pool)
            .await?
        {
            Some(rec) => Ok(Some(Author {
                id,
                name: rec.name,
                obfname: rec.obfname,
            })),
            None => return Ok(None),
        }
    }
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Author>, OmniError> {
        let recs = sqlx::query!("SELECT * FROM authors")
            .fetch_all(pool)
            .await?;
        let vec: Vec<Author> = recs
            .into_iter()
            .map(|rec| {
                Ok(Author {
                    id: Ulid::from_string(&rec.id).map_err(OmniError::UlidDecodeError)?,
                    name: rec.name,
                    obfname: rec.obfname,
                })
            })
            .collect::<Result<Vec<Author>, OmniError>>()?;

        Ok(vec)
    }
    pub async fn get_db_count(pool: &Pool<Sqlite>) -> Result<i64, OmniError> {
        Ok(sqlx::query!("SELECT COUNT(id) AS count FROM users")
            .fetch_one(pool)
            .await?
            .count)
    }
    pub async fn post(author: Author, pool: &Pool<Sqlite>) -> Result<Author, OmniError> {
        let idstr = author.id.to_string();
        match sqlx::query!(
            "INSERT INTO authors VALUES (?, ?, ?)",
            idstr,
            author.name,
            author.obfname
        )
        .execute(pool)
        .await
        {
            Ok(_) => Ok(author),
            Err(e) => Err(e)?,
        }
    }
    pub async fn delete(self, pool: &Pool<Sqlite>) -> Result<(), OmniError> {
        let idstr = self.id.to_string();
        match sqlx::query!("DELETE FROM authors WHERE id = ?", idstr)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e)?,
        }
    }
    pub async fn patch(self, patch: AuthorPatch, pool: &Pool<Sqlite>) -> Result<Author, OmniError> {
        let author = Author {
            id: self.id,
            name: patch.name.unwrap_or(self.name),
            obfname: patch.obfname.unwrap_or(self.obfname),
        };
        let name = &author.name;
        let obfname = &author.obfname;
        match sqlx::query!("UPDATE authors SET name = ?, obfname = ?", name, obfname)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(author),
            Err(e) => Err(e)?,
        }
    }
}
