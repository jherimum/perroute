use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, properties::Properties};
use perroute_connectors::types::ConnectorPluginId;
use sqlx::{types::Json, Executor, FromRow};
use std::todo;

#[derive(Debug, FromRow, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Connection {
    id: uuid::Uuid,
    name: String,
    plugin_id: ConnectorPluginId,
    enabled: bool,
    properties: Json<Properties>,
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
