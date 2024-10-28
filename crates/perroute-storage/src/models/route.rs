use perroute_commons::types::{id::Id, Configuration, Priority, Timestamp};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Route {
    pub id: Id,
    pub channel_id: Id,
    pub message_type_id: Id,
    pub configuration: Configuration,
    pub priority: Priority,
    pub enabled: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
