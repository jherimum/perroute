use perroute_commons::types::{
    actor::{Actor, ActorType},
    id::Id,
};
use serde::Serialize;
use sqlx::PgExecutor;
use tap::TapFallible;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CommandLog {
    id: Id,
    command_type: String,
    payload: serde_json::Value,
    actor_type: ActorType,
    actor_id: Option<Id>,
    created_at: time::OffsetDateTime,
    error: Option<String>,
}

impl CommandLog {
    pub fn new<E>(
        command: impl Into<String>,
        payload: serde_json::Value,
        actor: &Actor,
        error: Option<E>,
    ) -> Self
    where
        E: std::fmt::Display,
    {
        Self {
            id: Id::new(),
            command_type: command.into(),
            payload,
            actor_type: *actor.ty(),
            actor_id: *actor.id(),
            created_at: time::OffsetDateTime::now_utc(),
            error: error.map(|e| format!("{e}")),
        }
    }

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
        .bind(self.error)
        .fetch_one(exec)
        .await
        .tap_err(|e| tracing::error!("Query error. {e}"))
    }
}
