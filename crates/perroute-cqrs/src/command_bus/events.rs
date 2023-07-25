use perroute_commons::types::id::Id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    MessageCreated,
    MessageDistributed,
}

pub struct Event {
    entity_id: Id,
    ty: EventType,
}

impl Event {
    pub fn new(entity_id: Id, ty: EventType) -> Self {
        Self { entity_id, ty }
    }
}
