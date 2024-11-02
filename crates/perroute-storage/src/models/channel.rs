use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{
    id::Id, name::Name, Configuration, DispatchType, ProviderId, Timestamp,
};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]

pub struct Channel {
    #[setters(skip)]
    pub id: Id,
    #[setters(skip)]
    pub business_unit_id: Id,
    pub name: Name,
    #[setters(skip)]
    pub provider_id: ProviderId,
    #[setters(skip)]
    pub dispatch_type: DispatchType,
    pub configuration: Configuration,
    pub enabled: bool,
    #[setters(skip)]
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
