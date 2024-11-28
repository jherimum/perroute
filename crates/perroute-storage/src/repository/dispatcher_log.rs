use super::{PgRepository, RepositoryResult};
use crate::models::dispatcher_log::DispatcherLog;
use std::future::Future;

pub trait DispatcherLogRepository {
    fn save_all(
        &self,
        logs: Vec<DispatcherLog>,
    ) -> impl Future<Output = RepositoryResult<Vec<DispatcherLog>>>;
}

impl DispatcherLogRepository for PgRepository {
    async fn save_all(&self, logs: Vec<DispatcherLog>) -> RepositoryResult<Vec<DispatcherLog>> {
        todo!()
    }
}
