use super::connection::Connection;
use super::schema::Schema;
use super::{business_unit::BusinessUnit, channel::Channel};
use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, properties::Properties};
use sqlx::{types::Json, FromRow, PgExecutor, Postgres, QueryBuilder};

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct RouteQuery {
    id: Option<Id>,
    bu_id: Option<Id>,
    message_type_id: Option<Id>,
    schema_id: Option<Id>,
    connection_id: Option<Id>,
    enabled: Option<bool>,
}

impl ModelQueryBuilder<Route> for RouteQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, Postgres> {
        let mut builder = projection.query_builder();
        builder.push(" FROM routes WHERE 1=1");

        if let Some(id) = &self.id {
            builder.push(" and id = ");
            builder.push_bind(id);
        }

        if let Some(bu_id) = &self.bu_id {
            builder.push(" and bu_id = ");
            builder.push_bind(bu_id);
        }

        if let Some(message_type_id) = &self.message_type_id {
            builder.push(" and message_type_id = ");
            builder.push_bind(message_type_id);
        }

        if let Some(schema_id) = &self.schema_id {
            builder.push(" and schema_id = ");
            builder.push_bind(schema_id);
        }

        if let Some(connection_id) = &self.connection_id {
            builder.push(" and connection_id = ");
            builder.push_bind(connection_id);
        }

        if let Some(enabled) = &self.enabled {
            builder.push(" and enabled = ");
            builder.push_bind(enabled);
        }

        builder
    }
}

impl DatabaseModel for Route {}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Route {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    schema_id: Id,
    #[setters(skip)]
    channel_id: Id,
    #[setters(skip)]
    bu_id: Id,

    properties: Json<Properties>,
}

impl Route {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        todo!()
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        todo!()
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        todo!()
    }

    pub async fn schema<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Schema, sqlx::Error> {
        todo!()
    }

    pub async fn channel<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Channel, sqlx::Error> {
        todo!()
    }

    pub async fn connection<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Connection, sqlx::Error> {
        todo!()
    }

    pub async fn bu<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<BusinessUnit, sqlx::Error> {
        todo!()
    }
}
