use super::{PgRepository, RepositoryResult};
use crate::models::dispatcher_log::DispatcherLog;

#[async_trait::async_trait]
pub trait DispatcherLogRepository {
    async fn save_all(&self, logs: Vec<DispatcherLog>) -> RepositoryResult<Vec<DispatcherLog>>;
}

#[async_trait::async_trait]
impl DispatcherLogRepository for PgRepository {
    async fn save_all(&self, logs: Vec<DispatcherLog>) -> RepositoryResult<Vec<DispatcherLog>> {
        todo!()
    }
}
