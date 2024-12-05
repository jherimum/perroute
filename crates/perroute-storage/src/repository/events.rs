use super::{PgRepository, RepositoryResult};
use crate::{execute, fetch_all, fetch_one, models::event::DbEvent};
use perroute_commons::types::{id::Id, Timestamp};
use sqlx::{query, query_as};

#[async_trait::async_trait]
pub trait EventRepository {
    async fn set_consumed(
        &self,
        events: &[Id],
        skipped: bool,
        timestamp: Timestamp,
    ) -> RepositoryResult<()>;

    async fn unconsumed(&self, size: usize) -> RepositoryResult<Vec<DbEvent>>;

    async fn save(&self, event: DbEvent) -> RepositoryResult<DbEvent>;
}

#[async_trait::async_trait]
impl EventRepository for PgRepository {
    async fn save(&self, event: DbEvent) -> RepositoryResult<DbEvent> {
        let query = query_as(
            "INSERT INTO event_messages
            (id, event_type, entity_id, payload, actor_type, actor_id, created_at)
            VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
        .bind(event.id())
        .bind(event.event_type())
        .bind(event.entity_id())
        .bind(event.payload())
        .bind(event.actor_type())
        .bind(event.actor_id())
        .bind(event.created_at());
        Ok(fetch_one!(&self.source, query)?)
    }

    async fn set_consumed(
        &self,
        events: &[Id],
        skipped: bool,
        timestamp: Timestamp,
    ) -> RepositoryResult<()> {
        let query = query("UPDATE event_messages SET consumed_at = $1 WHERE id = ANY($2)")
            .bind(timestamp)
            .bind(events);

        let _ = execute!(&self.source, query)?;

        Ok(())
    }

    async fn unconsumed(&self, size: usize) -> RepositoryResult<Vec<DbEvent>> {
        Ok(fetch_all!(
            &self.source,
            query_as("SELECT * from event_messages where consumed_at is null ORDER BY created_at asc limit $1").bind(size as i64)
        )?)
    }
}
