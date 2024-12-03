use bon::{builder, Builder};
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::{
    events::{Event, EventData, EventType},
    types::{
        actor::{Actor, ActorType},
        entity::Entity,
        id::Id,
        Timestamp,
    },
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
#[builder(on(Id, into), on(Timestamp, into), on(EventType, into))]
pub struct DbEvent {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    event_type: EventType,
    #[setters(skip)]
    entity_id: Id,
    #[setters(skip)]
    payload: Value,

    #[setters(skip)]
    actor_type: ActorType,
    #[setters(skip)]
    actor_id: Option<Id>,

    #[setters(skip)]
    created_at: Timestamp,

    #[builder(skip)]
    consumed_at: Option<Timestamp>,
}

impl Entity for DbEvent {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl DbEvent {
    fn actor(&self) -> Actor {
        match self.actor_type() {
            ActorType::User => Actor::User(self.actor_id().as_ref().unwrap().clone()),
            ActorType::System => Actor::System,
            ActorType::Service => Actor::Service(self.actor_id().as_ref().unwrap().clone()),
        }
    }
}

impl<D: DeserializeOwned> From<&DbEvent> for EventData<D> {
    fn from(value: &DbEvent) -> Self {
        EventData::builder()
            .id(value.id().clone())
            .entity_id(value.entity_id().clone())
            .event_type(value.event_type().clone())
            .created_at(value.created_at().clone())
            .actor(value.actor())
            .payload(serde_json::from_value(value.payload().clone()).unwrap())
            .build()
    }
}

impl TryFrom<&DbEvent> for Event {
    type Error = String;

    fn try_from(value: &DbEvent) -> Result<Self, Self::Error> {
        Ok(match value.event_type() {
            EventType::BusinessUnitCreated => Event::BusinessUnitCreated(value.into()),
            EventType::BusinessUnitUpdated => Event::BusinessUnitUpdated(value.into()),
            EventType::BusinessUnitDeleted => Event::BusinessUnitCreated(value.into()),
            EventType::ChannelCreated => Event::ChannelCreated(value.into()),
            EventType::ChannelUpdated => Event::ChannelUpdated(value.into()),
            EventType::ChannelDeleted => Event::ChannelDeleted(value.into()),
            EventType::MessageTypeCreated => Event::MessageTypeCreated(value.into()),
            EventType::MessageTypeUpdated => Event::MessageTypeUpdated(value.into()),
            EventType::MessageTypeDeleted => Event::MessageTypeDeleted(value.into()),
            EventType::RouteCreated => Event::RouteCreated(value.into()),
            EventType::RouteUpdated => Event::RouteUpdated(value.into()),
            EventType::RouteDeleted => Event::RouteDeleted(value.into()),
            EventType::MessageCreated => Event::MessageCreated(value.into()),
            EventType::TemplateAssignmentCreated => Event::TemplateAssignmentCreated(value.into()),
            EventType::TemplateAssignmentUpdated => Event::TemplateAssignmentUpdated(value.into()),
            EventType::TemplateAssignmentDeleted => Event::TemplateAssignmentDeleted(value.into()),
        })
    }
}

impl TryFrom<Event> for DbEvent {
    type Error = String;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(match value {
            Event::BusinessUnitCreated(d) => DbEvent::from(&d),
            Event::BusinessUnitUpdated(d) => DbEvent::from(&d),
            Event::BusinessUnitDeleted(d) => DbEvent::from(&d),
            Event::ChannelCreated(d) => DbEvent::from(&d),
            Event::ChannelUpdated(d) => DbEvent::from(&d),
            Event::ChannelDeleted(d) => DbEvent::from(&d),
            Event::MessageTypeCreated(d) => DbEvent::from(&d),
            Event::MessageTypeUpdated(d) => DbEvent::from(&d),
            Event::MessageTypeDeleted(d) => DbEvent::from(&d),
            Event::RouteCreated(d) => DbEvent::from(&d),
            Event::RouteUpdated(d) => DbEvent::from(&d),
            Event::RouteDeleted(d) => DbEvent::from(&d),
            Event::MessageCreated(d) => DbEvent::from(&d),
            Event::TemplateAssignmentCreated(d) => DbEvent::from(&d),
            Event::TemplateAssignmentUpdated(d) => DbEvent::from(&d),
            Event::TemplateAssignmentDeleted(d) => DbEvent::from(&d),
        })
    }
}

impl<D: Serialize> From<&EventData<D>> for DbEvent {
    fn from(value: &EventData<D>) -> Self {
        DbEvent::builder()
            .id(value.id().clone())
            .event_type(value.event_type().clone())
            .entity_id(value.entity_id().clone())
            .payload(serde_json::to_value(value.payload()).unwrap())
            .created_at(value.created_at().clone())
            .maybe_actor_id(value.actor().id())
            .actor_type(value.actor().actor_type().clone())
            .build()
    }
}
