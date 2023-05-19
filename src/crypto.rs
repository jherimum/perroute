use std::todo;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use crate::{errors::OmniMessageError, types::OmniResult};

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error(transparent)]
    Argon2(#[from] argon2::password_hash::Error),
}

pub fn hash_password(clear_password: impl Into<String>) -> OmniResult<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password((clear_password.into() as String).as_bytes(), &salt)
        .map(|p| p.to_string())
        .map_err(CryptoError::from)
        .map_err(OmniMessageError::from)
}

pub fn verify_password(
    clear_password: impl Into<String>,
    hash: impl Into<String>,
) -> OmniResult<bool> {
    let clear_password: String = clear_password.into();
    let hash: String = hash.into();
    let p = PasswordHash::new(&hash)
        .map_err(CryptoError::from)
        .map_err(OmniMessageError::from)?;
    Argon2::default()
        .verify_password(clear_password.as_bytes(), &p)
        .map(|_| true)
        .map_err(CryptoError::from)
        .map_err(OmniMessageError::from)
}
