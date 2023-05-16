use super::account::Account;
use serde_json::Value;
use sqlx::{Executor, FromRow};

#[derive(Debug, FromRow)]
pub struct Connection {
    pub id: uuid::Uuid,
    pub code: String,
    pub account_id: uuid::Uuid,
    pub plugin_id: &'static str,
    pub description: String,
    pub properties: Value,
}

impl Connection {
    pub fn new(
        code: &str,
        account: &Account,
        plugin_id: &'static str,
        description: &str,
        properties: Value,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            code: code.to_owned(),
            account_id: account.id,
            plugin_id,
            description: description.to_owned(),
            properties,
        }
    }

    pub async fn save<'e, E: Executor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        Ok(self)
    }
}
