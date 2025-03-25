use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{
    id::Id, priority::Priority, Configuration, Timestamp,
};
use sqlx::{prelude::FromRow, types::Json};

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Route {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    channel_id: Id,
    #[setters(skip)]
    message_type_id: Id,

    #[getter(skip)]
    configuration: Json<Configuration>,
    priority: Priority,
    enabled: bool,
    #[setters(skip)]
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl AsRef<Configuration> for Route {
    fn as_ref(&self) -> &Configuration {
        &self.configuration
    }
}

impl perroute_commons::types::entity::Entity for Route {
    fn id(&self) -> &Id {
        &self.id
    }
}

#[bon::bon]
impl Route {
    #[builder]
    pub fn new(
        id: Id,
        channel_id: Id,
        message_type_id: Id,
        configuration: Configuration,
        priority: Priority,
        enabled: bool,
        created_at: Timestamp,
        updated_at: Timestamp,
    ) -> Route {
        Route {
            id,
            channel_id,
            message_type_id,
            configuration: Json(configuration),
            priority,
            enabled,
            created_at,
            updated_at,
        }
    }

    pub fn configuration(&self) -> &Configuration {
        self.configuration.as_ref()
    }
}
