use bon::Builder;
use serde::Serialize;
use std::collections::HashMap;

use crate::{
    events::EventType,
    types::{actor::Actor, id::Id, Timestamp},
};

#[derive(Debug, Clone, Serialize)]
pub enum NewEvent {
    BusinessUnitCreated(EventData<()>),
    BusinessUnitUpdated(EventData<()>),
    BusinessUnitDeleted(EventData<()>),
    ChannelCreated(EventData<()>),
    ChannelUpdated(EventData<()>),
    ChannelDeleted(EventData<()>),
    MessageTypeCreated(EventData<()>),
    MessageTypeUpdated(EventData<()>),
    MessageTypeDeleted(EventData<()>),
    RouteCreated(EventData<()>),
    RouteUpdated(EventData<()>),
    RouteDeleted(EventData<()>),
}

impl NewEvent {
    pub fn id(&self) -> &Id {
        match self {
            NewEvent::BusinessUnitCreated(event) => &event.id,
            NewEvent::BusinessUnitUpdated(event) => &event.id,
            NewEvent::BusinessUnitDeleted(event) => &event.id,
            NewEvent::ChannelCreated(event) => &event.id,
            NewEvent::ChannelUpdated(event) => &event.id,
            NewEvent::ChannelDeleted(event) => &event.id,
            NewEvent::MessageTypeCreated(event) => &event.id,
            NewEvent::MessageTypeUpdated(event) => &event.id,
            NewEvent::MessageTypeDeleted(event) => &event.id,
            NewEvent::RouteCreated(event) => &event.id,
            NewEvent::RouteUpdated(event) => &event.id,
            NewEvent::RouteDeleted(event) => &event.id,
        }
    }

    pub fn entity_id(&self) -> &Id {
        match self {
            NewEvent::BusinessUnitCreated(event) => &event.entity_id,
            NewEvent::BusinessUnitUpdated(event) => &event.entity_id,
            NewEvent::BusinessUnitDeleted(event) => &event.entity_id,
            NewEvent::ChannelCreated(event) => &event.entity_id,
            NewEvent::ChannelUpdated(event) => &event.entity_id,
            NewEvent::ChannelDeleted(event) => &event.entity_id,
            NewEvent::MessageTypeCreated(event) => &event.entity_id,
            NewEvent::MessageTypeUpdated(event) => &event.entity_id,
            NewEvent::MessageTypeDeleted(event) => &event.entity_id,
            NewEvent::RouteCreated(event) => &event.entity_id,
            NewEvent::RouteUpdated(event) => &event.entity_id,
            NewEvent::RouteDeleted(event) => &event.entity_id,
        }
    }
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct EventData<P: Serialize> {
    pub id: Id,
    pub event_type: EventType,
    pub entity_id: Id,
    pub payload: P,
    pub created_at: Timestamp,
    actor: Actor,
}
