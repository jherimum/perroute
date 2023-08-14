use chrono::Utc;
use derive_getters::Getters;
use perroute_commons::types::id::Id;
use perroute_storage::models::db_event::{DbEvent, DbEventBuilder, DbEventBuilderError};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

pub trait IntoEvent {
    fn into_event(&self) -> Option<Event>;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Display)]
pub enum EventType {
    BusinessUnitCreated,
    MessageCreated,
    MessageDistributed,
}

#[derive(Debug, Getters)]
pub struct Event {
    entity_id: Id,
    ty: EventType,
}

impl Event {
    pub fn new(entity_id: Id, ty: EventType) -> Self {
        Self { entity_id, ty }
    }
}

impl TryFrom<Event> for DbEvent {
    type Error = DbEventBuilderError;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        DbEventBuilder::default()
            .id(Id::new())
            .entity_id(value.entity_id)
            .event_type(value.ty().to_string())
            .created_at(Utc::now().naive_utc())
            .scheduled_to(Utc::now().naive_utc())
            .build()
    }
}
