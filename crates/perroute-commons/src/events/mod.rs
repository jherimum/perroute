use crate::{
    event, impl_sqlx_type,
    types::{actor::Actor, code::Code, id::Id, name::Name, vars::Vars, Timestamp},
};
use bon::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, vec};
use strum::EnumString;

impl_sqlx_type!(EventType as String);

#[derive(Debug, Clone, PartialEq, Eq, EnumString, strum::Display, Deserialize, Serialize, Hash)]
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
    TemplateAssignmentCreated,
    TemplateAssignmentUpdated,
    TemplateAssignmentDeleted,
    MessageCreated,
}

impl From<&Self> for EventType {
    fn from(event_type: &EventType) -> Self {
        event_type.clone()
    }
}

impl EventType {
    //TODO: Implement a better error handling
    pub fn parse(str: &str) -> Result<HashSet<Self>, Box<dyn std::error::Error>> {
        Ok(HashSet::from_iter(vec![EventType::BusinessUnitCreated]))
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Event {
    BusinessUnitCreated(EventData<BusinessUnitCreatedEvent>),
    BusinessUnitUpdated(EventData<BusinessUnitUpdatedEvent>),
    BusinessUnitDeleted(EventData<BusinessUnitDeletedEvent>),
    ChannelCreated(EventData<ChannelCreatedEvent>),
    ChannelUpdated(EventData<ChannelUpdatedEvent>),
    ChannelDeleted(EventData<ChannelDeletedEvent>),
    MessageTypeCreated(EventData<MessageTypeCreatedEvent>),
    MessageTypeUpdated(EventData<MessageTypeUpdatedEvent>),
    MessageTypeDeleted(EventData<MessageTypeDeletedEvent>),
    RouteCreated(EventData<RouteCreatedEvent>),
    RouteUpdated(EventData<RouteUpdatedEvent>),
    RouteDeleted(EventData<RouteDeletedEvent>),
    MessageCreated(EventData<MessageCreatedEvent>),
    TemplateAssignmentCreated(EventData<TemplateAssignmentCreatedEvent>),
    TemplateAssignmentUpdated(EventData<TemplateAssignmentUpdatedEvent>),
    TemplateAssignmentDeleted(EventData<TemplateAssignmentDeletedEvent>),
}

impl Event {
    pub fn event_type(&self) -> &EventType {
        match self {
            Event::BusinessUnitCreated(d) => d.event_type(),
            Event::BusinessUnitUpdated(d) => d.event_type(),
            Event::BusinessUnitDeleted(d) => d.event_type(),
            Event::ChannelCreated(d) => d.event_type(),
            Event::ChannelUpdated(d) => d.event_type(),
            Event::ChannelDeleted(d) => d.event_type(),
            Event::MessageTypeCreated(d) => d.event_type(),
            Event::MessageTypeUpdated(d) => d.event_type(),
            Event::MessageTypeDeleted(d) => d.event_type(),
            Event::RouteCreated(d) => d.event_type(),
            Event::RouteUpdated(d) => d.event_type(),
            Event::RouteDeleted(d) => d.event_type(),
            Event::MessageCreated(d) => d.event_type(),
            Event::TemplateAssignmentCreated(d) => d.event_type(),
            Event::TemplateAssignmentUpdated(d) => d.event_type(),
            Event::TemplateAssignmentDeleted(d) => d.event_type(),
        }
    }

    pub fn entity_id(&self) -> &Id {
        match self {
            Event::BusinessUnitCreated(d) => d.entity_id(),
            Event::BusinessUnitUpdated(d) => d.entity_id(),
            Event::BusinessUnitDeleted(d) => d.entity_id(),
            Event::ChannelCreated(d) => d.entity_id(),
            Event::ChannelUpdated(d) => d.entity_id(),
            Event::ChannelDeleted(d) => d.entity_id(),
            Event::MessageTypeCreated(d) => d.entity_id(),
            Event::MessageTypeUpdated(d) => d.entity_id(),
            Event::MessageTypeDeleted(d) => d.entity_id(),
            Event::RouteCreated(d) => d.entity_id(),
            Event::RouteUpdated(d) => d.entity_id(),
            Event::RouteDeleted(d) => d.entity_id(),
            Event::MessageCreated(d) => d.entity_id(),
            Event::TemplateAssignmentCreated(d) => d.entity_id(),
            Event::TemplateAssignmentUpdated(d) => d.entity_id(),
            Event::TemplateAssignmentDeleted(d) => d.entity_id(),
        }
    }

    pub fn id(&self) -> &Id {
        match self {
            Event::BusinessUnitCreated(d) => d.id(),
            Event::BusinessUnitUpdated(d) => d.id(),
            Event::BusinessUnitDeleted(d) => d.id(),
            Event::ChannelCreated(d) => d.id(),
            Event::ChannelUpdated(d) => d.id(),
            Event::ChannelDeleted(d) => d.id(),
            Event::MessageTypeCreated(d) => d.id(),
            Event::MessageTypeUpdated(d) => d.id(),
            Event::MessageTypeDeleted(d) => d.id(),
            Event::RouteCreated(d) => d.id(),
            Event::RouteUpdated(d) => d.id(),
            Event::RouteDeleted(d) => d.id(),
            Event::MessageCreated(d) => d.id(),
            Event::TemplateAssignmentCreated(d) => d.id(),
            Event::TemplateAssignmentUpdated(d) => d.id(),
            Event::TemplateAssignmentDeleted(d) => d.id(),
        }
    }

    pub fn actor(&self) -> &Actor {
        match self {
            Event::BusinessUnitCreated(d) => d.actor(),
            Event::BusinessUnitUpdated(d) => d.actor(),
            Event::BusinessUnitDeleted(d) => d.actor(),
            Event::ChannelCreated(d) => d.actor(),
            Event::ChannelUpdated(d) => d.actor(),
            Event::ChannelDeleted(d) => d.actor(),
            Event::MessageTypeCreated(d) => d.actor(),
            Event::MessageTypeUpdated(d) => d.actor(),
            Event::MessageTypeDeleted(d) => d.actor(),
            Event::RouteCreated(d) => d.actor(),
            Event::RouteUpdated(d) => d.actor(),
            Event::RouteDeleted(d) => d.actor(),
            Event::MessageCreated(d) => d.actor(),
            Event::TemplateAssignmentCreated(d) => d.actor(),
            Event::TemplateAssignmentUpdated(d) => d.actor(),
            Event::TemplateAssignmentDeleted(d) => d.actor(),
        }
    }

    pub fn created_at(&self) -> &Timestamp {
        match self {
            Event::BusinessUnitCreated(d) => d.created_at(),
            Event::BusinessUnitUpdated(d) => d.created_at(),
            Event::BusinessUnitDeleted(d) => d.created_at(),
            Event::ChannelCreated(d) => d.created_at(),
            Event::ChannelUpdated(d) => d.created_at(),
            Event::ChannelDeleted(d) => d.created_at(),
            Event::MessageTypeCreated(d) => d.created_at(),
            Event::MessageTypeUpdated(d) => d.created_at(),
            Event::MessageTypeDeleted(d) => d.created_at(),
            Event::RouteCreated(d) => d.created_at(),
            Event::RouteUpdated(d) => d.created_at(),
            Event::RouteDeleted(d) => d.created_at(),
            Event::MessageCreated(d) => d.created_at(),
            Event::TemplateAssignmentCreated(d) => d.created_at(),
            Event::TemplateAssignmentUpdated(d) => d.created_at(),
            Event::TemplateAssignmentDeleted(d) => d.created_at(),
        }
    }
}

pub trait ApplicationEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event;
}

#[derive(Debug, Clone, Builder, Serialize, Getters, Deserialize)]
pub struct EventData<P> {
    pub id: Id,
    pub event_type: EventType,
    pub entity_id: Id,
    pub payload: P,
    pub created_at: Timestamp,
    pub actor: Actor,
}

event!(BusinessUnitCreatedEvent, {
    business_unit_id: Id,
    name: Name,
    code: Code,
    vars: Vars,
});

impl ApplicationEvent for BusinessUnitCreatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        Event::BusinessUnitCreated(
            EventData::builder()
                .id(Id::new())
                .created_at(created_at.clone())
                .actor(actor.clone())
                .event_type(EventType::BusinessUnitCreated)
                .entity_id(self.business_unit_id.clone())
                .payload(self)
                .build(),
        )
    }
}

event!(BusinessUnitUpdatedEvent, {
    id: Id,
    name: Name,
    vars: Vars,
});

impl ApplicationEvent for BusinessUnitUpdatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(BusinessUnitDeletedEvent, {
    id: Id,
});

impl ApplicationEvent for BusinessUnitDeletedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(ChannelCreatedEvent, {});

impl ApplicationEvent for ChannelCreatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(ChannelUpdatedEvent, {});

impl ApplicationEvent for ChannelUpdatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(ChannelDeletedEvent, {});

impl ApplicationEvent for ChannelDeletedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(MessageTypeCreatedEvent, {});

impl ApplicationEvent for MessageTypeCreatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(MessageTypeUpdatedEvent, {});

impl ApplicationEvent for MessageTypeUpdatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(MessageTypeDeletedEvent, {});

impl ApplicationEvent for MessageTypeDeletedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(RouteCreatedEvent, {});

impl ApplicationEvent for RouteCreatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(RouteUpdatedEvent, {});

impl ApplicationEvent for RouteUpdatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}
event!(RouteDeletedEvent, {});

impl ApplicationEvent for RouteDeletedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(MessageCreatedEvent, {});

impl ApplicationEvent for MessageCreatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(TemplateAssignmentCreatedEvent, {});

impl ApplicationEvent for TemplateAssignmentCreatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(TemplateAssignmentUpdatedEvent, {});

impl ApplicationEvent for TemplateAssignmentUpdatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}

event!(TemplateAssignmentDeletedEvent, {});

impl ApplicationEvent for TemplateAssignmentDeletedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        todo!()
    }
}
