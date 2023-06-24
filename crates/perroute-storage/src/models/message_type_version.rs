use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::id::Id;
use sqlx::{FromRow, PgExecutor};

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct MessageTypeVersion {
    #[setters(skip)]
    id: Id,
    schema: String,
    #[setters(skip)]
    version: i32,
    published: bool,
    #[setters(skip)]
    message_type_id: Id,
    #[setters(skip)]
    channel_id: Id,
}

impl MessageTypeVersion {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                INSERT INTO message_type_versions (id, schema, version, published, message_type_id, channel_id) 
                VALUES($1, $2, $3, $4, $5, $6) RETURNING *
            "#)
            .bind(self.id)
            .bind(self.schema)
            .bind(self.version)
            .bind(self.published)
            .bind(self.message_type_id)
            .bind(self.channel_id)
            .fetch_one(exec)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE message_type_versions 
            SET 
                schema= $1, 
                published= $2
            WHERE id= $3 RETURNING *
            "#,
        )
        .bind(self.schema)
        .bind(self.published)
        .bind(self.id)
        .fetch_one(exec)
        .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        sqlx::query("DELETE FROM message_type_versions WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .map(|r| r.rows_affected() > 0)
    }

    pub async fn find_by_id<'e, E: PgExecutor<'e>>(
        exec: E,
        id: &Id,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT * 
            FROM message_type_versions 
            WHERE id= $1
            "#,
        )
        .bind(id)
        .fetch_optional(exec)
        .await
    }

    pub async fn find_by_channel_id_and_message_type_id_and_id<'e, E: PgExecutor<'e>>(
        exec: E,
        channel_id: &Id,
        message_type_id: &Id,
        id: &Id,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT * 
            FROM message_type_versions 
            WHERE 
                channel_id= $1 
                AND message_type_id= $2 
                AND id= $3
            "#,
        )
        .bind(channel_id)
        .bind(message_type_id)
        .bind(id)
        .fetch_optional(exec)
        .await
    }
}
