use crate::{
    impl_sqlx_type,
    types::{actor::Actor, id::Id, Timestamp},
};
use bon::{builder, Builder};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, vec};

#[derive(
    Debug, Clone, PartialEq, Eq, strum::EnumString, strum::Display, Deserialize, Serialize, Hash,
)]
pub enum EventType {
    BusinessUnitCreated,
    BusinessUnitUpdated,
    BusinessUnitDeleted,
    ChannelCreated,
    ChannelUpdated,
    ChannelDeleted,
    MessageTypeCreated,
    MessageTypeUpdated,
    MessageTypeDeleted,
    RouteCreated,
    RouteUpdated,
    RouteDeleted,
}

impl_sqlx_type!(EventType as String);

impl EventType {
    //TODO: Implement a better error handling
    pub fn parse(str: &str) -> Result<HashSet<Self>, Box<dyn std::error::Error>> {
        Ok(HashSet::from_iter(vec![EventType::BusinessUnitCreated]))
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Event {
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

impl Event {
    pub fn id(&self) -> &Id {
        match self {
            Event::BusinessUnitCreated(event) => &event.id,
            Event::BusinessUnitUpdated(event) => &event.id,
            Event::BusinessUnitDeleted(event) => &event.id,
            Event::ChannelCreated(event) => &event.id,
            Event::ChannelUpdated(event) => &event.id,
            Event::ChannelDeleted(event) => &event.id,
            Event::MessageTypeCreated(event) => &event.id,
            Event::MessageTypeUpdated(event) => &event.id,
            Event::MessageTypeDeleted(event) => &event.id,
            Event::RouteCreated(event) => &event.id,
            Event::RouteUpdated(event) => &event.id,
            Event::RouteDeleted(event) => &event.id,
        }
    }

    pub fn entity_id(&self) -> &Id {
        match self {
            Event::BusinessUnitCreated(event) => &event.entity_id,
            Event::BusinessUnitUpdated(event) => &event.entity_id,
            Event::BusinessUnitDeleted(event) => &event.entity_id,
            Event::ChannelCreated(event) => &event.entity_id,
            Event::ChannelUpdated(event) => &event.entity_id,
            Event::ChannelDeleted(event) => &event.entity_id,
            Event::MessageTypeCreated(event) => &event.entity_id,
            Event::MessageTypeUpdated(event) => &event.entity_id,
            Event::MessageTypeDeleted(event) => &event.entity_id,
            Event::RouteCreated(event) => &event.entity_id,
            Event::RouteUpdated(event) => &event.entity_id,
            Event::RouteDeleted(event) => &event.entity_id,
        }
    }
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct EventData<P: Serialize> {
    #[builder(default)]
    pub id: Id,
    pub event_type: EventType,
    pub entity_id: Id,
    pub payload: P,
    pub created_at: Timestamp,
    pub actor: Actor,
}
