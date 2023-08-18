use crate::{
    query::{FetchableModel, ModelQueryBuilder},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{id::Id, properties::Properties};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{types::Json, FromRow, PgExecutor, QueryBuilder, Type};

use super::{
    message::{Message, MessageQueryBuilder},
    route::{Route, RouteQueryBuilder},
    template::{Template, TemplatesQueryBuilder},
};

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type)]
pub struct MessageDispatchResult {
    pub reference: Option<String>,
    pub connection_properties: Json<Properties>,
    pub dispatcher_properties: Json<Properties>,
    pub response_data: Option<Json<Value>>,
}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Builder)]
#[builder(setter(into))]
pub struct MessageDispatch {
    id: Id,
    status: MessageDispatchStatus,
    result: Option<MessageDispatchResult>,
    template_id: Id,
    message_id: Id,
    route_id: Id,
}

impl MessageDispatch {
    pub fn commit(mut self, success: bool, result: impl Into<MessageDispatchResult>) -> Self {
        self.status = {
            if success {
                MessageDispatchStatus::Success
            } else {
                MessageDispatchStatus::Failed
            }
        };
        self.result = Some(result.into());
        self
    }

    pub async fn route<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Route, sqlx::Error> {
        Route::find_one(
            exec,
            RouteQueryBuilder::default()
                .id(Some(self.route_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn template<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Template, sqlx::Error> {
        Template::find_one(
            exec,
            TemplatesQueryBuilder::default()
                .id(Some(self.template_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn message<'e, E: PgExecutor<'e>>(
        &self,
        executor: E,
    ) -> Result<Message, sqlx::Error> {
        Message::find_one(
            executor,
            MessageQueryBuilder::default()
                .id(Some(self.message_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn save<'e, E: PgExecutor<'e>>(&self, executor: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO message_dispatches (id, message_id, route_id, status, template_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.message_id)
        .bind(self.route_id)
        .bind(self.status)
        .bind(self.template_id)
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
