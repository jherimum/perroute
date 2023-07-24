use std::collections::HashMap;

use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{dispatch_type::DispatcherType, id::Id};
use sqlx::{types::Json, FromRow, Postgres, QueryBuilder};

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct RouteQuery {
    id: Option<Id>,
    channel_id: Option<Id>,
    message_type_id: Option<Id>,
    shema_id: Option<Id>,
    connection_id: Option<Id>,
    enabled: Option<bool>,
}

impl ModelQueryBuilder<Route> for RouteQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, Postgres> {
        let mut builder = projection.query_builder();
        builder.push(" FROM routes WHERE 1=1");

        if let Some(id) = &self.id {
            builder.push(" and id = ");
            builder.push_bind(id);
        }

        if let Some(channel_id) = &self.channel_id {
            builder.push(" and channel_id = ");
            builder.push_bind(channel_id);
        }

        if let Some(message_type_id) = &self.message_type_id {
            builder.push(" and message_type_id = ");
            builder.push_bind(message_type_id);
        }

        if let Some(shema_id) = &self.shema_id {
            builder.push(" and shema_id = ");
            builder.push_bind(shema_id);
        }

        if let Some(connection_id) = &self.connection_id {
            builder.push(" and connection_id = ");
            builder.push_bind(connection_id);
        }

        if let Some(enabled) = &self.enabled {
            builder.push(" and enabled = ");
            builder.push_bind(enabled);
        }

        builder
    }
}

impl DatabaseModel for Route {}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Route {
    #[setters(skip)]
    id: Id,
    name: String,
    connection_id: Id,
    template_id: Id,
    dispatch_type: DispatcherType,
    dispatcher_properties: Json<HashMap<String, String>>,
    enabled: bool,

    #[setters(skip)]
    channel_id: Id,

    #[setters(skip)]
    message_type_id: Id,

    #[setters(skip)]
    schema_id: Id,
}
