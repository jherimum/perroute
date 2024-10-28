use perroute_commons::types::{
    id::Id, DispatchType, MessageStatus, Payload, Recipient, Tags, Timestamp,
};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Message {
    pub id: Id,
    pub message_type_id: Id,
    pub business_unit_id: Id,
    pub payload: Payload,
    pub dispatch_type: DispatchType,
    pub recipient: Recipient,
    pub status: MessageStatus,
    pub scheduled_at: Option<Timestamp>,
    pub tags: Tags,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
