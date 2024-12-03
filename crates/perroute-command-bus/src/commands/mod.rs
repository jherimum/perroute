use perroute_commons::{
    events::{Event, EventData, EventType},
    types::{actor::Actor, id::Id, Timestamp},
};
use serde::Serialize;

pub mod business_unit;
pub mod channel;
pub mod message;
pub mod message_type;
pub mod route;
pub mod template_assignment;

pub struct CommandWrapper<'c, C> {
    command: &'c C,
    created_at: &'c Timestamp,
    actor: &'c Actor,
}

impl<'c, C: Command> CommandWrapper<'c, C> {
    pub fn new(command: &'c C, created_at: &'c Timestamp, actor: &'c Actor) -> Self {
        Self {
            command,
            created_at,
            actor,
        }
    }
}

impl<'c, C: Command + Serialize> TryFrom<&CommandWrapper<'c, C>> for Event {
    type Error = String;

    fn try_from(value: &CommandWrapper<'c, C>) -> Result<Self, Self::Error> {
        let event_data = EventData::builder()
            .id(Id::new())
            .entity_id(value.command.entity_id().clone())
            .actor(value.actor.clone())
            .payload(serde_json::to_value(&value.command).unwrap())
            .created_at(value.created_at.clone())
            .build();

        Ok(match value.command.event_type() {
            EventType::BusinessUnitCreated => Event::BusinessUnitCreated(event_data),
            EventType::BusinessUnitDeleted => Event::BusinessUnitDeleted(event_data),
            EventType::BusinessUnitUpdated => Event::BusinessUnitUpdated(event_data),
            EventType::ChannelCreated => Event::ChannelCreated(event_data),
            EventType::ChannelDeleted => Event::ChannelDeleted(event_data),
            EventType::ChannelUpdated => Event::ChannelUpdated(event_data),
            EventType::MessageTypeCreated => Event::MessageTypeCreated(event_data),
            EventType::MessageTypeDeleted => Event::MessageTypeDeleted(event_data),
            EventType::MessageTypeUpdated => Event::MessageTypeUpdated(event_data),
            EventType::RouteCreated => Event::RouteCreated(event_data),
            EventType::RouteDeleted => Event::RouteDeleted(event_data),
            EventType::RouteUpdated => Event::RouteUpdated(event_data),
            EventType::MessageCreated => Event::MessageCreated(event_data),
        })
    }
}

impl<C: Command> CommandWrapper<'_, C> {
    pub fn inner(&self) -> &C {
        self.command
    }

    pub fn created_at(&self) -> &Timestamp {
        &self.created_at
    }

    pub fn actor(&self) -> &Actor {
        &self.actor
    }
}

pub trait Command {
    fn entity_id(&self) -> &Id;
    fn event_type(&self) -> EventType;
}
