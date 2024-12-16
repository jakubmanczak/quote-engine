use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use ulid::Ulid;

use crate::error::OmniError;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Author {
    #[serde(skip_deserializing)]
    #[serde(default = "Ulid::new")]
    pub id: Ulid,
    pub name: String,
    pub obfname: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum AuthorUpdate {
    #[serde(rename = "name")]
    Name(String),
    #[serde(rename = "obfname")]
    ObfName(String),
}

impl Author {
    // DATABASE QUERIES BELOW
    pub async fn get_by_id(id: Ulid, pool: &Pool<Sqlite>) -> Result<Option<Author>, OmniError> {
        let idstr = id.to_string();
        match sqlx::query!("SELECT * FROM authors WHERE id = ?", idstr)
            .fetch_optional(pool)
            .await
        {
            Ok(rec) => match rec {
                Some(rec) => Ok(Some(Author {
                    id,
                    name: rec.name,
                    obfname: rec.obfname,
                })),
                None => return Ok(None),
            },
            Err(e) => return Err(e)?,
        }
    }
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Author>, OmniError> {
        let recs = match sqlx::query!("SELECT * FROM authors").fetch_all(pool).await {
            Ok(recs) => recs,
            Err(e) => return Err(e)?,
        };
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
        match sqlx::query!("SELECT COUNT(id) AS count FROM users")
            .fetch_one(pool)
            .await
        {
            Ok(rec) => Ok(rec.count),
            Err(e) => Err(e)?,
        }
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
    pub async fn delete(id: Ulid, pool: &Pool<Sqlite>) -> Result<(), OmniError> {
        let idstr = id.to_string();
        match sqlx::query!("DELETE FROM authors WHERE id = ?", idstr)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e)?,
        }
    }
    pub async fn patch(
        id: Ulid,
        patch: AuthorUpdate,
        pool: &Pool<Sqlite>,
    ) -> Result<Option<Author>, OmniError> {
        let idstr = id.to_string();
        let author = match Author::get_by_id(id, pool).await? {
            Some(author) => author,
            None => return Ok(None),
        };
        match patch {
            AuthorUpdate::Name(name) => {
                match sqlx::query!("UPDATE authors SET name = ? WHERE id = ?", name, idstr)
                    .execute(pool)
                    .await
                {
                    Ok(_) => Ok(Some(Author {
                        id,
                        name,
                        obfname: author.obfname,
                    })),
                    Err(e) => Err(e)?,
                }
            }
            AuthorUpdate::ObfName(obfname) => match sqlx::query!(
                "UPDATE authors SET obfname = ? WHERE id = ?",
                obfname,
                idstr
            )
            .execute(pool)
            .await
            {
                Ok(_) => Ok(Some(Author {
                    id,
                    name: author.name,
                    obfname,
                })),
                Err(e) => Err(e)?,
            },
        }
    }
}
