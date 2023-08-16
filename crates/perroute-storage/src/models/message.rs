use std::collections::HashSet;

use crate::{log_query_error, query::ModelQueryBuilder, DatabaseModel};
use chrono::NaiveDateTime;
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, payload::Payload, recipient::Recipient};
use perroute_connectors::types::{DispatchType, DispatchTypes};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow, PgExecutor};
use tap::TapFallible;

use super::{business_unit::BusinessUnit, message_type::MessageType, schema::Schema};

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
    pub business_unit_id: Option<Id>,
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

        if let Some(business_unit_id) = self.business_unit_id {
            builder.push(" AND business_unit_id = ");
            builder.push_bind(business_unit_id);
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

    #[setters(skip)]
    recipient: Recipient,

    #[setters(skip)]
    #[builder(default)]
    dispatcher_types: DispatchTypes,

    status: Status,

    #[setters(skip)]
    schema_id: Id,

    #[setters(skip)]
    message_type_id: Id,

    #[setters(skip)]
    business_unit_id: Id,
}

impl Message {
    pub async fn message_type<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<MessageType, sqlx::Error> {
        todo!()
    }

    pub async fn business_unit<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<BusinessUnit, sqlx::Error> {
        todo!()
    }

    pub async fn schema<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Schema, sqlx::Error> {
        todo!()
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                INSERT INTO messages (id, payload, recipient, dispatcher_types, status, schema_id, message_type_id, business_unit_id) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *"#,
            ).bind(self.id)
            .bind(self.payload)
            .bind(self.recipient)
            .bind(self.dispatcher_types)            
            .bind(self.status)
            .bind(self.schema_id)
            .bind(self.message_type_id)
            .bind(self.business_unit_id)            
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
