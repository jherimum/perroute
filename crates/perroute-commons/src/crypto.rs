use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

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

pub struct ApiKeyHasher;

impl ApiKeyHasher {
    pub fn hash(&self, api_key: &str) -> String {
        bcrypt::hash(api_key, bcrypt::DEFAULT_COST).unwrap()
    }
}
