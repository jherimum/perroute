use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{
    dispatch_type::DispatchType, id::Id, recipient::Recipient, MessageStatus,
    Payload, Tags, Timestamp,
};
use sqlx::{prelude::FromRow, types::Json};

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Message {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    message_type_id: Id,
    #[setters(skip)]
    business_unit_id: Id,
    #[setters(skip)]
    payload: Payload,
    #[setters(skip)]
    dispatch_type: DispatchType,
    #[setters(skip)]
    recipient: Json<Recipient>,

    status: MessageStatus,

    #[setters(skip)]
    scheduled_at: Option<Timestamp>,

    #[setters(skip)]
    tags: Json<Tags>,

    #[setters(skip)]
    created_at: Timestamp,

    updated_at: Timestamp,
}

impl perroute_commons::types::entity::Entity for Message {
    fn id(&self) -> &Id {
        &self.id
    }
}
