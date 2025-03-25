use derive_getters::Getters;
use perroute_commons::types::{
    dispatch_type::DispatchType, entity::Entity, id::Id, priority::Priority,
    vars::Vars, Timestamp,
};
use sqlx::{prelude::FromRow, types::Json};

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters)]
pub struct TemplateAssignment {
    id: Id,
    business_unit_id: Id,
    message_type_id: Id,
    sms_template_id: Option<Id>,
    email_template_id: Option<Id>,
    push_template_id: Option<Id>,
    vars: Json<Vars>,
    priority: Priority,
    start_at: Timestamp,
    end_at: Option<Timestamp>,
    enabled: bool,
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl TemplateAssignment {
    pub fn template_id(&self, dispatch_type: &DispatchType) -> &Option<Id> {
        match dispatch_type {
            DispatchType::Email => self.email_template_id(),
            DispatchType::Sms => self.sms_template_id(),
            DispatchType::Push => self.push_template_id(),
        }
    }
}

impl Entity for TemplateAssignment {
    fn id(&self) -> &Id {
        &self.id
    }
}
