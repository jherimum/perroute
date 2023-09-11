use super::message::{Message, MessageQueryBuilder};
use crate::{
    query::{FetchableModel, ModelQueryBuilder},
    DatabaseModel, Result,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::id::Id;
use perroute_connectors::types::{delivery::Delivery, plugin_id::ConnectorPluginId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{types::Json, FromRow, PgExecutor, QueryBuilder, Type};

impl DatabaseModel for MessageDispatch {}

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct MessageDispatchQuery {
    id: Option<Id>,
    message_id: Option<Id>,
    route_id: Option<Id>,
    status: Option<MessageDispatchStatus>,
}

impl MessageDispatchQuery {
    pub fn with_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }
}

impl MessageDispatchQuery {
    pub fn with_route_id(route_id: Id) -> Self {
        Self {
            route_id: Some(route_id),
            ..Default::default()
        }
    }
}

impl ModelQueryBuilder<MessageDispatch> for MessageDispatchQuery {
    fn build(&self, projection: crate::query::Projection) -> QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM message_dispatches WHERE 1=1");

        if let Some(id) = &self.id {
            builder.push(" and id = ");
            builder.push_bind(id);
        }

        if let Some(message_id) = &self.message_id {
            builder.push(" and message_id = ");
            builder.push_bind(message_id);
        }

        if let Some(route_id) = &self.route_id {
            builder.push(" and route_id = ");
            builder.push_bind(route_id);
        }

        if let Some(status) = &self.status {
            builder.push(" and status = ");
            builder.push_bind(status);
        }

        builder
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Copy)]
#[sqlx(type_name = "message_dispatch_status", rename_all = "snake_case")]
pub enum MessageDispatchStatus {
    Pending,
    Success,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Default)]
pub struct MessageDispatchResult {
    reference: Option<String>,
    response_data: Option<Value>,
}

impl MessageDispatchResult {
    pub fn new(reference: Option<String>, response_data: Option<Value>) -> Self {
        Self {
            reference,
            response_data,
        }
    }
}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Builder, Setters)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct MessageDispatch {
    id: Id,
    success: bool,
    result: Option<MessageDispatchResult>,
    message_id: Id,
    plugin_id: ConnectorPluginId,
    created_at: chrono::NaiveDateTime,
    delivery: Json<Delivery>,
}

impl MessageDispatch {
    pub async fn save_all<'e, E: PgExecutor<'e>>(
        exec: E,
        dispatches: Vec<Self>,
    ) -> Result<Vec<Self>> {
        todo!()
    }

    pub async fn message<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Message> {
        Message::find_one(
            executor,
            MessageQueryBuilder::default()
                .id(Some(self.message_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn save<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO message_dispatches (id, message_id, success, plugin_id, delivery, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.message_id)
        .bind(self.success)
        .bind(self.plugin_id)
        .bind(self.delivery.clone())
        .bind(self.created_at)
        .fetch_one(executor)
        .await?)
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        todo!()
    }
}
