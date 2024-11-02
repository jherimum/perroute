use bon::Builder;
use perroute_commons::types::{
    id::Id, name::Name, Configuration, DispatchType, ProviderId, Timestamp,
};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder)]
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
