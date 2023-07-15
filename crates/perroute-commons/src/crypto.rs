use std::str::FromStr;

use anyhow::bail;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use derive_getters::Getters;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error(transparent)]
    PasswordHash(#[from] argon2::password_hash::Error),
}

pub fn hash_password(clear_password: impl Into<String>) -> Result<String, CryptoError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password((clear_password.into() as String).as_bytes(), &salt)
        .map(|p| p.to_string())
        .map_err(CryptoError::from)
}

pub fn verify_password(
    clear_password: impl Into<String>,
    hash: impl Into<String>,
) -> Result<bool, CryptoError> {
    let clear_password: String = clear_password.into();
    let hash: String = hash.into();
    let p = PasswordHash::new(&hash).map_err(CryptoError::from)?;

    Argon2::default()
        .verify_password(clear_password.as_bytes(), &p)
        .map(|_| true)
        .map_err(CryptoError::from)
}

pub struct Key {
    prefix: String,
    suffix: String,
}

#[derive(Getters)]
pub struct HashResult {
    prefix: String,
    key: String,
    hash: String,
}

impl FromStr for Key {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let x = value.split('.').collect::<Vec<_>>();
        match (x.first(), x.get(1)) {
            (Some(prefix), Some(suffix)) => Ok(Self {
                prefix: prefix.to_string(),
                suffix: suffix.to_string(),
            }),
            _ => bail!("Invalid key"),
        }
    }
}

impl Key {
    pub fn random() -> Self {
        let key = uuid::Uuid::new_v4().to_string().replace('-', "");
        Self {
            prefix: key[0..6].to_owned(),
            suffix: key[6..].to_owned(),
        }
    }

    pub fn hash(self) -> HashResult {
        HashResult {
            prefix: self.prefix.clone(),
            key: format!("{}{}", self.prefix, self.suffix),
            hash: sha256::digest(format!("{}{}", self.prefix, self.suffix)),
        }
    }
}
