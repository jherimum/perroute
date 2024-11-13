use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{
    dispatch_type::DispatchType, id::Id, name::Name, Configuration, ProviderId, Timestamp,
};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]

pub struct Channel {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    business_unit_id: Id,
    name: Name,
    #[setters(skip)]
    provider_id: ProviderId,
    #[setters(skip)]
    dispatch_type: DispatchType,
    configuration: Configuration,
    enabled: bool,
    #[setters(skip)]
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl perroute_commons::types::entity::Entity for Channel {
    fn id(&self) -> &Id {
        &self.id
    }
}
