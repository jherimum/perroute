use std::collections::HashSet;

use crate::{log_query_error, query::ModelQueryBuilder, DatabaseModel};
use chrono::NaiveDateTime;
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{
    dispatch_type::DispatcherType, id::Id, payload::Payload, recipient::Recipient,
};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow, PgExecutor};
use tap::TapFallible;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, Copy)]
#[sqlx(type_name = "message_status", rename_all = "snake_case")]
pub enum Status {
    Pending,
    Distributed,
}

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct MessageQuery {
    pub id: Option<Id>,
    pub status: Option<Status>,
    pub schema_id: Option<Id>,
    pub message_type_id: Option<Id>,
    pub channel_id: Option<Id>,
    pub scheduled_from: Option<NaiveDateTime>,
    pub scheduled_to: Option<NaiveDateTime>,
}

impl ModelQueryBuilder<Message> for MessageQuery {
    fn build(
        &self,
        projection: crate::query::Projection,
    ) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();
        builder.push("FROM messages WHERE 1=1");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        if let Some(status) = self.status {
            builder.push(" AND status = ");
            builder.push_bind(status);
        }

        if let Some(schema_id) = self.schema_id {
            builder.push(" AND schema_id = ");
            builder.push_bind(schema_id);
        }

        if let Some(message_type_id) = self.message_type_id {
            builder.push(" AND message_type_id = ");
            builder.push_bind(message_type_id);
        }

        if let Some(channel_id) = self.channel_id {
            builder.push(" AND channel_id = ");
            builder.push_bind(channel_id);
        }

        if let Some(scheduled_from) = self.scheduled_from {
            builder.push(" AND scheduled_to >= ");
            builder.push_bind(scheduled_from);
        }

        if let Some(scheduled_to) = self.scheduled_to {
            builder.push(" AND scheduled_to <= ");
            builder.push_bind(scheduled_to);
        }

        builder
    }
}

impl DatabaseModel for Message {}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Message {
    #[setters(skip)]
    id: Id,

    #[setters(skip)]
    payload: Payload,

    status: Status,

    #[setters(skip)]
    #[builder(default)]
    scheduled_to: Option<NaiveDateTime>,

    #[setters(skip)]
    schema_id: Id,

    #[setters(skip)]
    message_type_id: Id,

    #[setters(skip)]
    channel_id: Id,

    #[setters(skip)]
    recipient: Json<Recipient>,

    #[setters(skip)]
    #[builder(default)]
    include_dispatcher_types: Json<HashSet<DispatcherType>>,

    #[setters(skip)]
    #[builder(default)]
    exclude_dispatcher_types: Json<HashSet<DispatcherType>>,
}

impl Message {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                INSERT INTO messages (id, payload, status, scheduled_to, schema_id, message_type_id, channel_id, recipient, include_dispatcher_types, exclude_dispatcher_types) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *"#,
            ).bind(self.id)
            .bind(self.payload)
            .bind(self.status)
            .bind(self.scheduled_to)
            .bind(self.schema_id)
            .bind(self.message_type_id)
            .bind(self.channel_id)
            .bind(self.recipient)
            .bind(self.include_dispatcher_types)
            .bind(self.exclude_dispatcher_types)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                UPDATE messages 
                SET status = $2
                WHERE id = $1
                RETURNING *"#,
        )
        .bind(self.id)
        .bind(self.status)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }
}
