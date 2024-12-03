use perroute_commons::{
    events::{Event, EventType},
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
        todo!()
    }
}

impl<C: Command> CommandWrapper<'_, C> {
    pub fn inner(&self) -> &C {
        self.command
    }

    pub fn created_at(&self) -> &Timestamp {
        self.created_at
    }

    pub fn actor(&self) -> &Actor {
        self.actor
    }
}

pub trait Command {
    fn entity_id(&self) -> &Id;
    fn event_type(&self) -> EventType;
}
