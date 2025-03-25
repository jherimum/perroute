use crate::{
    event, impl_sqlx_type,
    types::{
        actor::Actor, code::Code, dispatch_type::DispatchType, entity::Entity,
        id::Id, name::Name, vars::Vars, Configuration, ProviderId, Timestamp,
    },
};
use bon::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Debug, str::FromStr};
use strum::{EnumString, ParseError};

impl_sqlx_type!(EventType as String);

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumString,
    strum::Display,
    Deserialize,
    Serialize,
    Hash,
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
    pub fn parse(str: &str) -> Result<HashSet<Self>, ParseError> {
        str.split(",")
            .filter(|e| !e.is_empty())
            .map(EventType::from_str)
            .collect::<Result<HashSet<_>, _>>()
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Event {
    BusinessUnitCreated(ApplicationEventData<BusinessUnitCreatedEvent>),
    BusinessUnitUpdated(ApplicationEventData<BusinessUnitUpdatedEvent>),
    BusinessUnitDeleted(ApplicationEventData<BusinessUnitDeletedEvent>),
    ChannelCreated(ApplicationEventData<ChannelCreatedEvent>),
    ChannelUpdated(ApplicationEventData<ChannelUpdatedEvent>),
    ChannelDeleted(ApplicationEventData<ChannelDeletedEvent>),
    MessageTypeCreated(ApplicationEventData<MessageTypeCreatedEvent>),
    MessageTypeUpdated(ApplicationEventData<MessageTypeUpdatedEvent>),
    MessageTypeDeleted(ApplicationEventData<MessageTypeDeletedEvent>),
    RouteCreated(ApplicationEventData<RouteCreatedEvent>),
    RouteUpdated(ApplicationEventData<RouteUpdatedEvent>),
    RouteDeleted(ApplicationEventData<RouteDeletedEvent>),
    MessageCreated(ApplicationEventData<MessageCreatedEvent>),
    TemplateAssignmentCreated(
        ApplicationEventData<TemplateAssignmentCreatedEvent>,
    ),
    TemplateAssignmentUpdated(
        ApplicationEventData<TemplateAssignmentUpdatedEvent>,
    ),
    TemplateAssignmentDeleted(
        ApplicationEventData<TemplateAssignmentDeletedEvent>,
    ),
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl Entity for Event {
    fn id(&self) -> &Id {
        self.id()
    }
}

impl Event {
    pub fn id(&self) -> &Id {
        match self {
            Event::BusinessUnitCreated(data) => data.id(),
            Event::BusinessUnitUpdated(data) => data.id(),
            Event::BusinessUnitDeleted(data) => data.id(),
            Event::ChannelCreated(data) => data.id(),
            Event::ChannelUpdated(data) => data.id(),
            Event::ChannelDeleted(data) => data.id(),
            Event::MessageTypeCreated(data) => data.id(),
            Event::MessageTypeUpdated(data) => data.id(),
            Event::MessageTypeDeleted(data) => data.id(),
            Event::RouteCreated(data) => data.id(),
            Event::RouteUpdated(data) => data.id(),
            Event::RouteDeleted(data) => data.id(),
            Event::MessageCreated(data) => data.id(),
            Event::TemplateAssignmentCreated(data) => data.id(),
            Event::TemplateAssignmentUpdated(data) => data.id(),
            Event::TemplateAssignmentDeleted(data) => data.id(),
        }
    }

    pub fn event_type(&self) -> &EventType {
        match self {
            Event::BusinessUnitCreated(data) => data.event_type(),
            Event::BusinessUnitUpdated(data) => data.event_type(),
            Event::BusinessUnitDeleted(data) => data.event_type(),
            Event::ChannelCreated(data) => data.event_type(),
            Event::ChannelUpdated(data) => data.event_type(),
            Event::ChannelDeleted(data) => data.event_type(),
            Event::MessageTypeCreated(data) => data.event_type(),
            Event::MessageTypeUpdated(data) => data.event_type(),
            Event::MessageTypeDeleted(data) => data.event_type(),
            Event::RouteCreated(data) => data.event_type(),
            Event::RouteUpdated(data) => data.event_type(),
            Event::RouteDeleted(data) => data.event_type(),
            Event::MessageCreated(data) => data.event_type(),
            Event::TemplateAssignmentCreated(data) => data.event_type(),
            Event::TemplateAssignmentUpdated(data) => data.event_type(),
            Event::TemplateAssignmentDeleted(data) => data.event_type(),
        }
    }

    pub fn actor(&self) -> &Actor {
        match self {
            Event::BusinessUnitCreated(data) => data.actor(),
            Event::BusinessUnitUpdated(data) => data.actor(),
            Event::BusinessUnitDeleted(data) => data.actor(),
            Event::ChannelCreated(data) => data.actor(),
            Event::ChannelUpdated(data) => data.actor(),
            Event::ChannelDeleted(data) => data.actor(),
            Event::MessageTypeCreated(data) => data.actor(),
            Event::MessageTypeUpdated(data) => data.actor(),
            Event::MessageTypeDeleted(data) => data.actor(),
            Event::RouteCreated(data) => data.actor(),
            Event::RouteUpdated(data) => data.actor(),
            Event::RouteDeleted(data) => data.actor(),
            Event::MessageCreated(data) => data.actor(),
            Event::TemplateAssignmentCreated(data) => data.actor(),
            Event::TemplateAssignmentUpdated(data) => data.actor(),
            Event::TemplateAssignmentDeleted(data) => data.actor(),
        }
    }

    pub fn entity_id(&self) -> &Id {
        match self {
            Event::BusinessUnitCreated(data) => data.entity_id(),
            Event::BusinessUnitUpdated(data) => data.entity_id(),
            Event::BusinessUnitDeleted(data) => data.entity_id(),
            Event::ChannelCreated(data) => data.entity_id(),
            Event::ChannelUpdated(data) => data.entity_id(),
            Event::ChannelDeleted(data) => data.entity_id(),
            Event::MessageTypeCreated(data) => data.entity_id(),
            Event::MessageTypeUpdated(data) => data.entity_id(),
            Event::MessageTypeDeleted(data) => data.entity_id(),
            Event::RouteCreated(data) => data.entity_id(),
            Event::RouteUpdated(data) => data.entity_id(),
            Event::RouteDeleted(data) => data.entity_id(),
            Event::MessageCreated(data) => data.entity_id(),
            Event::TemplateAssignmentCreated(data) => data.entity_id(),
            Event::TemplateAssignmentUpdated(data) => data.entity_id(),
            Event::TemplateAssignmentDeleted(data) => data.entity_id(),
        }
    }

    pub fn created_at(&self) -> &Timestamp {
        match self {
            Event::BusinessUnitCreated(data) => data.created_at(),
            Event::BusinessUnitUpdated(data) => data.created_at(),
            Event::BusinessUnitDeleted(data) => data.created_at(),
            Event::ChannelCreated(data) => data.created_at(),
            Event::ChannelUpdated(data) => data.created_at(),
            Event::ChannelDeleted(data) => data.created_at(),
            Event::MessageTypeCreated(data) => data.created_at(),
            Event::MessageTypeUpdated(data) => data.created_at(),
            Event::MessageTypeDeleted(data) => data.created_at(),
            Event::RouteCreated(data) => data.created_at(),
            Event::RouteUpdated(data) => data.created_at(),
            Event::RouteDeleted(data) => data.created_at(),
            Event::MessageCreated(data) => data.created_at(),
            Event::TemplateAssignmentCreated(data) => data.created_at(),
            Event::TemplateAssignmentUpdated(data) => data.created_at(),
            Event::TemplateAssignmentDeleted(data) => data.created_at(),
        }
    }
}

pub trait ApplicationEvent: Debug {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event;
}

#[derive(Debug, Clone, Builder, Serialize, Getters, Deserialize)]
pub struct ApplicationEventData<P> {
    pub id: Id,
    pub event_type: EventType,
    pub entity_id: Id,
    pub payload: P,
    pub created_at: Timestamp,
    pub actor: Actor,
}

impl From<ApplicationEventData<BusinessUnitCreatedEvent>> for Event {
    fn from(data: ApplicationEventData<BusinessUnitCreatedEvent>) -> Self {
        Event::BusinessUnitCreated(data)
    }
}

event!(BusinessUnitCreatedEvent, {
    business_unit_id: Id,
    name: Name,
    code: Code,
    vars: Vars,
});

impl ApplicationEvent for BusinessUnitCreatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        ApplicationEventData::builder()
            .id(Id::new())
            .created_at(created_at.clone())
            .actor(actor.clone())
            .event_type(EventType::BusinessUnitCreated)
            .entity_id(self.business_unit_id.clone())
            .payload(self)
            .build()
            .into()
    }
}

event!(BusinessUnitUpdatedEvent, {
    business_unit_id: Id,
    name: Name,
    vars: Vars,
});

impl ApplicationEvent for BusinessUnitUpdatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        Event::BusinessUnitUpdated(
            ApplicationEventData::builder()
                .id(Id::new())
                .created_at(created_at.clone())
                .actor(actor.clone())
                .event_type(EventType::BusinessUnitUpdated)
                .entity_id(self.business_unit_id.clone())
                .payload(self)
                .build(),
        )
    }
}

event!(BusinessUnitDeletedEvent, {
    business_unit_id: Id,
});

impl ApplicationEvent for BusinessUnitDeletedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        Event::BusinessUnitDeleted(
            ApplicationEventData::builder()
                .id(Id::new())
                .created_at(created_at.clone())
                .actor(actor.clone())
                .event_type(EventType::BusinessUnitDeleted)
                .entity_id(self.business_unit_id.clone())
                .payload(self)
                .build(),
        )
    }
}

event!(ChannelCreatedEvent, {
    id: Id,
    business_unit_id: Id,
    name: Name,
    provider_id: ProviderId,
    dispatch_type: DispatchType,
    configuration: Configuration,
    enabled: bool,
});

impl ApplicationEvent for ChannelCreatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        Event::ChannelCreated(
            ApplicationEventData::builder()
                .id(Id::new())
                .created_at(created_at.clone())
                .actor(actor.clone())
                .event_type(EventType::ChannelCreated)
                .entity_id(self.id.clone())
                .payload(self)
                .build(),
        )
    }
}

event!(ChannelUpdatedEvent, {
    id: Id,
    name: Option<Name>,
    configuration: Option<Configuration>,
    enabled: Option<bool>
});

impl ApplicationEvent for ChannelUpdatedEvent {
    fn to_event(self, actor: &Actor, created_at: &Timestamp) -> Event {
        Event::ChannelUpdated(
            ApplicationEventData::builder()
                .id(Id::new())
                .actor(actor.clone())
                .created_at(created_at.clone())
                .entity_id(self.id.clone())
                .event_type(EventType::ChannelUpdated)
                .payload(self)
                .build(),
        )
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
