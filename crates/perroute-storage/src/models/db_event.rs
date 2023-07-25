use chrono::NaiveDateTime;
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::id::Id;
use sqlx::{FromRow, PgExecutor};

use crate::query;

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct DbEvent {
    id: Id,
    entity_id: Id,
    event_type: String,
    created_at: NaiveDateTime,
    scheduled_to: NaiveDateTime,
    #[builder(default)]
    consumed_at: Option<NaiveDateTime>,
}

impl DbEvent {
    pub async fn all<'e, E: PgExecutor<'e>>(exec: E) -> Result<Vec<DbEvent>, sqlx::Error> {
        sqlx::query_as("select * from events").fetch_all(exec).await
    }

    pub async fn save<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<DbEvent, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO events (id, entity_id, event_type, created_at, scheduled_to)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.entity_id)
        .bind(self.event_type.clone())
        .bind(self.created_at)
        .bind(self.scheduled_to)
        .fetch_one(exec)
        .await
    }
}
