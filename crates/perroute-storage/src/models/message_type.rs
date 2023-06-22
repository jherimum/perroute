use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id};
use sqlx::{FromRow, PgExecutor};
use tap::TapFallible;

use crate::log_query_error;

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
}
