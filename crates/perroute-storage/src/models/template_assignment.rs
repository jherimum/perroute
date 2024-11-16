use derive_getters::Getters;
use perroute_commons::types::{id::Id, priority::Priority, vars::Vars, Timestamp};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters)]
pub struct TemplateAssignment {
    id: Id,
    business_unit_id: Id,
    message_type_id: Id,
    sms_template_id: Option<Id>,
    email_template_id: Option<Id>,
    push_template_id: Option<Id>,
    vars: Vars,
    priority: Priority,
    start_at: Timestamp,
    end_at: Option<Timestamp>,
    enabled: bool,
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl perroute_commons::types::entity::Entity for TemplateAssignment {
    fn id(&self) -> &Id {
        &self.id
    }
}
