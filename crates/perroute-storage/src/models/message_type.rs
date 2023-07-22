use super::schema::{Schema, SchemasQueryBuilder, Version};
use crate::{
    log_query_error,
    query::{FetchableModel, ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id};
use sqlx::{FromRow, PgExecutor, Postgres, QueryBuilder};
use tap::TapFallible;

impl DatabaseModel for MessageType {}

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

#[derive(Debug, Default, Builder)]
pub struct MessageTypeQuery {
    #[builder(default)]
    id: Option<Id>,
    #[builder(default)]
    code: Option<Code>,
    #[builder(default)]
    channel_id: Option<Id>,
}

impl ModelQueryBuilder<MessageType> for MessageTypeQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, Postgres> {
        let mut query_builder = projection.query_builder();

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
    pub async fn schema_by_version<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
        version: Version,
    ) -> Result<Option<Schema>, sqlx::Error> {
        Schema::find(
            exec,
            SchemasQueryBuilder::default()
                .version(Some(version))
                .message_type_id(Some(self.id))
                .build()
                .unwrap(),
        )
        .await
    }

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
}
