use std::future::Future;

use crate::models::event::DbEvent;

use super::{PgRepository, RepositoryResult};

pub trait EventRepository {
    fn save(&self, event: DbEvent) -> impl Future<Output = RepositoryResult<DbEvent>>;
}

impl EventRepository for PgRepository {
    async fn save(&self, event: DbEvent) -> RepositoryResult<DbEvent> {
        todo!()
    }
}
