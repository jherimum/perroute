use derive_getters::Getters;
use perroute_commons::types::{email::Email, id::Id};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgExecutor};

#[derive(Debug, FromRow, Getters, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    id: Id,
    email: Email,
}

impl User {
    pub fn new(email: Email) -> Self {
        Self {
            id: Id::new(),
            email,
        }
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, _exec: E) -> Result<User, sqlx::Error> {
        todo!()
    }
}
