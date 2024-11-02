use std::future::Future;

use crate::models::event::Event;

use super::{PgRepository, RepositoryResult};

pub trait EventRepository {
    fn save(&self, event: Event) -> impl Future<Output = RepositoryResult<Event>>;
}

impl EventRepository for PgRepository {
    async fn save(&self, event: Event) -> RepositoryResult<Event> {
        todo!()
    }
}
