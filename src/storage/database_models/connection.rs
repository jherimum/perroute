use crate::connector::ConnectorPlugin;
use serde_json::Value;
use sqlx::{Executor, FromRow};
use std::todo;

#[derive(Debug, FromRow)]
pub struct Connection {
    pub id: uuid::Uuid,
    pub code: String,
    pub plugin_id: String,
    pub description: String,
    pub properties: Value,
}

#[derive(Debug)]
pub struct ConnectionsQuery;

impl Connection {
    pub fn new(
        code: impl Into<String>,
        plugin: &dyn ConnectorPlugin,
        description: impl Into<String>,
        properties: &Value,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            code: code.into(),
            plugin_id: plugin.id().to_owned(),
            description: description.into(),
            properties: properties.clone(),
        }
    }

    pub async fn query<'e, E: Executor<'e>>(
        exec: E,
        query: ConnectionsQuery,
    ) -> Result<Vec<Connection>, sqlx::Error> {
        Ok(vec![])
    }

    pub async fn save<'e, E: Executor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        Ok(self)
    }

    pub async fn exists_code<'e, E: Executor<'e>>(
        exec: E,
        code: &str,
    ) -> Result<bool, sqlx::Error> {
        todo!()
    }
}
