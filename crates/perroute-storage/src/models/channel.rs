use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{
    dispatch_type::DispatchType, id::Id, name::Name, Configuration, ProviderId,
    Timestamp,
};
use sqlx::{prelude::FromRow, types::Json};

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
    configuration: Json<Configuration>,
    enabled: bool,
    #[setters(skip)]
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl AsRef<Configuration> for Channel {
    fn as_ref(&self) -> &Configuration {
        &self.configuration
    }
}

impl perroute_commons::types::entity::Entity for Channel {
    fn id(&self) -> &Id {
        &self.id
    }
}
