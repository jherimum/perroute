use sqlx::query_as;
use std::future::Future;

use super::{PgRepository, RepositoryResult};
use crate::{fetch_one, models::event::DbEvent};

const INSERT_QUERY: &str = r#"
            INSERT INTO event_messages (id, event_type, entity_id, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#;
pub trait EventRepository {
    fn save(&self, event: DbEvent) -> impl Future<Output = RepositoryResult<DbEvent>>;
}

impl EventRepository for PgRepository {
    async fn save(&self, event: DbEvent) -> RepositoryResult<DbEvent> {
        let query = query_as(INSERT_QUERY)
            .bind(event.id())
            .bind(event.event_type())
            .bind(event.entity_id())
            .bind(event.created_at());

        Ok(fetch_one!(&self.source, query)?)
    }
}
