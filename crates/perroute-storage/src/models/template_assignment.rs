use chrono::NaiveDateTime;
use perroute_commons::types::{id::Id, priority::Priority, vars::Vars};
use perroute_connectors::types::dispatch_type::DispatchType;

pub struct TemplateAssignment {
    id: Id,
    template_id: Id,
    business_unit_id: Id,
    message_type_id: Id,
    dispatch_type: DispatchType,
    priority: Priority,
    vars: Vars,
    start_at: NaiveDateTime,
    end_at: Option<NaiveDateTime>,
    enabled: bool,
}
