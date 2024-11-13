use derive_getters::Getters;
use perroute_commons::types::{
    dispatch_type::DispatchType, id::Id, recipient::Recipient, MessageStatus, Payload, Tags,
    Timestamp,
};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters)]
pub struct Message {
    id: Id,
    message_type_id: Id,
    business_unit_id: Id,
    payload: Payload,
    dispatch_type: DispatchType,
    recipient: Recipient,
    status: MessageStatus,
    scheduled_at: Option<Timestamp>,
    tags: Tags,

    created_at: Timestamp,
    updated_at: Timestamp,
}

impl perroute_commons::types::entity::Entity for Message {
    fn id(&self) -> &Id {
        &self.id
    }
}
