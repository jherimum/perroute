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
    pub fn event_type(&self) -> EventType {
        match self {
            Event::BusinessUnitCreated(_) => EventType::BusinessUnitCreated,
            Event::BusinessUnitUpdated(_) => EventType::BusinessUnitUpdated,
            Event::BusinessUnitDeleted(_) => EventType::BusinessUnitDeleted,
            Event::ChannelCreated(_) => EventType::ChannelCreated,
            Event::ChannelUpdated(_) => EventType::ChannelUpdated,
            Event::ChannelDeleted(_) => EventType::ChannelDeleted,
            Event::MessageTypeCreated(_) => EventType::MessageTypeCreated,
            Event::MessageTypeUpdated(_) => EventType::MessageTypeUpdated,
            Event::MessageTypeDeleted(_) => EventType::MessageTypeDeleted,
            Event::RouteCreated(_) => EventType::RouteCreated,
            Event::RouteUpdated(_) => EventType::RouteUpdated,
            Event::RouteDeleted(_) => EventType::RouteDeleted,
        }
    }

    pub fn created_at(&self) -> &Timestamp {
        match self {
            Event::BusinessUnitCreated(event) => &event.created_at,
            Event::BusinessUnitUpdated(event) => &event.created_at,
            Event::BusinessUnitDeleted(event) => &event.created_at,
            Event::ChannelCreated(event) => &event.created_at,
            Event::ChannelUpdated(event) => &event.created_at,
            Event::ChannelDeleted(event) => &event.created_at,
            Event::MessageTypeCreated(event) => &event.created_at,
            Event::MessageTypeUpdated(event) => &event.created_at,
            Event::MessageTypeDeleted(event) => &event.created_at,
            Event::RouteCreated(event) => &event.created_at,
            Event::RouteUpdated(event) => &event.created_at,
            Event::RouteDeleted(event) => &event.created_at,
        }
    }

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

    pub fn actor(&self) -> &Actor {
        match self {
            Event::BusinessUnitCreated(event) => &event.actor,
            Event::BusinessUnitUpdated(event) => &event.actor,
            Event::BusinessUnitDeleted(event) => &event.actor,
            Event::ChannelCreated(event) => &event.actor,
            Event::ChannelUpdated(event) => &event.actor,
            Event::ChannelDeleted(event) => &event.actor,
            Event::MessageTypeCreated(event) => &event.actor,
            Event::MessageTypeUpdated(event) => &event.actor,
            Event::MessageTypeDeleted(event) => &event.actor,
            Event::RouteCreated(event) => &event.actor,
            Event::RouteUpdated(event) => &event.actor,
            Event::RouteDeleted(event) => &event.actor,
        }
    }

    pub fn payload(&self) -> &impl Serialize {
        match self {
            Event::BusinessUnitCreated(event) => &event.payload,
            Event::BusinessUnitUpdated(event) => &event.payload,
            Event::BusinessUnitDeleted(event) => &event.payload,
            Event::ChannelCreated(event) => &event.payload,
            Event::ChannelUpdated(event) => &event.payload,
            Event::ChannelDeleted(event) => &event.payload,
            Event::MessageTypeCreated(event) => &event.payload,
            Event::MessageTypeUpdated(event) => &event.payload,
            Event::MessageTypeDeleted(event) => &event.payload,
            Event::RouteCreated(event) => &event.payload,
            Event::RouteUpdated(event) => &event.payload,
            Event::RouteDeleted(event) => &event.payload,
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
