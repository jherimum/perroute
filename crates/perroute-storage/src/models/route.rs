use super::business_unit::BusinessUnitQueryBuilder;
use super::channel::ChannelQueryBuilder;
use super::connection::{Connection, ConnectionQueryBuilder};
use super::message_type::{MessageType, MessageTypeQueryBuilder};
use super::schema::{Schema, SchemasQueryBuilder};
use super::{business_unit::BusinessUnit, channel::Channel};
use crate::log_query_error;
use crate::query::FetchableModel;
use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, properties::Properties};
use sqlx::{FromRow, PgExecutor, Postgres, QueryBuilder};
use tap::TapFallible;

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct RouteQuery {
    id: Option<Id>,
    business_unit_id: Option<Id>,
    message_type_id: Option<Id>,
    schema_id: Option<Id>,
    connection_id: Option<Id>,
    channel_id: Option<Id>,
}

impl ModelQueryBuilder<Route> for RouteQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, Postgres> {
        let mut builder = projection.query_builder();
        builder.push(" FROM routes WHERE 1=1");

        if let Some(id) = &self.id {
            builder.push(" and id = ");
            builder.push_bind(id);
        }

        if let Some(business_unit_id) = &self.business_unit_id {
            builder.push(" and business_unit_id = ");
            builder.push_bind(business_unit_id);
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
    business_unit_id: Id,

    #[setters(skip)]
    message_type_id: Id,

    #[setters(skip)]
    connection_id: Id,

    properties: Properties,
}

impl Route {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO routes (id, schema_id, channel_id, business_unit_id, message_type_id, connection_id, properties ) 
            VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.schema_id)
        .bind(self.channel_id)
        .bind(self.business_unit_id)
        .bind(self.message_type_id)
        .bind(self.connection_id)
        .bind(self.properties)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE routes 
            SET properties= $2
            WHERE id= $1 RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.properties)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        sqlx::query(
            r#"
                DELETE FROM routes 
                WHERE id= $1
                "#,
        )
        .bind(self.id)
        .execute(exec)
        .await
        .tap_err(log_query_error!())
        .map(|result| result.rows_affected() > 0)
    }

    pub async fn schema<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Schema, sqlx::Error> {
        Schema::find_one(
            exec,
            SchemasQueryBuilder::default()
                .id(Some(self.schema_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn channel<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Channel, sqlx::Error> {
        Channel::find_one(
            exec,
            ChannelQueryBuilder::default()
                .id(Some(self.channel_id))
                .build()
                .unwrap(),
        )
        .await
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
                .id(Some(self.connection_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn message_type<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<MessageType, sqlx::Error> {
        MessageType::find_one(
            exec,
            MessageTypeQueryBuilder::default()
                .id(Some(self.message_type_id))
                .build()
                .unwrap(),
        )
        .await
    }
}
