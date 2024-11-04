use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::{
    events::{Event, EventType},
    types::{id::Id, Timestamp},
};
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

impl perroute_commons::types::entity::Entity for DbEvent {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl From<&Event> for DbEvent {
    fn from(event: &Event) -> Self {
        Self {
            id: Id::new(),
            event_type: event.event_type(),
            entity_id: event.entity_id().clone(),
            created_at: Timestamp::now(),
            consumed_at: None,
        }
    }
}

impl From<&DbEvent> for Event {
    fn from(db_event: &DbEvent) -> Self {
        match db_event.event_type() {
            EventType::BusinessUnitCreated => {
                Event::BusinessUnitCreated(db_event.entity_id().clone())
            }
            EventType::BusinessUnitDeleted => {
                Event::BusinessUnitDeleted(db_event.entity_id().clone())
            }
            EventType::BusinessUnitUpdated => {
                Event::BusinessUnitUpdated(db_event.entity_id().clone())
            }
            EventType::ChannelCreated => Event::ChannelCreated(db_event.entity_id().clone()),
            EventType::ChannelDeleted => Event::ChannelDeleted(db_event.entity_id().clone()),
            EventType::ChannelUpdated => Event::ChannelUpdated(db_event.entity_id().clone()),
            EventType::MessageTypeCreated => {
                Event::MessageTypeCreated(db_event.entity_id().clone())
            }
            EventType::MessageTypeUpdated => {
                Event::MessageTypeUpdated(db_event.entity_id().clone())
            }
            EventType::MessageTypeDeleted => {
                Event::MessageTypeDeleted(db_event.entity_id().clone())
            }
            EventType::RouteCreated => Event::RouteCreated(db_event.entity_id().clone()),
            EventType::RouteDeleted => Event::RouteDeleted(db_event.entity_id().clone()),
            EventType::RouteUpdated => Event::RouteUpdated(db_event.entity_id().clone()),
        }
    }
}
