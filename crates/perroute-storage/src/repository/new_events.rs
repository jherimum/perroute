use super::{PgRepository, RepositoryResult};
use crate::{execute, fetch_all, models::new_event::NewDbEvent};
use perroute_commons::types::{id::Id, Timestamp};
use sqlx::{query, query_as};

#[async_trait::async_trait]
pub trait NewEventRepository {
    async fn set_consumed(&self, events: &[Id], timestamp: Timestamp) -> RepositoryResult<()>;

    async fn unconsumed(&self, size: usize) -> RepositoryResult<Vec<NewDbEvent>>;
}

#[async_trait::async_trait]
impl NewEventRepository for PgRepository {
    async fn set_consumed(&self, events: &[Id], timestamp: Timestamp) -> RepositoryResult<()> {
        let query = query("UPDATE event_messages SET consumed_at = $1 WHERE id = ANY($2)")
            .bind(timestamp)
            .bind(events);

        let _ = execute!(&self.source, query)?;

        Ok(())
    }

    async fn unconsumed(&self, size: usize) -> RepositoryResult<Vec<NewDbEvent>> {
        Ok(fetch_all!(
            &self.source,
            query_as("SELECT * from event_messages where consumed_at is null ORDER BY created_at asc limit $1").bind(size as i64)
        )?)
    }
}
