use crate::{
    models::dispatcher_log::DispatcherLog,
    repository::{dispatcher_log::DispatcherLogRepository, RepositoryResult},
};

use super::PgRepository;

#[async_trait::async_trait]
impl DispatcherLogRepository for PgRepository {
    async fn save_all_dispatch_logs(
        &self,
        logs: Vec<DispatcherLog>,
    ) -> RepositoryResult<Vec<DispatcherLog>> {
        todo!()
    }
}
