use crate::{
    impl_sqlx_type,
    types::{actor::Actor, id::Id, Timestamp},
};
use bon::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

impl From<&Self> for EventType {
    fn from(event_type: &EventType) -> Self {
        event_type.clone()
    }
}

impl_sqlx_type!(EventType as String);

impl EventType {
    //TODO: Implement a better error handling
    pub fn parse(str: &str) -> Result<HashSet<Self>, Box<dyn std::error::Error>> {
        Ok(HashSet::from_iter(vec![EventType::BusinessUnitCreated]))
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Event {
    BusinessUnitCreated(EventData),
    BusinessUnitUpdated(EventData),
    BusinessUnitDeleted(EventData),
    ChannelCreated(EventData),
    ChannelUpdated(EventData),
    ChannelDeleted(EventData),
    MessageTypeCreated(EventData),
    MessageTypeUpdated(EventData),
    MessageTypeDeleted(EventData),
    RouteCreated(EventData),
    RouteUpdated(EventData),
    RouteDeleted(EventData),
}

impl AsRef<EventData> for Event {
    fn as_ref(&self) -> &EventData {
        match self {
            Event::BusinessUnitCreated(event) => event,
            Event::BusinessUnitUpdated(event) => event,
            Event::BusinessUnitDeleted(event) => event,
            Event::ChannelCreated(event) => event,
            Event::ChannelUpdated(event) => event,
            Event::ChannelDeleted(event) => event,
            Event::MessageTypeCreated(event) => event,
            Event::MessageTypeUpdated(event) => event,
            Event::MessageTypeDeleted(event) => event,
            Event::RouteCreated(event) => event,
            Event::RouteUpdated(event) => event,
            Event::RouteDeleted(event) => event,
        }
    }
}

#[derive(Debug, Clone, Builder, Serialize, Getters)]
pub struct EventData {
    pub id: Id,
    pub event_type: EventType,
    pub entity_id: Id,
    pub payload: Value,
    pub created_at: Timestamp,
    pub actor: Actor,
}
