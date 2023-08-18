use super::{
    business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
    connection::{Connection, ConnectionQueryBuilder},
    route::{Route, RouteQueryBuilder},
};
use crate::{
    log_query_error,
    query::{FetchableModel, ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, priority::Priority, properties::Properties};
use perroute_connectors::types::DispatchType;
use sqlx::{FromRow, PgExecutor};
use tap::TapFallible;

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct ChannelQuery {
    id: Option<Id>,
    business_unit_id: Option<Id>,
    connection_id: Option<Id>,
}

impl ChannelQuery {
    pub fn with_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn with_business_unit(business_unit_id: Id) -> Self {
        Self {
            business_unit_id: Some(business_unit_id),
            ..Default::default()
        }
    }
}

impl ModelQueryBuilder<Channel> for ChannelQuery {
    fn build(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM channels where 1=1");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        if let Some(business_unit_id) = self.business_unit_id {
            builder.push(" AND business_unit_id = ");
            builder.push_bind(business_unit_id);
        }

        if let Some(connection_id) = self.connection_id {
            builder.push(" AND connection_id = ");
            builder.push_bind(connection_id);
        }

        builder
    }
}

impl DatabaseModel for Channel {}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Channel {
    #[setters(skip)]
    id: Id,

    #[setters(skip)]
    dispatch_type: DispatchType,

    properties: Properties,
    enabled: bool,
    priority: Priority,

    #[setters(skip)]
    connection_id: Id,
    #[setters(skip)]
    business_unit_id: Id,
}

impl Channel {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO channels (id, dispatch_type, properties, enabled, priority, connection_id, business_unit_id ) 
            VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.dispatch_type)
        .bind(self.properties)
        .bind(self.enabled)
        .bind(self.priority)
        .bind(self.connection_id)
        .bind(self.business_unit_id)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE channels 
            SET properties= $2, priority=$3, enabled=$4 
            WHERE id= $1 RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.properties)
        .bind(self.priority)
        .bind(self.enabled)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        sqlx::query(
            r#"
                DELETE FROM channels 
                WHERE id= $1
                "#,
        )
        .bind(self.id)
        .execute(exec)
        .await
        .tap_err(log_query_error!())
        .map(|result| result.rows_affected() > 0)
    }

    pub async fn connection<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Connection, sqlx::Error> {
        Connection::find_one(
            exec,
            ConnectionQueryBuilder::default()
                .id(Some(self.connection_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn business_unit<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<BusinessUnit, sqlx::Error> {
        BusinessUnit::find_one(
            exec,
            BusinessUnitQueryBuilder::default()
                .id(Some(self.business_unit_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn routes<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<Route>, sqlx::Error> {
        Route::query(
            exec,
            RouteQueryBuilder::default()
                .channel_id(Some(self.id))
                .build()
                .unwrap(),
        )
        .await
    }
}
