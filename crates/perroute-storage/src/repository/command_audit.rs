use sqlx::query_as;

use super::{PgRepository, RepositoryResult};
use crate::{fetch_one, models::command_audit::CommandAudit};
use std::future::Future;

const INSERT_QUERY: &str = r#"
            INSERT INTO command_audit (id, command_type, command_data, actor_type, actor_id, created_at) 
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"#;

pub trait CommandAuditRepository {
    fn save(&self, command: CommandAudit) -> impl Future<Output = RepositoryResult<CommandAudit>>;
}

impl CommandAuditRepository for PgRepository {
    async fn save(&self, command: CommandAudit) -> RepositoryResult<CommandAudit> {
        let query = query_as(INSERT_QUERY)
            .bind(command.id())
            .bind(command.command_type())
            .bind(command.command_data())
            .bind(command.actor_type())
            .bind(command.actor_id())
            .bind(command.created_at());

        Ok(fetch_one!(&self.source, query)?)
    }
}
