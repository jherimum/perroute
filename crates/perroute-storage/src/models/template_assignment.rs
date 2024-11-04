use perroute_commons::types::{id::Id, priority::Priority, vars::Vars, Timestamp};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct TemplateAssignment {
    pub id: Id,
    pub business_unit_id: Id,
    pub message_type_id: Id,
    pub vars: Vars,
    pub priority: Priority,
    pub start_at: Timestamp,
    pub end_at: Option<Timestamp>,
    pub enabled: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl perroute_commons::types::entity::Entity for TemplateAssignment {
    fn id(&self) -> &Id {
        &self.id
    }
}
