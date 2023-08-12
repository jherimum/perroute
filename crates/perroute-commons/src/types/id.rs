use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::{fmt::Display, str::FromStr};

#[macro_export]
macro_rules! new_id {
    () => {
        $crate::types::id::Id::new()
    };
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Invalid id {0}")]
pub struct ParseError(#[from] uuid::Error);

#[derive(Debug, PartialEq, Eq, Clone, Type, Copy, Deserialize, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Id(pub uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Id {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::parse_str(s)?))
    }
}

impl From<uuid::Uuid> for Id {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
