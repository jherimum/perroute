use crate::{
    log_query_error,
    query::{ModelQuery, ModelQueryFetch, Projection},
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id};
use sqlx::{FromRow, PgExecutor, Postgres, QueryBuilder};
use tap::TapFallible;

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct MessageType {
    #[setters(skip)]
    id: Id,

    #[setters(skip)]
    code: Code,

    description: String,
    enabled: bool,

    #[setters(skip)]
    channel_id: Id,
}

#[derive(Debug, Default)]
pub struct MessageTypeQuery {
    id: Option<Id>,
    code: Option<Code>,
    channel_id: Option<Id>,
}

impl MessageTypeQuery {
    pub fn by_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn by_channel_and_code(channel_id: Id, code: Code) -> Self {
        Self {
            channel_id: Some(channel_id),
            code: Some(code),
            ..Default::default()
        }
    }

    pub fn all_by_channel(channel_id: Id) -> Self {
        Self {
            channel_id: Some(channel_id),
            ..Default::default()
        }
    }

    pub fn by_channel_and_id(channel_id: Id, id: Id) -> Self {
        Self {
            channel_id: Some(channel_id),
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn by_id_and_maybe_channel(id: Id, channel_id: Option<Id>) -> Self {
        Self {
            channel_id,
            id: Some(id),
            ..Default::default()
        }
    }
}

impl ModelQuery<MessageType> for MessageTypeQuery {
    fn query_builder(&self, projection: Projection) -> QueryBuilder<'_, Postgres> {
        let mut query_builder = QueryBuilder::new({
            match projection {
                Projection::Row => "SELECT *",
                Projection::Count => "SELECT COUNT(*)",
                Projection::Id => "SELECT id",
            }
        });

        query_builder.push(" FROM message_types WHERE 1=1");

        if let Some(code) = &self.code {
            query_builder.push(" and code = ");
            query_builder.push_bind(code);
        }

        if let Some(channel_id) = &self.channel_id {
            query_builder.push(" and channel_id = ");
            query_builder.push_bind(channel_id);
        }

        if let Some(id) = &self.id {
            query_builder.push(" and id = ");
            query_builder.push_bind(id);
        }

        query_builder
    }
}

impl MessageType {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                    INSERT INTO message_types (id, code, description, enabled, channel_id) 
                    VALUES($1, $2, $3, $4, $5) RETURNING *
                "#,
        )
        .bind(self.id)
        .bind(self.code)
        .bind(self.description)
        .bind(self.enabled)
        .bind(self.channel_id)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                    UPDATE message_types 
                    SET description= $1, enabled= $2 WHERE id= $3 RETURNING *
                "#,
        )
        .bind(self.description)
        .bind(self.enabled)
        .bind(self.id)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        sqlx::query("DELETE FROM message_types WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected() > 0)
    }

    pub async fn find_one<'e, E: PgExecutor<'e>>(
        exec: E,
        id: Id,
        channel_id: Option<Id>,
    ) -> Result<Option<Self>, sqlx::Error> {
        MessageTypeQuery::by_id_and_maybe_channel(id, channel_id)
            .one(exec)
            .await
    }

    pub async fn exists_code<'e, E: PgExecutor<'e>>(
        exec: E,
        channel_id: Id,
        code: Code,
    ) -> Result<bool, sqlx::Error> {
        MessageTypeQuery::by_channel_and_code(channel_id, code)
            .count(exec)
            .await
            .map(|count| count > 0)
    }

    pub async fn find_by_channel_id_and_message_type_id<'e, E: PgExecutor<'e>>(
        exec: E,
        channel_id: Id,
        message_type_id: Id,
    ) -> Result<Option<Self>, sqlx::Error> {
        MessageTypeQuery::by_channel_and_id(channel_id, message_type_id)
            .one(exec)
            .await
    }

    pub async fn find_by_channel<'e, E: PgExecutor<'e>>(
        exec: E,
        channel_id: Id,
    ) -> Result<Vec<Self>, sqlx::Error> {
        MessageTypeQuery::all_by_channel(channel_id)
            .many(exec)
            .await
    }
}
