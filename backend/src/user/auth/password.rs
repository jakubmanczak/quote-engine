use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;

use crate::omnierror::OmniError;

pub fn hash_password(password: &str) -> Result<String, OmniError> {
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    match argon.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => Err(e)?,
    }
}

pub fn verify_password(candidate: &str, hash: &str) -> Result<bool, OmniError> {
    let argon = Argon2::default();
    let hash = PasswordHash::new(hash)?;
    Ok(argon.verify_password(candidate.as_bytes(), &hash).is_ok())
}
