use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, Timestamp};
use perroute_events::{Event, EventType};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct DbEvent {
    id: Id,
    event_type: EventType,
    entity_id: Id,
    created_at: Timestamp,
    consumed_at: Option<Timestamp>,
}

impl From<Event> for DbEvent {
    fn from(event: Event) -> Self {
        Self {
            id: Id::new(),
            event_type: event.event_type(),
            entity_id: event.entity_id().clone(),
            created_at: Timestamp::now(),
            consumed_at: None,
        }
    }
}
