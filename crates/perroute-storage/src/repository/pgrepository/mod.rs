pub mod business_unit;
pub mod dispatcher_log;
pub mod message;
pub mod message_type;
pub mod route;
pub mod template_assignment;

use sqlx::Database;

use super::{Repository, RepositoryResult, TransactionalRepository};

#[derive(Clone)]
pub struct PgRepository;

#[async_trait::async_trait]
impl Repository for PgRepository {
    type Tx = PgRepository;

    async fn begin_transaction(&self) -> RepositoryResult<PgRepository> {
        todo!()
    }
}

#[async_trait::async_trait]
impl TransactionalRepository for PgRepository {
    async fn commit(self) -> RepositoryResult<()> {
        todo!()
    }

    async fn rollback(self) -> RepositoryResult<()> {
        todo!()
    }
}
