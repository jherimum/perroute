use base58::ToBase58;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Id(String);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7().as_bytes().to_base58())
    }
}
