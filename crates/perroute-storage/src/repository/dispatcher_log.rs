use crate::models::dispatcher_log::DispatcherLog;

use super::RepositoryResult;

#[async_trait::async_trait]
pub trait DispatcherLogRepository {
    async fn save_all_dispatch_logs(
        &self,
        logs: Vec<DispatcherLog>,
    ) -> RepositoryResult<Vec<DispatcherLog>>;
}
