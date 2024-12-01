use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::{
    events::EventType,
    new_events::{EventData, NewEvent},
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
pub struct NewDbEvent {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    event_type: EventType,
    #[setters(skip)]
    entity_id: Id,
    #[setters(skip)]
    payload: DbEventPayload,

    #[setters(skip)]
    actor_type: ActorType,
    #[setters(skip)]
    actor_id: Option<Id>,

    #[setters(skip)]
    created_at: Timestamp,
    consumed_at: Option<Timestamp>,
}

impl Entity for NewDbEvent {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl NewDbEvent {
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

impl DbEventPayload {
    pub fn deserialize<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.0.clone())
    }
}

impl TryFrom<&NewDbEvent> for NewEvent {
    type Error = String;

    fn try_from(value: &NewDbEvent) -> Result<Self, Self::Error> {
        let builder = EventData::builder()
            .actor(value.actor())
            .created_at(value.created_at().clone())
            .entity_id(value.entity_id().clone())
            .event_type(value.event_type().clone())
            .id(value.id().clone());

        Ok(match value.event_type {
            EventType::BusinessUnitCreated => {
                NewEvent::BusinessUnitCreated(builder.payload(()).build())
            }
            EventType::BusinessUnitUpdated => {
                NewEvent::BusinessUnitUpdated(builder.payload(()).build())
            }
            EventType::BusinessUnitDeleted => {
                NewEvent::BusinessUnitDeleted(builder.payload(()).build())
            }
            EventType::ChannelCreated => NewEvent::ChannelCreated(builder.payload(()).build()),
            EventType::ChannelUpdated => NewEvent::ChannelUpdated(builder.payload(()).build()),
            EventType::ChannelDeleted => NewEvent::ChannelDeleted(builder.payload(()).build()),
            EventType::MessageTypeCreated => {
                NewEvent::MessageTypeCreated(builder.payload(()).build())
            }
            EventType::MessageTypeUpdated => {
                NewEvent::MessageTypeUpdated(builder.payload(()).build())
            }
            EventType::MessageTypeDeleted => {
                NewEvent::MessageTypeDeleted(builder.payload(()).build())
            }
            EventType::RouteCreated => NewEvent::RouteCreated(builder.payload(()).build()),
            EventType::RouteUpdated => NewEvent::RouteUpdated(builder.payload(()).build()),
            EventType::RouteDeleted => NewEvent::RouteDeleted(builder.payload(()).build()),
        })
    }
}
