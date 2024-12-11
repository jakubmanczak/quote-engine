use attributes::UserAttribute;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use strum::VariantArray;
use ulid::Ulid;

use crate::error::OmniError;
pub mod attributes;
pub mod defaultadmin;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_deserializing)]
    #[serde(default = "Ulid::new")]
    pub id: Ulid,
    pub name: String,
    #[serde(skip_deserializing)]
    pub color: String,
    #[serde(skip_deserializing)]
    pub picture: String,
    #[serde(skip_deserializing)]
    #[serde(default = "default_clearance")]
    clearance: u8,
    #[serde(skip_deserializing)]
    #[serde(default = "attributes::default_attributes_u64")]
    attributes: u64,
}

fn default_clearance() -> u8 {
    1
}

impl User {
    pub fn has_attribute(&self, attr: UserAttribute) -> bool {
        (self.attributes & attr.get_bit()) != 0
    }
    pub fn attributes_vec(&self) -> Vec<UserAttribute> {
        UserAttribute::VARIANTS
            .iter()
            .filter_map(|variant| {
                if self.attributes & variant.get_bit() != 0 {
                    Some(variant.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }

    // ONLY DATABASE QUERIES BELOW
    pub async fn get_by_id(id: Ulid, pool: &Pool<Sqlite>) -> Result<Option<User>, OmniError> {
        let idstr = id.to_string();
        let rec = match sqlx::query!(
            "SELECT name, clearance, attributes, color, picture FROM users WHERE id = ?",
            idstr
        )
        .fetch_optional(pool)
        .await
        {
            Ok(rec) => match rec {
                Some(rec) => rec,
                None => return Ok(None),
            },
            Err(e) => return Err(OmniError::SqlxError(e)),
        };

        Ok(Some(User {
            id,
            name: rec.name,
            color: rec.color,
            picture: rec.picture,
            attributes: rec.attributes as u64,
            clearance: rec.clearance as u8,
        }))
    }
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<User>, OmniError> {
        let recs = match sqlx::query!(
            "SELECT id, name, clearance, attributes, color, picture FROM users ORDER BY clearance, id"
        )
        .fetch_all(pool)
        .await
        {
            Ok(records) => records,
            Err(e) => return Err(OmniError::SqlxError(e)),
        };

        recs.into_iter()
            .map(|record| {
                Ok(User {
                    id: Ulid::from_string(&record.id)?,
                    name: record.name,
                    color: record.color,
                    picture: record.picture,
                    clearance: record.clearance as u8,
                    attributes: record.attributes as u64,
                })
            })
            .collect()
    }
    pub async fn get_db_count(pool: &Pool<Sqlite>) -> Result<i64, OmniError> {
        match sqlx::query!("SELECT COUNT(id) AS count FROM users")
            .fetch_one(pool)
            .await
        {
            Ok(rec) => Ok(rec.count),
            Err(e) => Err(OmniError::SqlxError(e)),
        }
    }
}
