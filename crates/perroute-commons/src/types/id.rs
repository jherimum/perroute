use std::fmt::Display;

use base58::ToBase58;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type, Hash,
)]
#[sqlx(transparent)]
pub struct Id(String);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7().as_bytes().to_base58())
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<&Id> for String {
    fn from(id: &Id) -> Self {
        id.0.clone()
    }
}

impl From<&Id> for Id {
    fn from(id: &Id) -> Self {
        id.clone()
    }
}

impl AsRef<String> for Id {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&String> for Id {
    fn from(s: &String) -> Self {
        Self(s.to_string())
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
