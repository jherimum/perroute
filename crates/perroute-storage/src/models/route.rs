use derive_getters::Getters;
use perroute_commons::types::{id::Id, priority::Priority, Configuration, Timestamp};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters)]
pub struct Route {
    id: Id,
    channel_id: Id,
    message_type_id: Id,
    configuration: Configuration,
    priority: Priority,
    enabled: bool,
    created_at: Timestamp,
    updated_at: Timestamp,
}
