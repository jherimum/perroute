use chrono::NaiveDateTime;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::ActorType, id::Id};
use serde::Serialize;
use sqlx::{FromRow, PgExecutor};
use tap::TapFallible;

use crate::log_query_error;

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Builder, Serialize)]
#[builder(setter(into))]
pub struct CommandLog {
    id: Id,
    command_type: String,
    payload: serde_json::Value,
    actor_type: ActorType,
    actor_id: Option<Id>,
    created_at: NaiveDateTime,
    error: Option<String>,
}

impl CommandLog {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                    INSERT INTO command_logs (id, command_type, payload, actor_type, actor_id, created_at, error ) 
                    VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *
                "#,
        )
        .bind(self.id)
        .bind(self.command_type)
        .bind(self.payload)
        .bind(self.actor_type)
        .bind(self.actor_id)
        .bind(self.created_at)
        .bind(self.error)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }
}
