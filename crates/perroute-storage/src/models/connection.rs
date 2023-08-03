use derive_getters::Getters;
use perroute_commons::types::id::Id;
use sqlx::{types::Json, Executor, FromRow};
use std::{collections::HashMap, todo};

#[derive(Debug, FromRow, Getters)]
pub struct Connection {
    id: uuid::Uuid,
    code: String,
    plugin_id: String,
    description: String,
    properties: Json<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct ConnectionsQuery {
    id: Option<Id>,
}

impl Connection {
    pub async fn query<'e, E: Executor<'e>>(
        _exec: E,
        _query: ConnectionsQuery,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(vec![])
    }

    pub async fn save<'e, E: Executor<'e>>(self, _exec: E) -> Result<Self, sqlx::Error> {
        Ok(self)
    }

    pub async fn exists_code<'e, E: Executor<'e>>(
        _exec: E,
        _code: &str,
    ) -> Result<bool, sqlx::Error> {
        todo!()
    }
}
