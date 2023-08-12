use crate::{query::ModelQueryBuilder, DatabaseModel};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::id::Id;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow, PgExecutor, QueryBuilder, Type};
use std::collections::HashMap;

impl DatabaseModel for MessageDispatch {}

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct MessageDispatchQuery {
    id: Option<Id>,
    message_id: Option<Id>,
    route_id: Option<Id>,
    status: Option<MessageDispatchStatus>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type)]
pub struct MessageDispatchResult {
    connection_properties: Json<HashMap<String, String>>,
    dispatcher_properties: Json<HashMap<String, String>>,
    response: Json<HashMap<String, String>>,
}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct MessageDispatch {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    message_id: Id,
    #[setters(skip)]
    route_id: Id,
    status: MessageDispatchStatus,
    result: Option<MessageDispatchResult>,
}

impl MessageDispatch {
    pub async fn save<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO message_dispatches (id, message_id, route_id, status, result)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.message_id)
        .bind(self.route_id)
        .bind(self.status)
        .bind(self.result.clone())
        .fetch_one(executor)
        .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE message_dispatches
            SET status = $2, result = $3
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.status)
        .bind(self.result.clone())
        .fetch_one(executor)
        .await
    }
}
