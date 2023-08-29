use super::channel::{Channel, ChannelQueryBuilder};
use crate::{
    log_query_error,
    query::{FetchableModel, ModelQueryBuilder, Projection},
    DatabaseModel, Result,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, properties::Properties};
use perroute_connectors::types::plugin_id::ConnectorPluginId;
use sqlx::{FromRow, PgExecutor};
use tap::TapFallible;

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
    plugin_id: Option<ConnectorPluginId>,
}

impl ConnectionQuery {
    pub fn with_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }
}

impl ModelQueryBuilder<Connection> for ConnectionQuery {
    fn build(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM connections where 1=1");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        if let Some(plugin_id) = self.plugin_id {
            builder.push(" AND plugin_id = ");
            builder.push_bind(plugin_id);
        }

        builder
    }
}

impl DatabaseModel for Connection {}

impl Connection {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO connections (id, name, plugin_id, enabled, properties ) 
            VALUES($1, $2, $3, $4, $5) RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.name)
        .bind(self.plugin_id)
        .bind(self.enabled)
        .bind(self.properties)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
            UPDATE connections 
            SET name= $2, enabled=$3, properties=$4 
            WHERE id= $1 RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.name)
        .bind(self.enabled)
        .bind(self.properties)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool> {
        Ok(sqlx::query(
            r#"
                DELETE FROM connections 
                WHERE id= $1
                "#,
        )
        .bind(self.id)
        .execute(exec)
        .await
        .tap_err(log_query_error!())
        .map(|result| result.rows_affected() > 0)?)
    }

    pub async fn channels<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Vec<Channel>> {
        Channel::query(
            exec,
            ChannelQueryBuilder::default()
                .connection_id(Some(self.id))
                .build()
                .unwrap(),
        )
        .await
    }
}
