use super::business_unit::BusinessUnitQueryBuilder;
use super::channel::ChannelQueryBuilder;
use super::connection::{Connection, ConnectionQueryBuilder};
use super::message_type::{MessageType, MessageTypeQueryBuilder};
use super::{business_unit::BusinessUnit, channel::Channel};
use crate::query::FetchableModel;
use crate::{log_query_error, Result};
use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::priority::Priority;
use perroute_commons::types::{id::Id, properties::Properties};
use perroute_connectors::types::dispatch_type::DispatchType;
use sqlx::{FromRow, PgExecutor, Postgres, QueryBuilder};
use std::ops::Deref;
use tap::TapFallible;

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct RouteQuery {
    id: Option<Id>,
    business_unit_id: Option<Id>,
    message_type_id: Option<Id>,
    connection_id: Option<Id>,
    channel_id: Option<Id>,
}

impl RouteQuery {
    pub fn with_id(id: Id) -> RouteQuery {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn with_message_type_id(id: Id) -> RouteQuery {
        Self {
            message_type_id: Some(id),
            ..Default::default()
        }
    }
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
    channel_id: Id,

    #[setters(skip)]
    business_unit_id: Id,

    #[setters(skip)]
    message_type_id: Id,

    #[setters(skip)]
    connection_id: Id,

    enabled: bool,

    priority: Priority,

    properties: Properties,
}

impl Route {
    pub async fn delete_by_channel<'e, E: PgExecutor<'e>>(exec: E, channel_id: &Id) -> Result<u64> {
        Ok(sqlx::query("DELETE FROM routes WHERE channel_id= $1")
            .bind(channel_id)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected())?)
    }

    pub async fn dispatch_route_stack<'e, E: PgExecutor<'e>>(
        exec: E,
        dispatch_type: &DispatchType,
    ) -> Result<Vec<Route>> {
        let query = r#"
            SELECT r.* 
            FROM routes r
            INNER JOIN channels c
            ON c.id = r.channel_id
            WHERE c.dispatch_type = $1 
            AND c.enabled = true
            AND t.enabled = true
            ORDER by r.priority DESC
        "#;

        todo!()
    }

    pub async fn batch_delete<'e, E: PgExecutor<'e>>(ids: Vec<Id>, exec: E) -> Result<u64> {
        let uuids = ids.iter().map(|id| *id.deref()).collect::<Vec<_>>();
        Ok(sqlx::query("DELETE FROM routes WHERE id= ANY($1)")
            .bind(uuids)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected())?)
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO routes 
            (
                id, 
                channel_id, 
                business_unit_id, 
                message_type_id, 
                connection_id, 
                properties, 
                priority, 
                enabled) 
            VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.channel_id)
        .bind(self.business_unit_id)
        .bind(self.message_type_id)
        .bind(self.connection_id)
        .bind(self.properties)
        .bind(self.priority)
        .bind(self.enabled)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
            UPDATE routes 
            SET 
                properties= $2, 
                priority= $3,
                enabled = $4
            WHERE id= $1 RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.properties)
        .bind(self.priority)
        .bind(self.enabled)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool> {
        Ok(sqlx::query(
            r#"
                DELETE FROM routes 
                WHERE id= $1
                "#,
        )
        .bind(self.id)
        .execute(exec)
        .await
        .tap_err(log_query_error!())
        .map(|result| result.rows_affected() > 0)?)
    }

    pub async fn channel<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Channel> {
        Channel::find_one(
            exec,
            ChannelQueryBuilder::default()
                .id(Some(self.channel_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn connection<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Connection> {
        Connection::find_one(
            exec,
            ConnectionQueryBuilder::default()
                .id(Some(self.connection_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn business_unit<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<BusinessUnit> {
        BusinessUnit::find_one(
            exec,
            BusinessUnitQueryBuilder::default()
                .id(Some(self.connection_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn message_type<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<MessageType> {
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
