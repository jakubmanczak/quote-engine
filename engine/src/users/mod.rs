use crate::error::OmniError;
use attributes::UserAttribute;
use patch::{UserPatch, UserPatchError};
use serde::{Deserialize, Serialize};
use sessions::UserSession;
use sqlx::{Pool, Sqlite};
use ulid::Ulid;

pub mod attributes;
pub mod defaultadmin;
pub mod patch;
pub mod sessions;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    #[serde(skip_deserializing)]
    #[serde(default = "Ulid::new")]
    pub id: Ulid,
    pub name: String,
    #[serde(skip_deserializing)]
    #[serde(default = "default_clearance")]
    pub clearance: u8,
    #[serde(skip_deserializing)]
    #[serde(default = "attributes::default_attributes_u64")]
    attributes: u64,
}

fn default_clearance() -> u8 {
    1
}

impl User {
    pub fn has_attribute(&self, attr: UserAttribute) -> bool {
        self.attributes & attr.get_bit() != 0
    }
    pub fn has_permission(&self, attr: UserAttribute) -> bool {
        (self.attributes & attr.get_bit()) != 0
            || (self.attributes & UserAttribute::TheEverythingPermission.get_bit() != 0)
    }
    // pub fn attributes_vec(&self) -> Vec<UserAttribute> {
    //     UserAttribute::VARIANTS
    //         .iter()
    //         .filter_map(|variant| {
    //             if self.attributes & variant.get_bit() != 0 {
    //                 Some(variant.to_owned())
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect()
    // }

    // ONLY DATABASE QUERIES BELOW
    pub async fn get_by_id(id: Ulid, pool: &Pool<Sqlite>) -> Result<Option<User>, OmniError> {
        let idstr = id.to_string();
        match sqlx::query!(
            "SELECT name, clearance, attributes FROM users WHERE id = ?",
            idstr
        )
        .fetch_optional(pool)
        .await
        {
            Ok(rec) => match rec {
                Some(rec) => Ok(Some(User {
                    id,
                    name: rec.name,
                    clearance: rec.clearance as u8,
                    attributes: rec.attributes as u64,
                })),
                None => return Ok(None),
            },
            Err(e) => return Err(e)?,
        }
    }
    pub async fn get_by_username(
        username: &str,
        pool: &Pool<Sqlite>,
    ) -> Result<Option<User>, OmniError> {
        match sqlx::query!(
            "SELECT id, clearance, attributes FROM users WHERE name = ?",
            username
        )
        .fetch_optional(pool)
        .await
        {
            Ok(opt) => match opt {
                Some(rec) => Ok(Some(User {
                    id: Ulid::from_string(&rec.id)?,
                    name: username.to_string(),
                    clearance: rec.clearance as u8,
                    attributes: rec.attributes as u64,
                })),
                None => return Ok(None),
            },
            Err(e) => return Err(e)?,
        }
    }
    pub async fn patch(self, patch: UserPatch, pool: &Pool<Sqlite>) -> Result<User, OmniError> {
        if self.id.is_nil() && patch.clearance.is_some() {
            return Err(UserPatchError::NoChangeRootClearance)?;
        }
        let user = User {
            id: self.id,
            name: patch.name.unwrap_or(self.name),
            clearance: patch.clearance.unwrap_or(self.clearance),
            attributes: self.attributes,
        };
        let name = &user.name;
        let clearance = user.clearance as i64;
        match sqlx::query!("UPDATE users SET name = ?, clearance = ?", name, clearance)
            .execute(pool)
            .await
        {
            Ok(_) => Ok(user),
            Err(e) => return Err(e)?,
        }
    }
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<User>, OmniError> {
        let recs = match sqlx::query!(
            "SELECT id, name, clearance, attributes FROM users ORDER BY clearance, id"
        )
        .fetch_all(pool)
        .await
        {
            Ok(records) => records,
            Err(e) => return Err(e)?,
        };

        recs.into_iter()
            .map(|record| {
                Ok(User {
                    id: Ulid::from_string(&record.id)?,
                    name: record.name,
                    clearance: record.clearance as u8,
                    attributes: record.attributes as u64,
                })
            })
            .collect()
    }
    pub async fn get_sessions(&self, pool: &Pool<Sqlite>) -> Result<Vec<UserSession>, OmniError> {
        let idstr = self.id.to_string();
        match sqlx::query!("SELECT * FROM sessions WHERE user = ?", idstr)
            .fetch_all(pool)
            .await
        {
            Ok(records) => records
                .into_iter()
                .map(|r| {
                    Ok(UserSession {
                        id: Ulid::from_string(&r.id)?,
                        userid: self.id,
                        // token: r.token,
                        issued: r.issued,
                        expiry: r.expiry,
                        lastaccess: r.lastaccess,
                    })
                })
                .collect::<Result<Vec<UserSession>, OmniError>>(),
            Err(e) => return Err(e)?,
        }
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
}
