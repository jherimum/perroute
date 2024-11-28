use super::{PgRepository, RepositoryResult};
use crate::execute;
use crate::{fetch_all, fetch_one, models::event::DbEvent};
use perroute_commons::types::id::Id;
use perroute_commons::types::Timestamp;
use sqlx::{query, query_as};

pub enum EventQuery {
    Unconsumed,
}

const INSERT_QUERY: &str = r#"
            INSERT INTO event_messages (id, event_type, entity_id, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#;

#[async_trait::async_trait]
pub trait EventRepository {
    async fn set_consumed(&self, events: &[Id], timestamp: Timestamp) -> RepositoryResult<()>;

    async fn save(&self, event: DbEvent) -> RepositoryResult<DbEvent>;
    async fn update(&self, events: DbEvent) -> RepositoryResult<DbEvent>;
    async fn query(&self, query: &EventQuery, size: usize) -> RepositoryResult<Vec<DbEvent>>;
}

#[async_trait::async_trait]
impl EventRepository for PgRepository {
    async fn set_consumed(&self, events: &[Id], timestamp: Timestamp) -> RepositoryResult<()> {
        let query = query("UPDATE event_messages SET consumed_at = $1 WHERE id = ANY($2)")
            .bind(timestamp)
            .bind(events);

        let _ = execute!(&self.source, query)?;

        Ok(())
    }

    async fn update(&self, events: DbEvent) -> RepositoryResult<DbEvent> {
        Ok(fetch_one!(
            &self.source,
            query_as("UPDATE event_messages SET consumed_at = $1 WHERE id = $2 RETURNING *")
                .bind(events.consumed_at())
                .bind(events.id())
        )?)
    }
    async fn query(&self, query: &EventQuery, size: usize) -> RepositoryResult<Vec<DbEvent>> {
        Ok(fetch_all!(
            &self.source,
            query_as("SELECT * from event_messages where consumed_at is null limit $1")
                .bind(size as i64)
        )?)
    }

    async fn save(&self, event: DbEvent) -> RepositoryResult<DbEvent> {
        let query = query_as(INSERT_QUERY)
            .bind(event.id())
            .bind(event.event_type())
            .bind(event.entity_id())
            .bind(event.created_at());

        Ok(fetch_one!(&self.source, query)?)
    }
}
