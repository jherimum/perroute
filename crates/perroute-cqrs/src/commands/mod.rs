use perroute_commons::types::id::Id;
use perroute_storage::models::event::Event as DbEvent;

pub mod channel;
pub mod connection;
pub mod plugins;
pub mod token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    ChannelEvent(ChannelEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelEvent {
    Created(Id),
    Updated(Id),
    Deleted(Id),
}

impl Into<DbEvent> for Event {
    fn into(self) -> DbEvent {
        todo!()
    }
}
