use std::sync::Arc;

use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, properties::Properties};
use perroute_connectors::{api::ConnectorPlugin, types::ConnectorPluginId, Plugins};
use sqlx::{Executor, FromRow};

#[derive(Debug, FromRow, Getters, Setters, Builder, Clone)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Connection {
    #[setters(skip)]
    id: Id,
    name: String,

    #[setters(skip)]
    plugin_id: ConnectorPluginId,
    enabled: bool,
    properties: Properties,
}

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct ConnectionQuery {
    id: Option<Id>,
}

impl ModelQueryBuilder<Connection> for ConnectionQuery {
    fn build(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM connections where 1=1");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        builder
    }
}

impl DatabaseModel for Connection {}

impl Connection {
    pub fn plugin(&self, plugins: &Plugins) -> Option<Arc<dyn ConnectorPlugin>> {
        plugins.get(self.plugin_id())
    }

    pub async fn save<'e, E: Executor<'e>>(self, _exec: E) -> Result<Self, sqlx::Error> {
        Ok(self)
    }

    pub async fn update<'e, E: Executor<'e>>(self, _exec: E) -> Result<Self, sqlx::Error> {
        Ok(self)
    }

    pub async fn delete<'e, E: Executor<'e>>(self, _exec: E) -> Result<Self, sqlx::Error> {
        Ok(self)
    }
}
