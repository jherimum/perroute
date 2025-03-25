use bon::{builder, Builder};
use derive_getters::Getters;
use perroute_commons::{
    events::{ApplicationEventData, Event, EventType},
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
use tap::TapFallible;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters)]
pub struct DbEvent {
    #[builder(into)]
    id: Id,

    #[builder(into)]
    event_type: EventType,

    entity_id: Id,

    payload: Value,

    #[builder(into)]
    actor_type: ActorType,

    #[builder(into)]
    actor_id: Option<Id>,

    #[builder(into)]
    created_at: Timestamp,

    #[builder(skip)]
    consumed_at: Option<Timestamp>,

    #[builder(skip)]
    skipped: Option<bool>,
}

impl Entity for DbEvent {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl DbEvent {
    fn actor(&self) -> Actor {
        match self.actor_type() {
            ActorType::User => {
                Actor::User(self.actor_id().as_ref().unwrap().clone())
            }
            ActorType::System => Actor::System,
            ActorType::Service => {
                Actor::Service(self.actor_id().as_ref().unwrap().clone())
            }
        }
    }
}

impl<D: DeserializeOwned> TryFrom<&DbEvent> for ApplicationEventData<D> {
    type Error = serde_json::Error;

    fn try_from(value: &DbEvent) -> Result<Self, Self::Error> {
        Ok(ApplicationEventData::builder()
            .id(value.id().clone())
            .entity_id(value.entity_id().clone())
            .event_type(value.event_type().clone())
            .created_at(value.created_at().clone())
            .actor(value.actor())
            .payload(serde_json::from_value(value.payload().clone()).tap_err(
                |e| log::error!("Failed to deserialize event payload: {e}"),
            )?)
            .build())
    }
}

impl TryFrom<&DbEvent> for Event {
    type Error = serde_json::Error;

    fn try_from(value: &DbEvent) -> Result<Self, Self::Error> {
        Ok(match value.event_type() {
            EventType::BusinessUnitCreated => Event::BusinessUnitCreated(
                ApplicationEventData::try_from(value)?,
            ),
            EventType::BusinessUnitUpdated => Event::BusinessUnitUpdated(
                ApplicationEventData::try_from(value)?,
            ),
            EventType::BusinessUnitDeleted => Event::BusinessUnitCreated(
                ApplicationEventData::try_from(value)?,
            ),
            EventType::ChannelCreated => {
                Event::ChannelCreated(ApplicationEventData::try_from(value)?)
            }
            EventType::ChannelUpdated => {
                Event::ChannelUpdated(ApplicationEventData::try_from(value)?)
            }
            EventType::ChannelDeleted => {
                Event::ChannelDeleted(ApplicationEventData::try_from(value)?)
            }
            EventType::MessageTypeCreated => Event::MessageTypeCreated(
                ApplicationEventData::try_from(value)?,
            ),
            EventType::MessageTypeUpdated => Event::MessageTypeUpdated(
                ApplicationEventData::try_from(value)?,
            ),
            EventType::MessageTypeDeleted => Event::MessageTypeDeleted(
                ApplicationEventData::try_from(value)?,
            ),
            EventType::RouteCreated => {
                Event::RouteCreated(ApplicationEventData::try_from(value)?)
            }
            EventType::RouteUpdated => {
                Event::RouteUpdated(ApplicationEventData::try_from(value)?)
            }
            EventType::RouteDeleted => {
                Event::RouteDeleted(ApplicationEventData::try_from(value)?)
            }
            EventType::MessageCreated => {
                Event::MessageCreated(ApplicationEventData::try_from(value)?)
            }
            EventType::TemplateAssignmentCreated => {
                Event::TemplateAssignmentCreated(
                    ApplicationEventData::try_from(value)?,
                )
            }
            EventType::TemplateAssignmentUpdated => {
                Event::TemplateAssignmentUpdated(
                    ApplicationEventData::try_from(value)?,
                )
            }
            EventType::TemplateAssignmentDeleted => {
                Event::TemplateAssignmentDeleted(
                    ApplicationEventData::try_from(value)?,
                )
            }
        })
    }
}

impl TryFrom<Event> for DbEvent {
    type Error = serde_json::Error;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value {
            Event::BusinessUnitCreated(d) => DbEvent::try_from(&d),
            Event::BusinessUnitUpdated(d) => DbEvent::try_from(&d),
            Event::BusinessUnitDeleted(d) => DbEvent::try_from(&d),
            Event::ChannelCreated(d) => DbEvent::try_from(&d),
            Event::ChannelUpdated(d) => DbEvent::try_from(&d),
            Event::ChannelDeleted(d) => DbEvent::try_from(&d),
            Event::MessageTypeCreated(d) => DbEvent::try_from(&d),
            Event::MessageTypeUpdated(d) => DbEvent::try_from(&d),
            Event::MessageTypeDeleted(d) => DbEvent::try_from(&d),
            Event::RouteCreated(d) => DbEvent::try_from(&d),
            Event::RouteUpdated(d) => DbEvent::try_from(&d),
            Event::RouteDeleted(d) => DbEvent::try_from(&d),
            Event::MessageCreated(d) => DbEvent::try_from(&d),
            Event::TemplateAssignmentCreated(d) => DbEvent::try_from(&d),
            Event::TemplateAssignmentUpdated(d) => DbEvent::try_from(&d),
            Event::TemplateAssignmentDeleted(d) => DbEvent::try_from(&d),
        }
        .tap_err(|e| log::error!("Failed to convert event to DbEvent: {e}"))
    }
}

impl<D: Serialize> TryFrom<&ApplicationEventData<D>> for DbEvent {
    type Error = serde_json::Error;

    fn try_from(value: &ApplicationEventData<D>) -> Result<Self, Self::Error> {
        Ok(DbEvent::builder()
            .id(value.id().clone())
            .event_type(value.event_type().clone())
            .entity_id(value.entity_id().clone())
            .payload(serde_json::to_value(value.payload()).tap_err(|e| {
                log::error!("Failed to serialize event payload: {e}")
            })?)
            .created_at(value.created_at().clone())
            .maybe_actor_id(value.actor().id())
            .actor_type(value.actor().actor_type().clone())
            .build())
    }
}
