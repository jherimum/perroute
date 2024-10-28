use perroute_commons::types::{id::Id, Configuration, DispatchType, Name, ProviderId, Timestamp};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Channel {
    pub id: Id,
    pub business_unit_id: Id,
    pub name: Name,
    pub provider_id: ProviderId,
    pub dispatch_type: DispatchType,
    pub configuration: Configuration,
    pub enabled: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
