use crate::{impl_sqlx_type, types::id::Id};
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

impl EventType {
    //TODO: Implement a better error handling
    pub fn parse(str: &str) -> Result<HashSet<Self>, Box<dyn std::error::Error>> {
        Ok(HashSet::from_iter(vec![EventType::BusinessUnitCreated]))
    }
}

impl_sqlx_type!(EventType as String);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Event {
    BusinessUnitCreated(Id),
    BusinessUnitUpdated(Id),
    BusinessUnitDeleted(Id),
    ChannelCreated(Id),
    ChannelUpdated(Id),
    ChannelDeleted(Id),
    MessageTypeCreated(Id),
    MessageTypeUpdated(Id),
    MessageTypeDeleted(Id),
    RouteCreated(Id),
    RouteUpdated(Id),
    RouteDeleted(Id),
}

impl Event {
    pub fn entity_id(&self) -> &Id {
        match self {
            Event::BusinessUnitCreated(id) => id,
            Event::BusinessUnitUpdated(id) => id,
            Event::BusinessUnitDeleted(id) => id,
            Event::ChannelCreated(id) => id,
            Event::ChannelUpdated(id) => id,
            Event::ChannelDeleted(id) => id,
            Event::MessageTypeCreated(id) => id,
            Event::MessageTypeUpdated(id) => id,
            Event::MessageTypeDeleted(id) => id,
            Event::RouteCreated(id) => id,
            Event::RouteUpdated(id) => id,
            Event::RouteDeleted(id) => id,
        }
    }

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
}
