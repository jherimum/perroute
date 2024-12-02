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
use serde_json::Value;
use sqlx::prelude::{FromRow, Type};

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

impl TryFrom<Event> for DbEvent {
    type Error = String;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        let event_data = AsRef::as_ref(&value);
        Ok(DbEvent::builder()
            .maybe_actor_id(event_data.actor().id().cloned())
            .actor_type(event_data.actor().actor_type())
            .entity_id(event_data.entity_id())
            .id(event_data.id())
            .created_at(event_data.created_at())
            .payload(event_data.payload().clone())
            .event_type(event_data.event_type())
            .build())
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct DbEventPayload(Value);

impl From<Value> for DbEventPayload {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl From<&Value> for DbEventPayload {
    fn from(value: &Value) -> Self {
        DbEventPayload::from(value.clone())
    }
}

impl AsRef<Value> for DbEventPayload {
    fn as_ref(&self) -> &Value {
        &self.0
    }
}

impl DbEventPayload {
    pub fn deserialize<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.0.clone())
    }

    pub fn serialize(value: &impl serde::Serialize) -> Result<Self, serde_json::Error> {
        Ok(Self(serde_json::to_value(value)?))
    }
}

impl TryFrom<&DbEvent> for Event {
    type Error = String;

    fn try_from(value: &DbEvent) -> Result<Self, Self::Error> {
        let data = EventData::builder()
            .actor(value.actor())
            .created_at(value.created_at().clone())
            .entity_id(value.entity_id().clone())
            .event_type(value.event_type().clone())
            .payload(value.payload().clone())
            .id(value.id().clone())
            .build();

        Ok(match value.event_type {
            EventType::BusinessUnitCreated => Event::BusinessUnitCreated(data),
            EventType::BusinessUnitUpdated => Event::BusinessUnitUpdated(data),
            EventType::BusinessUnitDeleted => Event::BusinessUnitDeleted(data),
            EventType::ChannelCreated => Event::ChannelCreated(data),
            EventType::ChannelUpdated => Event::ChannelUpdated(data),
            EventType::ChannelDeleted => Event::ChannelDeleted(data),
            EventType::MessageTypeCreated => Event::MessageTypeCreated(data),
            EventType::MessageTypeUpdated => Event::MessageTypeUpdated(data),
            EventType::MessageTypeDeleted => Event::MessageTypeDeleted(data),
            EventType::RouteCreated => Event::RouteCreated(data),
            EventType::RouteUpdated => Event::RouteUpdated(data),
            EventType::RouteDeleted => Event::RouteDeleted(data),
        })
    }
}
